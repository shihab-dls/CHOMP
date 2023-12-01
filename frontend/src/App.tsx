import { useQuery } from "@apollo/client";
import { gql } from './__generated__/gql';
import React from "react";
import { theme } from "@diamondlightsource/ui-components"
import { ChakraProvider, Alert, AlertIcon, AlertTitle, AlertDescription, Button, HStack } from "@chakra-ui/react";
import { PaginationTable } from "./components/PaginationTable";

const GET_INFO = gql(`
query pinInfo ($after: String) {
  libraryPins(cursor: {first: 2, after: $after}) {
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
`);

// Displays libraryPins query in table component. The table can load more data if required 
function DisplayPinInfo(): React.JSX.Element {
  const { loading, error, data, fetchMore } = useQuery(
    GET_INFO,
    {
      notifyOnNetworkStatusChange: true,
    });

  var loadingRows = loading ? 2 : 0

  if (error) return (
    <Alert status='error'>
      <AlertIcon />
      <AlertTitle>{error.message}</AlertTitle>
      <AlertDescription>{error.extraInfo}</AlertDescription>
    </Alert>
  )

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
    <>
      <PaginationTable 
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
        loadingRows={loadingRows}
      />
      <HStack justify='center' width='100%'>
        <Button 
          colorScheme='teal' 
          variant='outline' 
          onClick={loadMore} 
          isLoading={loadingRows !== 0} 
          loadingText='Loading' 
          isDisabled={data ? !data.libraryPins.pageInfo.hasNextPage : false}
        >
          Load More
        </Button>
      </HStack>
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
