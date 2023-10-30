import { sql } from "drizzle-orm";
import { text, sqliteTable, integer } from "drizzle-orm/sqlite-core";

export const signatures = sqliteTable('signatures', {
	id: text("id").notNull().primaryKey(),
	name: text("name").notNull(),
	ts_created: text("ts_created").notNull().default(sql`current_timestamp`),
	ts_modified: text("ts_modified"),
	approved: integer("approved", { mode: 'boolean' }),
	signature: text("signature", { mode: "json" }).notNull(),
})