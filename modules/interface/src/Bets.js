import React, { useEffect, useState } from 'react'
import { Form, Input, Grid, Card } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

function Main(props) {
  const { api } = useSubstrate()
  const { accountPair } = props

  const [status, setStatus] = useState('')
  const [bets, setBets] = useState([])
  const [formValue, setFormValue] = useState(0)

  useEffect(() => {
    let unsubscribe

    api.derive.chain
      .bestNumber(() => {
        getBets()
      })
      .then((unsub) => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.derive.chain.bestNumber])

  const getBets = async () => {
    const sessionId = await api.query.jackBlock.sessionId()
    const bets = await api.query.jackBlock.bets(sessionId.toHuman())
    setBets(bets.toHuman())
  }

  return (
    <Grid.Column width={8}>
      <h1>Bets</h1>
      <Form>
        <Form.Field>
          <Input
            label="Add a new bet"
            state="newValue"
            type="string"
            onChange={(_, { value }) => setFormValue(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label="AddNewBet"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'jackBlock',
              callable: 'addNewBet',
              interxType: 'EXTRINSIC',
              inputParams: [formValue],
              paramFields: [true],
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>

      <h3>All Bets</h3>
      <Card fluid>
        <Card.Content>
          <Card.Description>
            {bets.map(({ account_id, guess_numbers }, i) => (
              <div key={i} style={{ overflowWrap: 'break-word' }}>
                <p>
                  <b>account_id:</b> {account_id}
                </p>
                <p>
                  <b>guess_numbers:</b> {guess_numbers}
                </p>
                <hr />
              </div>
            ))}
          </Card.Description>
        </Card.Content>
      </Card>
    </Grid.Column>
  )
}

export default function Bets(props) {
  const { api } = useSubstrate()

  return api.query.jackBlock && api.query.jackBlock.sessionId ? <Main {...props} /> : null
}
