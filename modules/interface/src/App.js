import React, { useState, createRef } from 'react'
import { Container, Dimmer, Loader, Grid, Sticky, Message } from 'semantic-ui-react'
import 'semantic-ui-css/semantic.min.css'

import { SubstrateContextProvider, useSubstrate } from './substrate-lib'
import { DeveloperConsole } from './substrate-lib/components'

import AccountSelector from './AccountSelector'
import BlockNumber from './BlockNumber'
import Bets from './Bets'
import Events from './Events'
import SessionId from './SessionId'
import PotBalance from './PotBalance'
import NFTs from './NFTs'

function Main() {
  const [accountAddress, setAccountAddress] = useState(null)
  const { apiState, keyring, keyringState, apiError } = useSubstrate()
  const accountPair = accountAddress && keyringState === 'READY' && keyring.getPair(accountAddress)

  const loader = (text) => (
    <Dimmer active>
      <Loader size="small">{text}</Loader>
    </Dimmer>
  )

  const message = (err) => (
    <Grid centered columns={2} padded>
      <Grid.Column>
        <Message
          negative
          compact
          floating
          header="Error Connecting to Substrate"
          content={`${JSON.stringify(err, null, 4)}`}
        />
      </Grid.Column>
    </Grid>
  )

  if (apiState === 'ERROR') return message(apiError)
  else if (apiState !== 'READY') return loader('Connecting to Substrate')

  if (keyringState !== 'READY') {
    return loader("Loading accounts (please review any extension's authorization)")
  }

  const contextRef = createRef()

  return (
    <div ref={contextRef}>
      <Sticky context={contextRef}>
        <AccountSelector setAccountAddress={setAccountAddress} />
      </Sticky>
      <Container>
        <Grid stackable columns="equal">
          <Grid.Row stretched>
            <PotBalance />
            <SessionId />
            <BlockNumber />
            <BlockNumber finalized />
          </Grid.Row>
          <Grid.Row>
            <Bets accountPair={accountPair} />
            <Events />
          </Grid.Row>
          <Grid.Row>
            <NFTs accountPair={accountPair} />
          </Grid.Row>
        </Grid>
      </Container>
      <DeveloperConsole />
    </div>
  )
}

export default function App() {
  return (
    <SubstrateContextProvider>
      <Main />
    </SubstrateContextProvider>
  )
}
