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
RUN \
    --mount=type=secret,id=PUBLIC_CF_IMAGES_ENDPOINT \
    --mount=type=secret,id=PUBLIC_STORYBLOK_TOKEN \
    --mount=type=secret,id=PUBLIC_POSTHOG_TOKEN \
    --mount=type=secret,id=PUBLIC_VERSION_DATE \
    --mount=type=secret,id=PUBLIC_SIGNATURES_WORKER \
    --mount=type=secret,id=ADMIN_PASSWORD \
    --mount=type=secret,id=WORKER_PSK \
    --mount=type=secret,id=RESEND_TOKEN \
    PUBLIC_CF_IMAGES_ENDPOINT="$(cat /run/secrets/PUBLIC_CF_IMAGES_ENDPOINT)" \
    PUBLIC_STORYBLOK_TOKEN="$(cat /run/secrets/PUBLIC_STORYBLOK_TOKEN)" \
    PUBLIC_POSTHOG_TOKEN="$(cat /run/secrets/PUBLIC_POSTHOG_TOKEN)" \
    PUBLIC_VERSION_DATE="$(cat /run/secrets/PUBLIC_VERSION_DATE)" \
    PUBLIC_SIGNATURES_WORKER="$(cat /run/secrets/PUBLIC_SIGNATURES_WORKER)" \
    ADMIN_PASSWORD="$(cat /run/secrets/ADMIN_PASSWORD)" \
    WORKER_PSK="$(cat /run/secrets/WORKER_PSK)" \
    RESEND_TOKEN="$(cat /run/secrets/RESEND_TOKEN)" \
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
