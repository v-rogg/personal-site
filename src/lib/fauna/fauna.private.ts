import { FAUNA_SECRET } from "$env/static/private";
import { Client } from "fauna";

export const getPrivateFaunaClient = () => {
	return new Client({ secret: FAUNA_SECRET });
}