import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import { SUPABASE_CONNECTION_STRING } from "$env/static/private";


const index = postgres(SUPABASE_CONNECTION_STRING);
export default drizzle(index);
