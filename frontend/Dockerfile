# Build stage
FROM denoland/deno:alpine AS builder

ARG FONTAWESOME_TOKEN
# PUBLIC_API_URL - Client-side browser requests (empty = same-origin in production)
ARG PUBLIC_API_URL=
# API_URL - Server-side requests within Docker network
ARG API_URL=http://api:8080
ARG GIT_COMMIT=

WORKDIR /app

# Install dependencies
COPY package.json deno.json ./
RUN echo "@awesome.me:registry=https://npm.fontawesome.com/" > .npmrc && \
  echo "@fortawesome:registry=https://npm.fontawesome.com/" >> .npmrc && \
  echo "//npm.fontawesome.com/:_authToken=${FONTAWESOME_TOKEN}" >> .npmrc && \
  deno install && \
  rm -f .npmrc

# Copy source
COPY . .

# Build the application
ENV NODE_ENV=production
ENV PUBLIC_API_URL=${PUBLIC_API_URL}
ENV API_URL=${API_URL}
ENV PUBLIC_GIT_COMMIT=${GIT_COMMIT}
RUN deno task build

# Runtime stage
FROM denoland/deno:alpine

WORKDIR /app

COPY --from=builder /app/dist ./dist

ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000

CMD ["deno", "run", "--allow-net", "--allow-read", "--allow-env", "./dist/server/entry.mjs"]
