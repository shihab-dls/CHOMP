import {
    Box,
    HStack,
    Button,
    Divider,
    Stack,
    BoxProps,
  } from "@chakra-ui/react";
import React from "react";
import {  useEffect, useState } from "react";
  
  type PageChangeCallback = (page: number) => void;
  type ItemChangeCallback = (items: number) => void;
  
  export interface PaginationProps extends BoxProps {
    /** Total number of items to paginate */
    total: number;
    /** Array with all available "items per page" amounts */
    possibleItemsPerPage?: Array<number>;
    /** External bind for current page */
    page?: number;
    /** Number of items to display per page */
    limit?: number;
    /** Callback for page change events */
    onPageChange?: PageChangeCallback;
    /** Callback for item count change event */
    onItemCountChange?: ItemChangeCallback;
  }
  
  const CursorPaginator = ({
    total,
    possibleItemsPerPage = [5, 10, 15, 20, 30, 50, 100],
    limit = 5,
    page,
    onPageChange,
    onItemCountChange,
    ...props
  }: PaginationProps) => {
    const [internalPage, setInternalPage] = useState(page || 1);
    // Use limit set in instance, unless it does not exist in the list of possible items per page.
    // Default to middle.
    const [itemsPerPage] = useState(
      possibleItemsPerPage.includes(limit)
        ? limit
        : possibleItemsPerPage[Math.floor(possibleItemsPerPage.length / 2)],
    );
    const [pageAmount, setPageAmount] = useState(1);
  
    useEffect(() => {
      if (page) {
        setInternalPage(page);
      }
    }, [page]);
  
    useEffect(() => {
      if (onPageChange !== undefined) {
        onPageChange(internalPage);
      }
    }, [internalPage, onPageChange]);
  
    useEffect(() => {
      if (onItemCountChange !== undefined) {
        onItemCountChange(itemsPerPage);
      }
    }, [itemsPerPage, onItemCountChange]);
  
  
    useEffect(() => {
      const newPageAmount = Math.ceil(total / itemsPerPage);
      setInternalPage((prevPage) => (prevPage > newPageAmount ? 1 : prevPage));
      setPageAmount(newPageAmount);
    }, [total, itemsPerPage, setInternalPage]);
  
    return (
      <Box py={2} {...props}>
        <Stack w='100%' direction={{ base: "column", md: "row" }}>
          <HStack>
            <Button
              aria-label='First Page'
              size='sm'
              variant='pgNotSelected'
              onClick={() => setInternalPage(1)}
              isDisabled={internalPage <= 1}
            >
              &lt;&lt;
            </Button>
            <Button
              aria-label='Previous Page'
              size='sm'
              variant='pgNotSelected'
              isDisabled={internalPage <= 1}
              onClick={() => setInternalPage(internalPage - 1)}
            >
              &lt;
            </Button>
            <Button
              aria-label='Next Page'
              size='sm'
              variant='pgNotSelected'
              isDisabled={internalPage >= pageAmount}
              onClick={() => setInternalPage(internalPage + 1)}
            >
              &gt;
            </Button>
            <Button
              aria-label='Last Page'
              size='sm'
              variant='pgNotSelected'
              isDisabled={internalPage >= pageAmount}
              onClick={() => setInternalPage(pageAmount)}
            >
              &gt;&gt;
            </Button>
          </HStack>
          <Divider display={{ base: "none", md: "initial" }} orientation='vertical' h='30px' />
          <HStack flexGrow='1'>
          </HStack>
        </Stack>
      </Box>
    );
  };
  
  export { CursorPaginator };