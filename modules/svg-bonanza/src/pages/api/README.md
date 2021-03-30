### Generating SVG based on a request

Run `yarn dev` in the module root directory, and then send your request to generate an NFT reward. The endpoint will returned the NTF's metadata object. Example of such request:

```
curl --location --request POST 'http://localhost:3000/api/generate-nft-reward' \
--header 'Content-Type: application/json' \
--data-raw '{
    "reward": 98234.23,
    "score": 4,
    "scoreOutOf": 6,
    "sessionId": "skibidibiba"
}'
```
