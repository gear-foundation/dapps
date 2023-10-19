import { gql } from 'urql'

export const GetAccountNFTQuery = gql`
  query ($account_id: String!) {
    nfts(where: { owner: { id_eq: $account_id } }) {
      owner {
        id
      }
      name
      description
      mediaUrl
      attribUrl
    }
  }
`

export const GetNftsByNameQuery = gql`
  query ($search_query: String) {
    nfts(
      where: {
        name_containsInsensitive: $search_query
        OR: { owner: { id_eq: $search_query } }
      }
    ) {
      owner {
        id
      }
      name
      description
      mediaUrl
      attribUrl
    }
  }
`
