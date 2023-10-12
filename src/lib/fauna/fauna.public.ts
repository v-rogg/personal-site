import { PUBLIC_FAUNA_SECRET } from "$env/static/public";
import { Client } from "fauna";

export const getPublicFaunaClient = () => {
	return new Client({ secret: PUBLIC_FAUNA_SECRET });
}