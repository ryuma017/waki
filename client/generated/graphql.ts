import { GraphQLClient } from "graphql-request";
import * as Dom from "graphql-request/dist/types.dom";
import gql from "graphql-tag";
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]?: Maybe<T[SubKey]>;
};
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]: Maybe<T[SubKey]>;
};
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type CreatePageInput = {
  source: Scalars["String"];
  title: Scalars["String"];
};

export type MutationRoot = {
  __typename?: "MutationRoot";
  createPage: Page;
  updatePage?: Maybe<Page>;
};

export type MutationRootCreatePageArgs = {
  input: CreatePageInput;
};

export type MutationRootUpdatePageArgs = {
  input: UpdatePageInput;
};

export type Page = {
  __typename?: "Page";
  bodyHtml: Scalars["String"];
  id: Scalars["Int"];
  title: Scalars["String"];
};

export type QueryRoot = {
  __typename?: "QueryRoot";
  msg: Scalars["String"];
  pageById?: Maybe<Page>;
  pageByTitle?: Maybe<Page>;
};

export type QueryRootPageByIdArgs = {
  id: Scalars["Int"];
};

export type QueryRootPageByTitleArgs = {
  title: Scalars["String"];
};

export type UpdatePageInput = {
  id: Scalars["Int"];
  source?: InputMaybe<Scalars["String"]>;
  title?: InputMaybe<Scalars["String"]>;
};

export type CreatePageMutationVariables = Exact<{ [key: string]: never }>;

export type CreatePageMutation = {
  __typename?: "MutationRoot";
  createPage: { __typename?: "Page"; id: number };
};

export type GetPageByIdQueryVariables = Exact<{
  id: Scalars["Int"];
}>;

export type GetPageByIdQuery = {
  __typename?: "QueryRoot";
  pageById?: {
    __typename?: "Page";
    id: number;
    title: string;
    bodyHtml: string;
  } | null;
};

export type GetPageByTitleQueryVariables = Exact<{
  title: Scalars["String"];
}>;

export type GetPageByTitleQuery = {
  __typename?: "QueryRoot";
  pageByTitle?: {
    __typename?: "Page";
    id: number;
    title: string;
    bodyHtml: string;
  } | null;
};

export const CreatePageDocument = gql`
  mutation createPage {
    createPage(input: { title: "Hello, world!", source: "# Hello, world!" }) {
      id
    }
  }
`;
export const GetPageByIdDocument = gql`
  query getPageById($id: Int!) {
    pageById(id: $id) {
      id
      title
      bodyHtml
    }
  }
`;
export const GetPageByTitleDocument = gql`
  query getPageByTitle($title: String!) {
    pageByTitle(title: $title) {
      id
      title
      bodyHtml
    }
  }
`;

export type SdkFunctionWrapper = <T>(
  action: (requestHeaders?: Record<string, string>) => Promise<T>,
  operationName: string,
  operationType?: string
) => Promise<T>;

const defaultWrapper: SdkFunctionWrapper = (
  action,
  _operationName,
  _operationType
) => action();

export function getSdk(
  client: GraphQLClient,
  withWrapper: SdkFunctionWrapper = defaultWrapper
) {
  return {
    createPage(
      variables?: CreatePageMutationVariables,
      requestHeaders?: Dom.RequestInit["headers"]
    ): Promise<CreatePageMutation> {
      return withWrapper(
        (wrappedRequestHeaders) =>
          client.request<CreatePageMutation>(CreatePageDocument, variables, {
            ...requestHeaders,
            ...wrappedRequestHeaders,
          }),
        "createPage",
        "mutation"
      );
    },
    getPageById(
      variables: GetPageByIdQueryVariables,
      requestHeaders?: Dom.RequestInit["headers"]
    ): Promise<GetPageByIdQuery> {
      return withWrapper(
        (wrappedRequestHeaders) =>
          client.request<GetPageByIdQuery>(GetPageByIdDocument, variables, {
            ...requestHeaders,
            ...wrappedRequestHeaders,
          }),
        "getPageById",
        "query"
      );
    },
    getPageByTitle(
      variables: GetPageByTitleQueryVariables,
      requestHeaders?: Dom.RequestInit["headers"]
    ): Promise<GetPageByTitleQuery> {
      return withWrapper(
        (wrappedRequestHeaders) =>
          client.request<GetPageByTitleQuery>(
            GetPageByTitleDocument,
            variables,
            { ...requestHeaders, ...wrappedRequestHeaders }
          ),
        "getPageByTitle",
        "query"
      );
    },
  };
}
export type Sdk = ReturnType<typeof getSdk>;
