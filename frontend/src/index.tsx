import React from "react";
import * as ReactDOM from "react-dom/client";
import { ApolloClient, InMemoryCache, ApolloProvider, createHttpLink, ApolloLink, NormalizedCacheObject } from "@apollo/client";
import App from "./App";
import { setContext } from '@apollo/client/link/context';


const httpLink: ApolloLink = createHttpLink({
  uri: '/api',
});

const authLink: ApolloLink = setContext((_, { headers }) => {
  const token: string = "ValidToken";
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : "",
    }
  }
});

const client: ApolloClient<NormalizedCacheObject> = new ApolloClient({
  link: authLink.concat(httpLink),
  cache: new InMemoryCache()
});

const root: ReactDOM.Root = ReactDOM.createRoot(document.getElementById("root"));

root.render(
  <ApolloProvider client={client}>
    <App />
  </ApolloProvider>
);
