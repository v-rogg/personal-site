import { PUBLIC_FAUNA_GQL_ENDPOINT } from "$env/static/public";
import { FAUNA_SECRET } from "$env/static/private";
import { GraphQLClient } from "graphql-request";

export const client = new GraphQLClient(PUBLIC_FAUNA_GQL_ENDPOINT, {
	fetch,
	headers: {
		Authorization: `Bearer ${FAUNA_SECRET}`
	}
});
