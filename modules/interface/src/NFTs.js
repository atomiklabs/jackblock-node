import React, { useEffect, useState } from 'react'
import { Grid, Card } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import NFT_JSON from './mock/nfts.json'
// https://ipfs.io/ipfs/bafkreiakognvnpaukbw4tgpevs3qwsdc22r7tnsgaxfcrpxt6eerdviwji

const NFTS_MOCK = [NFT_JSON, NFT_JSON, NFT_JSON]

function Main(props) {
  const { api } = useSubstrate()
  const { accountPair } = props

  const [nfts, setNFTs] = useState(NFTS_MOCK)

  // useEffect(() => {
  //   let unsubscribe;
  //   api.query.jackBlock.nfts(newValue => {
  //     setNFTs(newValue.unwrap().toNumber());
  //   }).then(unsub => {
  //     unsubscribe = unsub;
  //   })
  //     .catch(console.error);

  //   return () => unsubscribe && unsubscribe();
  // }, [api.query.jackBlock]);

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
  return api.query.jackBlock ? (
    // return api.query.jackBlock && api.query.jackBlock.nfts
    <Main {...props} />
  ) : null
}
