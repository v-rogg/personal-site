import { Pool } from "pg";
import { drizzle as drizzlePG, type NodePgDatabase } from "drizzle-orm/node-postgres";
import { drizzle, type DrizzleD1Database } from "drizzle-orm/d1";
import { eq, isNull } from "drizzle-orm";
import { createCors, error, json, Router, withParams } from "itty-router";
import { Signature } from "$drizzle/types";
import { signatures } from "$drizzle/schema.sqlite";
import { signatures as signaturesPG } from "$drizzle/schema.pg";

export interface Env {
	DB_CONN_STRING: string;
	SECURE_PSK: string;
	DB: DrizzleD1Database;
	PG: NodePgDatabase;
	POOL: Pool;
	SECURE: boolean;
	D1DB: D1Database
}

const { preflight, corsify } = createCors({
	methods: ["GET", "POST", "PUT", "DELETE"]
});

const router = Router();

router
	.all("*", preflight)
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

		// ctx.waitUntil(env.POOL.end());
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
			// ctx.waitUntil(env.POOL.end());
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
			try {
				const result = await env.DB.insert(signatures)
					.values({ id: signature.id, name: signature.name, signature: signature.signature })
					.returning({
						id: signatures.id,
						name: signatures.name,
						signature: signatures.signature,
						ts_created: signatures.ts_created
					}).then(res => res[0]);
				return new Response(JSON.stringify(result), { status: 201 });
			} catch (e) {
				console.log(e);
			}
			// ctx.waitUntil(env.POOL.end());
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
				// ctx.waitUntil(env.POOL.end());
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
				// ctx.waitUntil(env.POOL.end());
				return new Response( `Deleted ${id}`)
			}
		}
	})

	.get("/migrate", async (req, env: Env, ctx) => {
		if (req.SECURE) {
			const result = await env.PG.select().from(signaturesPG);

			result.forEach(async (entry) => {
				await env.DB.insert(signatures).values({
					id: entry.id,
					name: entry.name,
					signature: entry.signature,
					ts_created: entry.ts_created,
					ts_modified: entry.ts_modified,
					approved: entry.approved
				});
			})


			ctx.waitUntil(env.POOL.end());
			return new Response("Migrated Data")
		}
	})

	.all("*", () => {
		return error(405)
	})

export default {
	async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
		env.POOL = new Pool({ connectionString: env.DB_CONN_STRING });
		env.DB = drizzle(env.D1DB);
		env.PG = drizzlePG(env.POOL);
		return router.handle(request, env, ctx).then(json).catch(error).then(corsify);
	}
};
