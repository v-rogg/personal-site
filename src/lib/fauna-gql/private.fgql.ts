import { GraphQLClient } from "graphql-request";
import { PUBLIC_FAUNA_GQL_ENDPOINT } from "$env/static/public";
import { FAUNA_SECRET } from "$env/static/private";

export const privateFGQLClient = new GraphQLClient(PUBLIC_FAUNA_GQL_ENDPOINT, {
	headers: {
		Authorization: `Bearer ${FAUNA_SECRET}`
	}
});
