### Generating SVG based on a request

Run `yarn dev` in the module root directory, and then send your request to generate an SVG markup. Example of such request:

```
curl --location --request POST 'http://localhost:3000/api/generate-svg' \
--header 'Content-Type: application/json' \
--data-raw '{
    "reward": 98234.23,
    "score": 4,
    "scoreOutOf": 6
}'
```
