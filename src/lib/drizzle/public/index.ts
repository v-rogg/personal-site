import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import { PUBLIC_SUPABASE_CONNECTION_STRING } from "$env/static/public";


const index = postgres(PUBLIC_SUPABASE_CONNECTION_STRING);
export default drizzle(index);

// export const getPublicDrizzleClient = () => {
	// const client = postgres(PUBLIC_SUPABASE_CONNECTION_STRING)
	// return drizzle(client)
// }