import React, { useEffect, useState } from 'react'
import { Grid, Card } from 'semantic-ui-react'
import axios from 'axios'

import { useSubstrate } from './substrate-lib'

function Main(props) {
  const { api } = useSubstrate()
  const { accountPair } = props

  const [nfts, setNFTs] = useState([])

  useEffect(() => {
    let unsubscribe

    api.derive.chain
      .bestNumber((blockN) => {
        const everySecondBlock = blockN.toHuman() % 2
        if (everySecondBlock) loadNFTs()
      })
      .then((unsub) => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.derive.chain.bestNumber])

  const loadNFTs = async () => {
    // const userTokens = await getUserTokens()
    // const userTokensFromIPFS = await getUserTokensFromIPFS(userTokens)
    // setNFTs(userTokensFromIPFS)

    const tokenIPFSUri = 'bafkreiakognvnpaukbw4tgpevs3qwsdc22r7tnsgaxfcrpxt6eerdviwji'
    const userTokens = [{ data: tokenIPFSUri }, { data: tokenIPFSUri }, { data: tokenIPFSUri }] // = getUserTokens()
    const userTokensFromIPFS = await getUserTokensFromIPFS(userTokens)
    setNFTs(userTokensFromIPFS)
  }

  const getUserTokens = async () => {
    const accountId = accountPair.address
    const tokensByOwner = await api.query.nft.tokensByOwner(accountId, [0, 0]) // (accountId, [classId, tokenId ???])
    const tokensByOwnerWithInfoPromise = tokensByOwner.map(getTokenInfo)
    const tokensByOwnerWithInfo = await Promise.all(tokensByOwnerWithInfoPromise)
    return tokensByOwnerWithInfo
  }

  const getTokenInfo = async (data) => {
    const { classId, tokenId } = data
    return api.query.nft.tokens(classId, tokenId) // (classId, tokenId)
  }

  const getUserTokensFromIPFS = async (userTokens) => {
    const userTokensFromIPFSPromise = userTokens.map(getUserTokenFromIPFSInfo)
    const userTokensFromIPFS = await Promise.all(userTokensFromIPFSPromise)
    return userTokensFromIPFS
  }

  const getUserTokenFromIPFSInfo = async (token) => {
    const uri = token.data
    try {
      const { data } = await axios.get(`https://ipfs.io/ipfs/${uri}`)
      return data
    } catch (error) {
      console.error(error)
    }
  }

  return (
    <Grid.Column>
      <h1>Your NFTs</h1>
      <Card.Group itemsPerRow={3}>
        {nfts.map((nft, i) => (
          <NFT {...nft} key={i} />
        ))}
      </Card.Group>
    </Grid.Column>
  )
}

const NFT = (props) => {
  const nftProps = props.properties
  const name = nftProps.name.description
  const description = nftProps.description.description
  const imageSrc = nftProps.image.description

  return (
    <Card>
      <svg style={{ height: '355px' }}>
        <image href={imageSrc} style={{ width: '100%' }} />
      </svg>

      <Card.Content>
        <Card.Header>{name}</Card.Header>
        <Card.Description>{description}</Card.Description>
      </Card.Content>
    </Card>
  )
}

export default function NFTs(props) {
  const { api } = useSubstrate()
  return api.query.jackBlock ? <Main {...props} /> : null
}
