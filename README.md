# JackBlock - Substrate blockchain
Jackblock is a **lottery game** build in [**Substrate**](https://www.substrate.io/).
Players can be rewarded with extra **coins** and **dynamic NFT's** hosted on **IPFS**.

### Rules in a nutshell
- To participate in the game, **player can bet 6 numbers** within a range **1 - 49**
- Each **bet costs 100 coins** and it's added to shared **session pot**
- Session is closed **every 24h**, then **6 magic numbers are announced**.
- Each player who bet some correct numbers, would be rewarded accordignly. All split rules are available here [Session pot split rules](https://github.com/korzewski/jackblock/wiki/Jackblock-session-pot-split-rules)
- Additionally each player has a chance to win a **unique NFT** !

### Architecture diagram

![alt text](https://github.com/korzewski/jackblock/blob/master/readme-files/jackblock-concept_v1.jpg?raw=true)

### Run `nodes` and start private network
- Remove `tmp-private-chain` folder (if exist inside project root)
- |terminal 1| -  Build executable file: `make node-build`
- |terminal 1| - Start predefined Node-0 instance: `make node-0-start`
- |terminal 2| - Start predefined Node-1 instance: `make node-1-start`
- |terminal 3| - Add predefined keys to keystore: `make add-all-keys`
- |terminal 1| - Stop Node-0 and run it again: `make node-0-start`
- |terminal 2| - Stop Node-1 and run it again: `make node-1-start`

At this point you should be able to see new blocks initializing & finalizing properly. 
Congrats, your blockchain works! enjoy ;)