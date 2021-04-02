### Generating SVG based on a request

Run `yarn dev` in the module root directory, and then send your request to generate an NFT reward. The endpoint will returned the NTF's metadata object. Example of such request:

```
curl --location --request POST 'http://localhost:3000/api/create-erc721-metadata' \
--header 'Content-Type: application/json' \
--data-raw '{
    "score": 4,
    "scoreOutOf": 6,
    "reward": 1000000,
    "sessionId": "abc"
}'
```
