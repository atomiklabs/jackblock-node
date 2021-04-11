# Production image, copy all the files and run next
FROM node:alpine AS runner
WORKDIR /app

ENV NODE_ENV production

COPY . .

RUN npm install -g serve

RUN addgroup -g 1001 -S nodejs
RUN adduser -S serve-static -u 1001
RUN chown -R serve-static:nodejs /app/build
USER serve-static

EXPOSE 5000

ENTRYPOINT [ "serve" ]


