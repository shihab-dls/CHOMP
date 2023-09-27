/* eslint-disable react/jsx-no-undef */
import { useQuery, gql, DocumentNode } from "@apollo/client";
import React from "react";
import { theme } from "@diamondlightsource/ui-components"
import { ChakraProvider } from "@chakra-ui/react";
import { LoadMoreTable } from "./components/LoadMoreTable";

const GET_INFO: DocumentNode = gql`
query pinInfo ($after: String) {
  libraryPins(first: 2, after: $after) {
    pageInfo {
      hasPreviousPage,
      hasNextPage,
      startCursor,
      endCursor
    },
    edges {
      cursor
      node {
        barcode,
        loopSize,
        status
      }
    }
  }
}
`;

function DisplayPinInfo(): React.JSX.Element {
  const { loading, error, data, fetchMore } = useQuery(GET_INFO);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error : {error.message} {error.extraInfo}</p>;

  // function to load more content and update query result
  const loadMore = () => {
    // fetchMore function from `useQuery` to fetch more content with `updateQuery`
    fetchMore({

      // update `after` variable with `endCursor` from previous result
      variables: {
        after: data.libraryPins.pageInfo.endCursor,
      },

      // pass previous query result and the new results to `updateQuery`
      updateQuery: (previousQueryResult, { fetchMoreResult }) => {
        // define edges and pageInfo from new results
        const newEdges = fetchMoreResult.libraryPins.edges;
        const pageInfo = fetchMoreResult.libraryPins.pageInfo;
        console.log(pageInfo)

        // if newEdges actually have items,
        return newEdges.length
          ? // return a reconstruction of the query result with updated values
            {
              // spread the value of the previous result
              ...previousQueryResult,

              libraryPins: {
                // spread the value of the previous `allStarhips` data into this object
                ...previousQueryResult.libraryPins,

                // concatenate edges
                edges: [...previousQueryResult.libraryPins.edges, ...newEdges],

                // override with new pageInfo
                pageInfo,
              },
            }
          : // else, return the previous result
            previousQueryResult;
      },
    });
  };

  return (
    <>
    <LoadMoreTable
      headers={[
        {
          key: 'barcode',
          label: 'Barcode'
        },
        {
          key: 'loopSize',
          label: 'Loop Size'
        },
        {
          key: 'status',
          label: 'Status'
        }
      ]}
      data={data.libraryPins.edges.map((edge) => edge.node)} 
      onButtonClick={loadMore}
      />
    </>
  );
}

export default function App(): React.JSX.Element {
  return (
    <ChakraProvider theme={theme}>
      <DisplayPinInfo />
    </ChakraProvider>
  );
}
