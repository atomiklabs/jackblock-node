import React, { useEffect, useState, useRef } from 'react'
import { Grid, Card, Dimmer, Loader } from 'semantic-ui-react'
import axios from 'axios'

import { useSubstrate } from './substrate-lib'

function Main(props) {
  const { api } = useSubstrate()
  const { accountPair } = props
  const accountAddress = useRef(accountPair.address)
  const [nfts, setNFTs] = useState([])
  const nftsCount = useRef(0)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    let unsubscribe

    api.derive.chain
      .bestNumber((_blockN) => {
        loadNFTs()
      })
      .then((unsub) => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.derive.chain.bestNumber])

  useEffect(() => {
    setIsLoading(true)
    setNFTs([])
    nftsCount.current = 0
    accountAddress.current = props.accountPair.address
    loadNFTs()
  }, [props.accountPair.address])

  const loadNFTs = async () => {
    const userTokens = await getUserTokens()
    if (userTokens.length === nftsCount.current) {
      setIsLoading(false)
      return
    }
    setIsLoading(true)
    const userTokensFromIPFS = await getUserTokensFromIPFS(userTokens)
    setNFTs(userTokensFromIPFS)
    nftsCount.current = userTokens.length
    setIsLoading(false)
  }

  const getUserTokens = async () => {
    const tokensByOwner = await getTokensIdByOwner(accountAddress.current)
    const tokensByOwnerWithInfoPromise = tokensByOwner.map(getTokenInfo)
    const tokensByOwnerWithInfoRaw = await Promise.all(tokensByOwnerWithInfoPromise)
    const tokensByOwnerWithInfo = tokensByOwnerWithInfoRaw.map((token) => token.toHuman())
    return tokensByOwnerWithInfo
  }

  const getTokensIdByOwner = async (accountId) => {
    const tokensByOwnerQuery = await api.query.nft.tokensByOwner.entries(accountId)
    const tokensByOwner = tokensByOwnerQuery.map((tokenMapRaw) => {
      const tokenMap = tokenMapRaw[0].toHuman()
      const classIdAndTokenId = tokenMap[1]
      const tokenId = classIdAndTokenId[1]
      return tokenId
    })
    return tokensByOwner
  }

  const getTokenInfo = async (tokenId) => {
    const CLASS_ID = 0
    return api.query.nft.tokens(CLASS_ID, tokenId)
  }

  const getUserTokensFromIPFS = async (userTokensInfo) => {
    const userTokensFromIPFSPromise = userTokensInfo.map(getUserTokenFromIPFSInfo)
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
    <Grid.Column style={{ minHeight: '360px' }}>
      <h1>Your NFTs</h1>
      {isLoading && <LoaderNFTs />}
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

const LoaderNFTs = () => {
  return (
    <Dimmer active inverted>
      <Loader>Loading NFTS...</Loader>
    </Dimmer>
  )
}

export default function NFTs(props) {
  const { api } = useSubstrate()
  return props.accountPair && props.accountPair.address ? <Main {...props} /> : null
}
