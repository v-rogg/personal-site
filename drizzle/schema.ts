import { pgTable, pgEnum, uuid, text, timestamp, boolean, json } from "drizzle-orm/pg-core"

import { sql } from "drizzle-orm"
export const keyStatus = pgEnum("key_status", ['expired', 'invalid', 'valid', 'default'])
export const keyType = pgEnum("key_type", ['stream_xchacha20', 'secretstream', 'secretbox', 'kdf', 'generichash', 'shorthash', 'auth', 'hmacsha256', 'hmacsha512', 'aead-det', 'aead-ietf'])
export const factorType = pgEnum("factor_type", ['webauthn', 'totp'])
export const factorStatus = pgEnum("factor_status", ['verified', 'unverified'])
export const aalLevel = pgEnum("aal_level", ['aal3', 'aal2', 'aal1'])
export const codeChallengeMethod = pgEnum("code_challenge_method", ['plain', 's256'])


export const signatures = pgTable("signatures", {
	id: uuid("id").default(sql`uuid_generate_v4()`).primaryKey().notNull(),
	name: text("name").notNull(),
	ts_created: timestamp("ts_created", { mode: 'string' }).defaultNow().notNull(),
	ts_modified: timestamp("ts_modified", { mode: 'string' }),
	approved: boolean("approved"),
	signature: json("signature").notNull(),
});