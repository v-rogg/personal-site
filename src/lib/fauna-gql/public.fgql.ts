import { GraphQLClient } from "graphql-request";
import { PUBLIC_FAUNA_GQL_ENDPOINT, PUBLIC_FAUNA_SECRET } from "$env/static/public";

export const publicFGQLClient = new GraphQLClient(PUBLIC_FAUNA_GQL_ENDPOINT, {
	headers: {
		Authorization: `Bearer ${PUBLIC_FAUNA_SECRET}`
	}
});
