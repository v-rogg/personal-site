import type { Config } from "drizzle-kit";
import * as dotenv from "dotenv";
dotenv.config();

if (!process.env.DB_CONN_STRING) {
	throw new Error("DB_CONN_STRING is missing")
}

export default {
	schema: "./drizzle/schema.ts",
	driver: "pg",
	dbCredentials: {
		connectionString: process.env.DB_CONN_STRING
	}
} satisfies Config;