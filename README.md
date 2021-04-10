# JackBlock - Substrate blockchain for Polkadot
Jackblock is a **lottery game** build in [Substrate](https://www.substrate.io/) for [Polkadot ecosystem](https://polkadot.network/).
Players can be rewarded with extra **coins** and **dynamic NFT's with a unique image representation!** hosted on **IPFS**.

Jackblock testnet is available here: [https://www.jackblock.network](https://www.jackblock.network)

### Rules in a nutshell
- To participate in the game, **player can bet 6 numbers** within a range **1 - 10**
- Each **bet costs 1 coin** and it's transfered to shared **session pot**
- Session is closed **every 5 blocks**, then **6 magic numbers are announced**
- Each player who bet some correct numbers, would be rewarded accordignly. All split rules are available here [Session pot split rules](https://github.com/korzewski/jackblock/wiki/Jackblock-session-pot-split-rules)
- Additionally each winner would get a **dynamic NFT with a unique image representation**!

### DAPP - Interface preview (in progress)
![alt text](https://github.com/korzewski/jackblock/blob/master/readme-files/dapp_interface.png?raw=true)

### High level architecture diagram

![alt text](https://github.com/korzewski/jackblock/blob/master/readme-files/high_level_architecture_diagram.jpg?raw=true)

### Blockchain architecture diagram

![alt text](https://github.com/korzewski/jackblock/blob/master/readme-files/jackblock_architecture.jpg?raw=true)

### (Option 1) Start substrate development blockchain
- Make sure you have installed everything for substrate development https://substrate.dev/docs/en/knowledgebase/getting-started/
- |terminal 1| To run your blockchain you simply execute: `make run` (and wait...)
- |terminal 2| Add predefined keys to keystore: `make keystore-add`
- |terminal 2| Run nodejs NFT's generator
  - Open `./modules/svg-bonanza`
  - Install npm dependencies: `yarn`
  - Run NFT's generator app: `yarn dev`

Congrats, your blockchain works! 
You may follow the console logs to identify lottery session results:
`--- Finalize_the_session: 5`
`--- Session_numbers: [8, 10, 5, 7, 4, 9]`
`--- Winners: []`

### Interact with blockchain via web interface
- Open in your terminal `./modules/interface`
- Install npm dependencies: `yarn`
- Run interfce app: `yarn start`
- This should open your browser at `http://localhost:8000/substrate-front-end-template`
  - We know... this is work in progress development environment :)
  - Now you can join to the game by adding your 6 numbers guess. Example bet of `[1, 2, 3, 4, 5, 6]` has to be written in hex format like so `0x010203040506`
  - Each session is finalized in **5 blocks period**, so wait for the winners announcement!
  - At this moment each winner is going to get a **dynamic NFT with a unique image representation!** (This process may take about 30sec)

 Awesome, you can now interact and apply new bets for current lottery session! ;)

### (Option 2) Start substrate local private network
- Remove `tmp-private-chain` folder (if exist inside project root)
- |terminal 1| -  Build executable file: `make node-build`
- |terminal 1| - Start predefined Node-0 instance: `make local-node-0-start`
- |terminal 2| - Start predefined Node-1 instance: `make local-node-1-start`
- |terminal 3| - Add predefined keys to keystore: `make local-add-all-keys`
- |terminal 1| - Stop Node-0 and run it again: `make local-node-0-start`
- |terminal 2| - Stop Node-1 and run it again: `make local-node-1-start`

At this point you should be able to see new blocks initializing & finalizing properly.



### (Option 3 - advanced) Start public network
- Remove `tmp-public-chain` folder (if exist inside project root)
- Build executable file: `make node-build`
- Each node has to identify themselves for `aura` (initializing) and `grandpa` (finalizing) block consensus:
  - Use `./keys/local-node-0-aura.json` and `./keys/local-node-0-grandpa.json` as a template
  - Create `./keys/private-key-aura.json` and `./keys/private-key-grandpa.json` where you replace only **Private Seed** and **Public key**
- One person has to initialize the network
  - Create **publicChainSpecRaw.json** file and share it with your validators: `make public-chain-spec`
  - Run node: `make public-boot-node-start`
  - This will log in console something like this: "**Local node identity is**: `12D3KooWJxtLahuTcXBZhrpCbmjWqQnNEtM3sctoubmU6F5Dr2YY`"
  - Share this identity with validators
  - Add aura and grandpa keys to keystore: `make public-node-add-keys`
- Rest of the validators:
  - Run node: `make public-node-start NAME=<validator-name> BOOT_NODE_IP=<boot-node-ip-address> BOOT_NODE_IDENTITY=<initial-node-identity>`
  - Add aura and grandpa keys to keystore: `make public-node-add-keys`

At this point you should be able to see new blocks initializing & finalizing properly. 

# Enjoy :)
