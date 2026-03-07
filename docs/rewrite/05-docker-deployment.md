# Docker & Deployment Specification

## Overview

Single VPS deployment using Docker Compose with:
- Caddy (reverse proxy, automatic HTTPS)
- Rust API service
- SvelteKit/Astro frontend
- SQLite database (volume-mounted)
- Crowdsec (security engine)

See also `08-dns-and-security.md` for full Crowdsec configuration.

---

## Directory Structure (on VPS)

```
/opt/www/
├── docker-compose.yml
├── Caddyfile
├── .env                    # Environment variables
├── data/
│   └── app.db              # SQLite database
├── backups/
│   └── app_YYYYMMDD.db     # Daily backups
└── crowdsec/               # Crowdsec configuration
    ├── acquis.yaml         # Log sources
    ├── profiles.yaml       # Ban profiles
    └── bouncer.yaml        # Firewall bouncer config
```

---

## Docker Compose

```yaml
# docker-compose.yml

version: '3.8'

services:
  caddy:
    image: caddy:2-alpine
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile:ro
      - caddy_data:/data
      - caddy_config:/config
    depends_on:
      - frontend
      - api
    networks:
      - web

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    restart: unless-stopped
    environment:
      - NODE_ENV=production
      - PORT=3000
    networks:
      - web

  api:
    build:
      context: ./backend
      dockerfile: Dockerfile
    restart: unless-stopped
    environment:
      - DATABASE_URL=/data/app.db
      - RUST_LOG=info,vr_www_api=debug
      # SMTP (configurable provider)
      - SMTP_HOST=${SMTP_HOST}
      - SMTP_PORT=${SMTP_PORT}
      - SMTP_USER=${SMTP_USER}
      - SMTP_PASS=${SMTP_PASS}
      # Friendly Captcha (Germany)
      - FRIENDLY_CAPTCHA_SECRET=${FRIENDLY_CAPTCHA_SECRET}
      - FRIENDLY_CAPTCHA_SITEKEY=${FRIENDLY_CAPTCHA_SITEKEY}
      # Email settings
      - EMAIL_FROM=${EMAIL_FROM}
      - EMAIL_TO=${EMAIL_TO}
      - BASE_URL=${BASE_URL}
    volumes:
      - ./data:/data
    networks:
      - web

volumes:
  caddy_data:
  caddy_config:

networks:
  web:
    driver: bridge
```

---

## Caddyfile

```caddyfile
# Caddyfile

valentinrogg.de {
    # API routes → Rust backend
    handle /api/* {
        reverse_proxy api:8080
    }
    
    # Health check
    handle /health {
        reverse_proxy api:8080
    }
    
    # Everything else → Frontend
    handle {
        reverse_proxy frontend:3000
    }
    
    # Security headers
    header {
        X-Content-Type-Options nosniff
        X-Frame-Options DENY
        Referrer-Policy strict-origin-when-cross-origin
        -Server
    }
    
    # Gzip compression
    encode gzip
    
    # Logging
    log {
        output file /var/log/caddy/access.log
        format json
    }
}

# Redirect www to non-www
www.valentinrogg.de {
    redir https://valentinrogg.de{uri} permanent
}
```

---

## Backend Dockerfile

```dockerfile
# backend/Dockerfile

# Build stage
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev sqlite-dev

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build actual application
COPY . .
RUN touch src/main.rs  # Force rebuild
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

RUN apk add --no-cache sqlite-libs ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/vr-www-api /app/vr-www-api
COPY --from=builder /app/migrations /app/migrations

ENV DATABASE_URL=/data/app.db
ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["/app/vr-www-api"]
```

---

## Frontend Dockerfile (SvelteKit)

```dockerfile
# frontend/Dockerfile

# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build

# Runtime stage
FROM node:20-alpine

WORKDIR /app

COPY --from=builder /app/build ./build
COPY --from=builder /app/package*.json ./
RUN npm ci --omit=dev

ENV NODE_ENV=production
ENV PORT=3000

EXPOSE 3000

CMD ["node", "build"]
```

---

## Frontend Dockerfile (Astro Static)

```dockerfile
# frontend/Dockerfile

# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build

# Runtime stage - serve static files with Caddy
FROM caddy:2-alpine

COPY --from=builder /app/dist /srv

EXPOSE 80

CMD ["caddy", "file-server", "--root", "/srv", "--listen", ":80"]
```

For static Astro, update docker-compose to not use separate Caddy:

```yaml
# Alternative: Astro serves itself
frontend:
  build:
    context: ./frontend
    dockerfile: Dockerfile
  restart: unless-stopped
  networks:
    - web
```

---

## Environment Variables

```bash
# .env

# Database
DATABASE_URL=/data/app.db

# SMTP (configure for your provider - see options below)
SMTP_HOST=
SMTP_PORT=465
SMTP_USER=
SMTP_PASS=

# Friendly Captcha (German provider)
# Get keys from https://friendlycaptcha.com/
FRIENDLY_CAPTCHA_SECRET=your-secret-key
FRIENDLY_CAPTCHA_SITEKEY=your-site-key

# Email configuration
EMAIL_FROM=noreply@valentinrogg.de
EMAIL_TO=mail@valentinrogg.de

# App settings
BASE_URL=https://valentinrogg.de

# Logging
RUST_LOG=info,vr_www_api=debug
```

### Provider Setup

**Email Provider Options:**

You can use any SMTP provider. Here are some options:

| Provider | Cost | Notes |
|----------|------|-------|
| **Resend** | Free < 100/day | API-based, easy setup, current provider |
| **mailbox.org** | ~€3/month | German, GDPR-compliant, traditional SMTP |
| **Sendgrid** | Free < 100/day | Popular, good deliverability |
| **Self-hosted** | Free | Full control, more maintenance |

**Example: Resend (current)**
```bash
SMTP_HOST=smtp.resend.com
SMTP_PORT=465
SMTP_USER=resend
SMTP_PASS=re_xxxxx  # API key
```

**Example: mailbox.org (optional)**
```bash
SMTP_HOST=smtp.mailbox.org
SMTP_PORT=465
SMTP_USER=you@mailbox.org
SMTP_PASS=your-app-specific-password
```

**Friendly Captcha (Free < 1k/month)**
1. Sign up at friendlycaptcha.com
2. Create new site
3. Copy sitekey (for frontend) and secret (for backend)
4. Free tier: 1,000 verifications/month
5. Paid: €9/month for 10,000 verifications

---

## Hetzner VPS Setup

### Recommended Server

**Hetzner CX22** (~€4/month)
- 2 vCPU (Intel)
- 4 GB RAM
- 40 GB SSD
- 20 TB traffic

### Initial Setup

```bash
# Update system
apt update && apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com | sh

# Install Docker Compose
apt install docker-compose-plugin -y

# Create app user
useradd -m -s /bin/bash www
usermod -aG docker www

# Create directories
mkdir -p /opt/www/{data,backups}
chown -R www:www /opt/www
```

### Firewall (UFW)

```bash
ufw default deny incoming
ufw default allow outgoing
ufw allow ssh
ufw allow http
ufw allow https
ufw enable
```

### Deploy

```bash
# As www user
cd /opt/www

# Clone/copy files
git clone <repo> .
# or scp files

# Set environment variables
cp .env.example .env
nano .env  # Edit with real values

# Build and start
docker compose build
docker compose up -d

# Check logs
docker compose logs -f
```

---

## CI/CD (GitHub Actions)

```yaml
# .github/workflows/deploy.yml

name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      
      - name: Build images
        run: |
          docker compose build
          docker save -o images.tar $(docker compose config --images)
      
      - name: Copy to server
        uses: appleboy/scp-action@v0.1.7
        with:
          host: ${{ secrets.SERVER_HOST }}
          username: www
          key: ${{ secrets.SSH_PRIVATE_KEY }}
          source: "images.tar,docker-compose.yml,Caddyfile"
          target: /opt/www
      
      - name: Deploy
        uses: appleboy/ssh-action@v1.0.3
        with:
          host: ${{ secrets.SERVER_HOST }}
          username: www
          key: ${{ secrets.SSH_PRIVATE_KEY }}
          script: |
            cd /opt/www
            docker load -i images.tar
            docker compose up -d
            rm images.tar
            docker system prune -f
```

### Alternative: Build on Server

```yaml
# .github/workflows/deploy.yml

name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
      - name: Deploy to server
        uses: appleboy/ssh-action@v1.0.3
        with:
          host: ${{ secrets.SERVER_HOST }}
          username: www
          key: ${{ secrets.SSH_PRIVATE_KEY }}
          script: |
            cd /opt/www
            git pull origin main
            docker compose build
            docker compose up -d
            docker system prune -f
```

---

## Backup Strategy

### Cron Job

```bash
# /etc/cron.d/www-backup

# Daily backup at 3 AM
0 3 * * * www /opt/www/backup.sh
```

### Backup Script

```bash
#!/bin/bash
# /opt/www/backup.sh

set -e

DB_PATH="/opt/www/data/app.db"
BACKUP_DIR="/opt/www/backups"
DATE=$(date +%Y%m%d_%H%M%S)

# Ensure backup directory exists
mkdir -p "$BACKUP_DIR"

# Checkpoint WAL to ensure consistency
docker compose exec -T api sqlite3 /data/app.db "PRAGMA wal_checkpoint(TRUNCATE);"

# Create backup
docker compose exec -T api sqlite3 /data/app.db ".backup '/data/backup_${DATE}.db'"

# Move backup to host
mv "/opt/www/data/backup_${DATE}.db" "$BACKUP_DIR/app_${DATE}.db"

# Compress
gzip "$BACKUP_DIR/app_${DATE}.db"

# Keep last 7 days
find "$BACKUP_DIR" -name "app_*.db.gz" -mtime +7 -delete

echo "Backup completed: app_${DATE}.db.gz"
```

### Restore from Backup

```bash
# Stop services
docker compose down

# Restore
gunzip -c /opt/www/backups/app_YYYYMMDD_HHMMSS.db.gz > /opt/www/data/app.db

# Start services
docker compose up -d
```

---

## Monitoring

### Health Check Endpoint

The API exposes `/health` for monitoring:

```bash
curl https://valentinrogg.de/health
# Response: OK
```

### Simple Uptime Monitoring

Use free services:
- [UptimeRobot](https://uptimerobot.com/) - 50 monitors free
- [Healthchecks.io](https://healthchecks.io/) - Cron job monitoring

### Docker Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f api

# Last 100 lines
docker compose logs --tail=100 api
```

### Resource Monitoring

```bash
# Container stats
docker stats

# Disk usage
df -h

# Database size
du -h /opt/www/data/app.db
```

---

## SSL/TLS

Caddy automatically provisions and renews Let's Encrypt certificates. No manual configuration needed.

For custom certificates:

```caddyfile
valentinrogg.de {
    tls /path/to/cert.pem /path/to/key.pem
    
    # ... rest of config
}
```

---

## Scaling Considerations

For a personal site, single VPS is sufficient. If needed later:

1. **Vertical scaling**: Upgrade to CX32 or CX42
2. **Database**: Move to managed PostgreSQL (Hetzner doesn't offer this, consider Supabase EU)
3. **CDN**: Add Cloudflare in front (just DNS + proxy, no workers)
4. **Multiple instances**: Add load balancer, share SQLite via Litestream → S3

But realistically, CX22 handles thousands of requests/day easily.

---

## Security Checklist

- [ ] SSH key authentication only (disable password auth)
- [ ] Firewall configured (UFW)
- [ ] Automatic security updates enabled
- [ ] Docker images from trusted sources
- [ ] Environment variables not committed to git
- [ ] HTTPS enforced (Caddy handles this)
- [ ] Security headers configured
- [ ] Rate limiting on API endpoints (optional, add later if needed)
- [ ] Regular backups tested
