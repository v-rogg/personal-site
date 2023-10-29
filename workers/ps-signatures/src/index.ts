import { Pool } from "pg";
import { drizzle, type NodePgDatabase } from "drizzle-orm/node-postgres";
import { eq, isNull } from "drizzle-orm";
import { error, json, Router, withParams } from "itty-router";
import { Signature } from "$drizzle/types";
import { signatures } from "$drizzle/schema";

export interface Env {
	DB_CONN_STRING: string;
	SECURE_PSK: string;
	DB: NodePgDatabase;
	POOL: Pool;
	SECURE: boolean;
}

const router = Router();

router
	.all("*", withParams)

	.all("*", (req, env) => {
		const PRESHARED_AUTH_HEADER_KEY = "Authorization";
		const PRESHARED_AUTH_HEADER_VALUE = env.SECURE_PSK;
		const psk = req.headers.get(PRESHARED_AUTH_HEADER_KEY) || "";

		req.SECURE = `Bearer ${PRESHARED_AUTH_HEADER_VALUE}` === psk;
	})

	.get("/", async (req, env, ctx) => {
		let result;
		if (req.SECURE) {
			switch (req.query.filter) {
				case "unreviewed":
					result = await env.DB.select({ id: signatures.id }).from(signatures).where(isNull(signatures.approved));
					break;
				default:
					result = await env.DB.select({ id: signatures.id }).from(signatures);
					break;
			}
		} else {
			result = await env.DB.select({ id: signatures.id }).from(signatures).where(eq(signatures.approved, true));
		}

		ctx.waitUntil(env.POOL.end());
		return result;
	})

	.get("/:id", async (req, env, ctx) => {
		const id = <string>req.params.id;
		try {
			const result = await env.DB.select()
				.from(signatures)
				.where(eq(signatures.id, id))
				.limit(1)
				.then((res: Signature[]) => res[0]);
			ctx.waitUntil(env.POOL.end());
			return result;
		} catch (e) {
			return error(400);
		}
	})

	.post("/", async (req, env: Env, ctx) => {
		if (req.SECURE) {
			const signature = <Signature>await req.json();
			if (!signature.name) {
				return error(400);
			}
			const result = await env.DB.insert(signatures)
				.values({ name: signature.name, signature: signature.signature })
				.returning({
					id: signatures.id,
					name: signatures.name,
					signature: signatures.signature,
					ts_created: signatures.ts_created
				}).then(res => res[0]);
			ctx.waitUntil(env.POOL.end());
			return new Response(JSON.stringify(result), { status: 201 });
		} else {
			return error(401);
		}
	})

	.put("/:id", async (req, env: Env, ctx) => {
		if (req.SECURE) {
			const id = <string>req.params.id;
			const data = <Partial<Signature>>await req.json();

			if (data.approved !== null && id !== null) {
				const result = await env.DB.update(signatures)
					.set(data)
					.where(eq(signatures.id, id))
					.returning({
						id: signatures.id,
						name: signatures.name,
						signature: signatures.signature,
						ts_created: signatures.ts_created,
						approved: signatures.approved
					}).then(res => res[0]);
				ctx.waitUntil(env.POOL.end());
				return result;
			} else {
				return error(400);
			}
		}
	})

	.delete("/:id", async (req, env: Env, ctx) => {
		if (req.SECURE) {
			const id = <string>req.params.id;

			if (id !== null) {
				await env.DB.delete(signatures).where(eq(signatures.id, id));
				ctx.waitUntil(env.POOL.end());
				return new Response( `Deleted ${id}`)
			}
		}
	})

	.all("*", () => {
		return error(405)
	})

export default {
	async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
		env.POOL = new Pool({ connectionString: env.DB_CONN_STRING });
		env.DB = drizzle(env.POOL);
		return router.handle(request, env, ctx).then(json).catch(error);
	}
};
