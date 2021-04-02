import React, { useEffect, useState } from 'react'
import { Form, Input, Grid, Card, Statistic } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

function Main(props) {
  const { api } = useSubstrate()
  const { accountPair } = props

  const [currentValue, setCurrentValue] = useState([])
  const [potBalance, setpotBalance] = useState(0)

  useEffect(() => {
    let unsubscribe

    api.derive.chain
      .bestNumber((newValue) => {
        getBets()
      })
      .then((unsub) => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.derive.chain.bestNumber])

  const getBets = async () => {
    const currentSessionId = await api.query.jackBlock.sessionId()
    const bets = await api.query.jackBlock.bets(currentSessionId)
    // console.log('bets', bets.toHuman())

    setCurrentValue(bets.toHuman())

    let {
      data: { free: previousFree },
      nonce: previousNonce,
    } = await api.query.system.account('5EYCAe5b71oc992GHYBjkkKJ4oEY7LiZuQUkN7Czcru7ggzs')

    setpotBalance(previousFree.toHuman())
  }

  return (
    <Grid.Column width={8}>
      <h1>Bets</h1>
      <Card centered>
        <Card.Content >
          <Card.Description>
            {currentValue.map(({ account_id, guess_numbers }, i) => (
              <div key={i}>
                <p><b>account_id:</b> {account_id}</p>
                <p><b>guess_numbers:</b> {guess_numbers}</p>
                <hr />
              </div>
            ))}
          </Card.Description>
        </Card.Content>
      </Card>

      <h1>Pot</h1>
      <Card centered>
        <Card.Content textAlign="center">
          <Statistic label="Balance" value={potBalance} />
        </Card.Content>
      </Card>
    </Grid.Column>
  )
}

export default function TemplateModule(props) {
  const { api } = useSubstrate()
  return api.derive.chain.bestNumber ? <Main {...props} /> : null
}
