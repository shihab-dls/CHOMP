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

// Displays libraryPins query in table component. The table can load more data if required 
function DisplayPinInfo(): React.JSX.Element {
  const { loading, error, data, fetchMore } = useQuery(
    GET_INFO,
    {
      notifyOnNetworkStatusChange: true,
    });

  if (error) return <p>Error : {error.message} {error.extraInfo}</p>;

  const loadMore = () => {
    fetchMore({

      variables: {
        after: data.libraryPins.pageInfo.endCursor,
      },

      updateQuery: (previousQueryResult, { fetchMoreResult }) => {
        const newEdges = fetchMoreResult.libraryPins.edges;
        const pageInfo = fetchMoreResult.libraryPins.pageInfo;

        // if newEdges actually have items,
        return newEdges.length
          ? // return a reconstruction of the query result with updated values
            {
              ...previousQueryResult,

              libraryPins: {
                ...previousQueryResult.libraryPins,

                edges: [...previousQueryResult.libraryPins.edges, ...newEdges],

                pageInfo,
              },
            }
          : // else, return the previous result
            previousQueryResult;
      },
    });
  };

  return (
    <LoadMoreTable
      headers={[
        {
          key: 'barcode',
          label: 'Barcode',
          skeletonWidth: 12
        },
        {
          key: 'loopSize',
          label: 'Loop Size',
          skeletonWidth: 3
        },
        {
          key: 'status',
          label: 'Status',
          skeletonWidth: 7
        }
      ]}
      data={data ? data.libraryPins.edges.map((edge) => edge.node): []} 
      onButtonClick={loadMore}
      loadingRows={loading ? 2 : 0}
      isDisabled={data ? !data.libraryPins.pageInfo.hasNextPage : false}
      />
  );
}

export default function App(): React.JSX.Element {
  return (
    <ChakraProvider theme={theme}>
      <DisplayPinInfo />
    </ChakraProvider>
  );
}
