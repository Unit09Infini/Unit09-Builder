FROM node:22-alpine AS builder

WORKDIR /app

COPY package.json tsconfig.json ./
COPY src ./src

RUN npm install && npm run build

FROM node:22-alpine

WORKDIR /app

ENV NODE_ENV=production

COPY --from=builder /app/package.json ./package.json
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/dist ./dist

EXPOSE 8080

CMD ["node", "dist/index.js"]
