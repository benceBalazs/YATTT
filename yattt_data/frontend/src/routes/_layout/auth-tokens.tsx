import {
  Button,
  Container,
  Flex,
  Heading,
  SkeletonText,
  Table,
  TableContainer,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@chakra-ui/react"
import { useQuery, useQueryClient } from "@tanstack/react-query"
import { createFileRoute, useNavigate } from "@tanstack/react-router"
import { useEffect } from "react"
import { z } from "zod"

import { AuthTokensService } from "../../client"
import ActionsMenu from "../../components/Common/ActionsMenu"
import Navbar from "../../components/Common/Navbar"
import AddAuthToken from "../../components/AuthTokens/AddAuthToken.tsx"

const authTokensSearchSchema = z.object({
  page: z.number().catch(1),
})

export const Route = createFileRoute("/_layout/auth-tokens")({
  component: AuthTokens,
  validateSearch: (search) => authTokensSearchSchema.parse(search),
})

const PER_PAGE = 5

function getAuthTokensQueryOptions({ page }: { page: number }) {
  return {
    queryFn: () =>
      AuthTokensService.readAuthTokens({ skip: (page - 1) * PER_PAGE, limit: PER_PAGE }),
    queryKey: ["authTokens", { page }],
  }
}

function AuthTokensTable() {
  const queryClient = useQueryClient()
  const { page } = Route.useSearch()
  const navigate = useNavigate({ from: Route.fullPath })
  const setPage = (page: number) =>
    navigate({ search: (prev) => ({ ...prev, page }) })

  const {
    data: authTokens,
    isPending,
    isPlaceholderData,
  } = useQuery({
    ...getAuthTokensQueryOptions({ page }),
    placeholderData: (prevData) => prevData,
  })

  const hasNextPage = !isPlaceholderData && authTokens?.data.length === PER_PAGE
  const hasPreviousPage = page > 1

  useEffect(() => {
    if (hasNextPage) {
      queryClient.prefetchQuery(getAuthTokensQueryOptions({ page: page + 1 }))
    }
  }, [page, queryClient, hasNextPage])

  return (
    <>
      <TableContainer>
        <Table size={{ base: "sm", md: "md" }}>
          <Thead>
            <Tr>
              <Th>ID</Th>
              <Th>TagId</Th>
              <Th>DeviceId</Th>
              <Th>ScannedIn</Th>
              <Th>ScannedOut</Th>
              <Th>Description</Th>
              <Th>Actions</Th>
            </Tr>
          </Thead>
          {isPending ? (
            <Tbody>
              <Tr>
                {new Array(7).fill(null).map((_, index) => (
                  <Td key={index}>
                    <SkeletonText noOfLines={1} paddingBlock="16px" />
                  </Td>
                ))}
              </Tr>
            </Tbody>
          ) : ( authTokens && (
              <Tbody>
                {authTokens.data.map((auth_token) => (
                    <Tr key={auth_token.id}>
                      <Td>{auth_token.id}</Td>
                      <Td>{auth_token.tag_id}</Td>
                      <Td>{auth_token.device_id}</Td>
                      <Td>{auth_token.scanned_in.toString()}</Td>
                      <Td>{auth_token.scanned_out.toString()}</Td>
                      <Td color={!auth_token.description ? 'gray.400' : 'inherit'}>
                        {auth_token.description || 'N/A'}
                      </Td>
                      <Td>
                        <ActionsMenu type={'AuthToken'} value={auth_token} />
                      </Td>
                    </Tr>
                ))}
              </Tbody>
              )
          )}
        </Table>
      </TableContainer>
      <Flex
        gap={4}
        alignItems="center"
        mt={4}
        direction="row"
        justifyContent="flex-end"
      >
        <Button onClick={() => setPage(page - 1)} isDisabled={!hasPreviousPage}>
          Previous
        </Button>
        <span>Page {page}</span>
        <Button isDisabled={!hasNextPage} onClick={() => setPage(page + 1)}>
          Next
        </Button>
      </Flex>
    </>
  )
}

function AuthTokens() {
  return (
    <Container maxW="full">
      <Heading size="lg" textAlign={{ base: "center", md: "left" }} pt={12}>
        Auth Tokens Management
      </Heading>

      <Navbar type={"AuthTokens"} addModalAs={AddAuthToken} />
      <AuthTokensTable />
    </Container>
  )
}
