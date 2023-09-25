# syntax = docker/dockerfile:1

# Adjust BUN_VERSION as desired
ARG BUN_VERSION=1.0.0
FROM oven/bun:${BUN_VERSION} as base

LABEL fly_launch_runtime="Bun"

# Bun app lives here
WORKDIR /app

# Set production environment
ENV NODE_ENV="production"

# Throw-away build stage to reduce size of final image
FROM base as build

# Install packages needed to build node modules
RUN apt-get update -qq && \
    apt-get install -y build-essential pkg-config python

# Install node modules
COPY --link .npmrc bun.lockb package.json ./
RUN bun install

# Copy application code
COPY --link . .

# Build application
RUN --mount=type=secret,id=ADMIN_PASSWORD \
    --mount=type=secret,id=PUBLIC_FAUNA_GQL_ENDPOINT \
    --mount=type=secret,id=PUBLIC_FAUNA_SECRET \
    --mount=type=secret,id=FAUNA_SECRET \
    --mount=type=secret,id=PUBLIC_TELEMETRYDECK_APP_ID \
    --mount=type=secret,id=PUBLIC_CF_IMAGES_ENDPOINT \
    ADMIN_PASSWORD="$(cat /run/secrets/ADMIN_PASSWORD)" \
    PUBLIC_FAUNA_GQL_ENDPOINT="$(cat /run/secrets/PUBLIC_FAUNA_GQL_ENDPOINT)" \
    PUBLIC_FAUNA_SECRET="$(cat /run/secrets/PUBLIC_FAUNA_SECRET)" \
    FAUNA_SECRET="$(cat /run/secrets/FAUNA_SECRET)" \
    PUBLIC_TELEMETRYDECK_APP_ID="$(cat /run/secrets/PUBLIC_TELEMETRYDECK_APP_ID)" \
    PUBLIC_CF_IMAGES_ENDPOINT="$(cat /run/secrets/PUBLIC_CF_IMAGES_ENDPOINT)" \
    bun run build


# Remove development dependencies
RUN rm -rf node_modules && \
    bun install --ci


# Final stage for app image
FROM base

# Copy built application
COPY --from=build /app /app

# Start the server by default, this can be overwritten at runtime
EXPOSE 3000
CMD [ "bun", "./build/index.js" ]
