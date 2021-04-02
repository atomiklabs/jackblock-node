import React, { useEffect, useState } from 'react'
import { Form, Input, Grid, Card, Statistic } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

function Main(props) {
  const { api } = useSubstrate()
  const { accountPair } = props

  // The transaction submission status
  const [status, setStatus] = useState('')

  // The currently stored value
  const [currentValue, setCurrentValue] = useState(0)
  const [formValue, setFormValue] = useState(0)

  useEffect(() => {
    let unsubscribe = getSessionId()
    // console.log('uns', unsubscribe  )

    // api.query.jackBlock
    //   .sessionId((newValue) => {
    //     // The storage value is an Option<u32>
    //     // So we have to check whether it is None first
    //     // There is also unwrapOr
    //     if (newValue.isNone) {
    //       setCurrentValue('<None>')
    //     } else {
    //       console.log('new', newValue.toHuman())

    //       setCurrentValue(newValue.toHuman())
    //     }
    //   })
    //   .then((unsub) => {
    //     unsubscribe = unsub
    //   })
    //   .catch(console.error)


    return () => unsubscribe && unsubscribe()
  }, [api.query.jackBlock])

  const getSessionId = async () => {
    const unsub = await api.query.jackBlock.sessionId((newValue) => {
      // console.log('new', newValue.toHuman())

      // setCurrentBets(newValue.toHuman())
      setCurrentValue(newValue.toHuman())
    })

    // console.log('unsub2', unsub )


    return unsub
  }

  return (
    <Grid.Column width={8}>
      <h1>Session ID</h1>
      <Card centered>
        <Card.Content textAlign="center">
          <Statistic label="Session ID" value={currentValue} />
        </Card.Content>
      </Card>
      <Form>
        <Form.Field>
          <Input label="Add a new bet" state="newValue" type="string" onChange={(_, { value }) => setFormValue(value)} />
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
    </Grid.Column>
  )
}

export default function jackBlock(props) {
  const { api } = useSubstrate()

  return api.query.jackBlock && api.query.jackBlock.sessionId ? <Main {...props} /> : null
}
