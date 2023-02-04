import { PUBLIC_FAUNA_GQL_ENDPOINT, PUBLIC_FAUNA_SECRET } from "$env/static/public";
import { GraphQLClient } from "graphql-request";

export const client = new GraphQLClient(PUBLIC_FAUNA_GQL_ENDPOINT, {
	headers: {
		Authorization: `Bearer ${PUBLIC_FAUNA_SECRET}`
	}
});
