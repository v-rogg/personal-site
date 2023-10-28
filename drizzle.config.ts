import type { Config } from "drizzle-kit";
import * as dotenv from "dotenv";
dotenv.config();

if (!process.env.PUBLIC_SUPABASE_CONNECTION_STRING) {
	throw new Error("DB CONNECTION STRING is missing")
}

export default {
	schema: "./drizzle/schema.ts",
	driver: "pg",
	dbCredentials: {
		connectionString: process.env.PUBLIC_SUPABASE_CONNECTION_STRING
	}
} satisfies Config;