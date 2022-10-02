import { GraphQLClient } from "graphql-request";
import { getSdk } from "../generated/graphql";

const endpoint = "http://localhost:8000";

const graphQLClient = new GraphQLClient(endpoint);
export const sdk = getSdk(graphQLClient);
