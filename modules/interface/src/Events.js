import React, { useEffect, useState } from 'react'
import { Feed, Grid, Button, Card } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'

function Main(props) {
  const { api } = useSubstrate()
  const [eventFeed, setEventFeed] = useState([])
  const [sessionResult, setSessionResult] = useState([])

  useEffect(() => {
    let unsub = null
    const allEvents = async () => {
      unsub = await api.query.system.events((events) => {
        events.forEach((record) => {
          const { event, phase } = record
          const types = event.typeDef

          if ('jackBlock' !== event.section) return

          const eventName = `${event.section}:${event.method}:: (phase=${phase.toString()})`
          const params = event.data.map((data, index) => `${types[index].type}: ${data.toString()}`)

          setEventFeed((e) => [
            {
              icon: 'bell',
              summary: `${eventName}-${e.length}`,
              extraText: event.meta.documentation.join(', ').toString(),
              content: params.join(', '),
            },
            ...e,
          ])

          const eventShortName = `${event.section}:${event.method}`
          if ('jackBlock:SessionResults' === eventShortName) setSessionResult(params)
        })
      })
    }

    allEvents()
    return () => unsub && unsub()
  }, [api.query.system])

  return (
    <Grid.Column width={8}>
      {/* Session  */}
      <h1>Results</h1>
      <Card fluid>
        <Card.Content>
          <Card.Description>
            {sessionResult.map((x, i) => (
              <div key={i} style={{ overflowWrap: 'break-word' }}>
                <p>
                  <b>{i}:</b> {x}
                </p>
                <hr />
              </div>
            ))}
          </Card.Description>
        </Card.Content>
      </Card>

      {/* Events */}
      <h1 style={{ float: 'left' }}>Events</h1>
      <Button basic circular size="mini" color="grey" floated="right" icon="erase" onClick={(_) => setEventFeed([])} />
      <Feed style={{ clear: 'both', overflow: 'auto', height: '250px' }} events={eventFeed} />
    </Grid.Column>
  )
}

export default function JackEvents(props) {
  const { api } = useSubstrate()
  return api.query && api.query.system && api.query.system.events ? <Main {...props} /> : null
}
