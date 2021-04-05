import React, { useEffect, useState } from 'react'
import { Grid, Card, Statistic } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'

const POT_ADDRESS = '5EYCAe5b71oc992GHYBjkkKJ4oEY7LiZuQUkN7Czcru7ggzs'

function Main(props) {
  const { api } = useSubstrate()

  const [potBalance, setPotBalance] = useState('')

  useEffect(() => {
    let unsubscribe

    api.query.system
      .account(POT_ADDRESS, (pot) => {
        const {
          data: { free: previousFree },
        } = pot
        setPotBalance(previousFree.toHuman())
      })
      .then((unsub) => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.query.system])

  return (
    <Grid.Column>
      <Card>
        <Card.Content textAlign="center">
          <Statistic size="tiny" label="Pot balance" value={potBalance} />
        </Card.Content>
      </Card>
    </Grid.Column>
  )
}

export default function PotBalance(props) {
  const { api } = useSubstrate()

  return api.query.system && api.query.system.account ? <Main {...props} /> : null
}
