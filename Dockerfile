# syntax = docker/dockerfile:1

# Adjust BUN_VERSION as desired
#ARG BUN_VERSION=1.0.0
#FROM oven/bun:${BUN_VERSION} as base
ARG NODE_VERSION=20
FROM node:${NODE_VERSION}-slim as base

LABEL fly_launch_runtime="Node.js"

# Bun app lives here
WORKDIR /app

# Set production environment
ENV NODE_ENV="production"
ENV PUBLIC_NODE_ENV="production"

# Throw-away build stage to reduce size of final image
FROM base as build

# Install packages needed to build node modules
RUN apt-get update -qq && \
    apt-get install -y python3 build-essential pkg-config

# Install node modules
COPY --link .npmrc package.json package-lock.json ./
RUN npm ci --include=dev

# Copy application code
COPY --link . .

# Build application
RUN --mount=type=secret,id=ADMIN_PASSWORD \
    --mount=type=secret,id=PUBLIC_FAUNA_GQL_ENDPOINT \
    --mount=type=secret,id=PUBLIC_FAUNA_SECRET \
    --mount=type=secret,id=FAUNA_SECRET \
    --mount=type=secret,id=PUBLIC_TELEMETRYDECK_APP_ID \
    --mount=type=secret,id=PUBLIC_CF_IMAGES_ENDPOINT \
    --mount=type=secret,id=PUBLIC_STORYBLOK_TOKEN \
    --mount=type=secret,id=PUBLIC_VERSION_DATE \
    ADMIN_PASSWORD="$(cat /run/secrets/ADMIN_PASSWORD)" \
    PUBLIC_FAUNA_GQL_ENDPOINT="$(cat /run/secrets/PUBLIC_FAUNA_GQL_ENDPOINT)" \
    PUBLIC_FAUNA_SECRET="$(cat /run/secrets/PUBLIC_FAUNA_SECRET)" \
    FAUNA_SECRET="$(cat /run/secrets/FAUNA_SECRET)" \
    PUBLIC_TELEMETRYDECK_APP_ID="$(cat /run/secrets/PUBLIC_TELEMETRYDECK_APP_ID)" \
    PUBLIC_CF_IMAGES_ENDPOINT="$(cat /run/secrets/PUBLIC_CF_IMAGES_ENDPOINT)" \
    PUBLIC_STORYBLOK_TOKEN="$(cat /run/secrets/PUBLIC_STORYBLOK_TOKEN)" \
    PUBLIC_VERSION_DATE="$(cat /run/secrets/PUBLIC_VERSION_DATE)" \
    npm run build


# Remove development dependencies
RUN npm prune --omit=dev


# Final stage for app image
FROM base

# Copy built application
COPY --from=build /app /app

# Start the server by default, this can be overwritten at runtime
EXPOSE 3000
CMD [ "node", "./build/index.js" ]
