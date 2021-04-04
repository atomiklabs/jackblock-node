import React, { useEffect, useState } from 'react'
import { Grid, Card, Statistic } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'

function Main(props) {
  const { api } = useSubstrate()

  const [sessionId, setSessionId] = useState(0)

  useEffect(() => {
    let unsubscribe

    api.query.jackBlock
      .sessionId((sessionId) => {
        setSessionId(sessionId.toHuman())
      })
      .then((unsub) => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.query.jackBlock])

  return (
    <Grid.Column>
      <Card>
        <Card.Content textAlign="center">
          <Statistic label="Session ID" value={sessionId} />
        </Card.Content>
      </Card>
    </Grid.Column>
  )
}

export default function SessionId(props) {
  const { api } = useSubstrate()

  return api.query.jackBlock && api.query.jackBlock.sessionId ? <Main {...props} /> : null
}
