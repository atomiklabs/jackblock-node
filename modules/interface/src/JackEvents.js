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
        // loop through the Vec<EventRecord>
        events.forEach((record) => {
          // extract the phase, event and the event types
          const { event, phase } = record
          const types = event.typeDef

          // show what we are busy with
          const eventName = `${event.section}:${event.method}:: (phase=${phase.toString()})`
          const ev = `${event.section}:${event.method}`

          if ('jackBlock' !== event.section) return

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

          if ('jackBlock:SessionResults' === ev) {
            setSessionResult(params)
          }
        })
      })
    }

    allEvents()
    return () => unsub && unsub()
  }, [api.query.system])

  const { feedMaxHeight = 250 } = props

  return (
    <Grid.Column width={8}>
      <h1>Session Results</h1>
      <Card centered>
        <Card.Content>
          <Card.Description>
            {sessionResult.map((x, i) => (
              <div key={i}>
                <p>
                  <b>{i}:</b> {x}
                </p>
                <hr />
              </div>
            ))}
          </Card.Description>
        </Card.Content>
      </Card>

      <h1 style={{ float: 'left' }}>JackBlock Events</h1>
      <Button basic circular size="mini" color="grey" floated="right" icon="erase" onClick={(_) => setEventFeed([])} />
      <Feed style={{ clear: 'both', overflow: 'auto', maxHeight: feedMaxHeight }} events={eventFeed} />
    </Grid.Column>
  )
}

export default function Events(props) {
  const { api } = useSubstrate()
  return api.query && api.query.system && api.query.system.events ? <Main {...props} /> : null
}
