# DNS Migration & Security Setup

## Overview

Replace Cloudflare with:
- **DNS:** deSEC (German non-profit, free)
- **Firewall:** Hetzner Cloud Firewall (network-level)
- **Security:** Crowdsec (application-level, French open-source)

---

## Part 1: DNS Migration to deSEC

### Why deSEC?

- German non-profit (Berlin)
- Free, no tracking
- DNSSEC support
- REST API for automation
- Privacy-focused

### 1.1 Set Up deSEC Account

1. Go to https://desec.io/
2. Create account (email verification)
3. Add your domain: `valentinrogg.de`
4. deSEC provides nameservers:
   ```
   ns1.desec.io
   ns2.desec.org
   ```

### 1.2 Configure DNS Records

In deSEC dashboard or via API:

```
# A Records (required)
valentinrogg.de.       A      <HETZNER_VPS_IP>
www.valentinrogg.de.   A      <HETZNER_VPS_IP>

# CAA (only Let's Encrypt can issue certs)
valentinrogg.de.       CAA    0 issue "letsencrypt.org"

# =============================================================================
# Mail Records (ONLY if switching to mailbox.org or another email provider)
# Skip this section if keeping current email provider (e.g., Resend)
# =============================================================================

# Example: mailbox.org MX records
# valentinrogg.de.       MX     10 mxext1.mailbox.org.
# valentinrogg.de.       MX     20 mxext2.mailbox.org.
# valentinrogg.de.       MX     30 mxext3.mailbox.org.

# Example: mailbox.org SPF
# valentinrogg.de.       TXT    "v=spf1 include:mailbox.org ~all"

# Example: DKIM (get from your email provider's dashboard)
# mbo0001._domainkey.valentinrogg.de.  TXT  "v=DKIM1; k=rsa; p=..."

# DMARC (recommended regardless of provider)
_dmarc.valentinrogg.de. TXT   "v=DMARC1; p=quarantine; rua=mailto:dmarc@valentinrogg.de"
```

> **Note:** MX, SPF, and DKIM records are only needed if you're switching email providers.
> If keeping Resend or your current setup, you may not need to change these.

### 1.3 Migration Steps

**Before migration (while still on Cloudflare):**

1. Lower TTL on all records to 300 seconds
2. Wait 24 hours for old TTL to expire
3. Document all existing DNS records

**Migration:**

1. Set up all records in deSEC (don't change nameservers yet)
2. At IONOS/Strato registrar, change nameservers to:
   - `ns1.desec.io`
   - `ns2.desec.org`
3. Wait for propagation (check with `dig valentinrogg.de NS`)

**After migration:**

1. Verify all records resolve correctly
2. Test email delivery (SPF/DKIM)
3. Verify HTTPS works (Caddy will get new cert)
4. Remove domain from Cloudflare
5. Increase TTL back to 3600+ seconds

### 1.4 deSEC API (Optional)

For automation:

```bash
# Get API token from deSEC dashboard

# List records
curl -H "Authorization: Token YOUR_TOKEN" \
  https://desec.io/api/v1/domains/valentinrogg.de/rrsets/

# Update A record
curl -X PUT \
  -H "Authorization: Token YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"records": ["NEW_IP"]}' \
  https://desec.io/api/v1/domains/valentinrogg.de/rrsets/www/A/
```

---

## Part 2: Hetzner Cloud Firewall

Network-level firewall, blocks traffic before it reaches your VPS.

### 2.1 Create Firewall

Via Hetzner Cloud Console:

1. Go to **Firewalls** → **Create Firewall**
2. Name: `www-firewall`
3. Add inbound rules:

| Protocol | Port | Source | Description |
|----------|------|--------|-------------|
| TCP | 22 | Your IP or 0.0.0.0/0 | SSH |
| TCP | 80 | 0.0.0.0/0 | HTTP |
| TCP | 443 | 0.0.0.0/0 | HTTPS |
| ICMP | - | 0.0.0.0/0 | Ping (optional) |

4. Apply to your VPS

### 2.2 Via Hetzner CLI

```bash
# Install hcloud CLI
brew install hcloud  # macOS
# or download from https://github.com/hetznercloud/cli

# Authenticate
hcloud context create www
# Enter API token from Hetzner Cloud Console

# Create firewall
hcloud firewall create --name www-firewall

# Add rules
hcloud firewall add-rule www-firewall \
  --direction in --protocol tcp --port 22 \
  --source-ips 0.0.0.0/0 --description "SSH"

hcloud firewall add-rule www-firewall \
  --direction in --protocol tcp --port 80 \
  --source-ips 0.0.0.0/0 --description "HTTP"

hcloud firewall add-rule www-firewall \
  --direction in --protocol tcp --port 443 \
  --source-ips 0.0.0.0/0 --description "HTTPS"

# Apply to server
hcloud firewall apply-to-resource www-firewall \
  --type server --server your-server-name
```

### 2.3 Optional: Restrict SSH to Your IP

For better security, only allow SSH from your IP:

```bash
hcloud firewall add-rule www-firewall \
  --direction in --protocol tcp --port 22 \
  --source-ips YOUR_HOME_IP/32 --description "SSH from home"
```

---

## Part 3: Crowdsec Setup

Application-level security engine that:
- Parses logs for attack patterns
- Blocks malicious IPs
- Shares threat intelligence with community
- Protects against brute force, scanners, CVE exploits

### 3.1 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Crowdsec Agent                                             │
│  - Reads Caddy access logs                                  │
│  - Detects attack patterns                                  │
│  - Adds malicious IPs to local blocklist                    │
│  - Shares with Crowdsec community (optional)                │
└─────────────────────────┬───────────────────────────────────┘
                          │ API
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  Crowdsec Bouncer (Caddy plugin or firewall)                │
│  - Queries agent for IP decisions                           │
│  - Blocks banned IPs at request level                       │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Docker Compose Integration

Update `docker-compose.yml`:

```yaml
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
      - caddy_logs:/var/log/caddy
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
      - SMTP_HOST=${SMTP_HOST}
      - SMTP_PORT=${SMTP_PORT}
      - SMTP_USER=${SMTP_USER}
      - SMTP_PASS=${SMTP_PASS}
      - FRIENDLY_CAPTCHA_SECRET=${FRIENDLY_CAPTCHA_SECRET}
      - FRIENDLY_CAPTCHA_SITEKEY=${FRIENDLY_CAPTCHA_SITEKEY}
      - EMAIL_FROM=${EMAIL_FROM}
      - EMAIL_TO=${EMAIL_TO}
      - BASE_URL=${BASE_URL}
    volumes:
      - ./data:/data
    networks:
      - web

  # Crowdsec security engine
  crowdsec:
    image: crowdsecurity/crowdsec:latest
    restart: unless-stopped
    volumes:
      - ./crowdsec/acquis.yaml:/etc/crowdsec/acquis.yaml:ro
      - ./crowdsec/profiles.yaml:/etc/crowdsec/profiles.yaml:ro
      - crowdsec_config:/etc/crowdsec
      - crowdsec_data:/var/lib/crowdsec/data
      - caddy_logs:/var/log/caddy:ro
    environment:
      - COLLECTIONS=crowdsecurity/caddy crowdsecurity/http-cve crowdsecurity/base-http-scenarios
      - GID=1000
    networks:
      - web

  # Crowdsec firewall bouncer (uses iptables)
  crowdsec-firewall-bouncer:
    image: crowdsecurity/crowdsec-firewall-bouncer-iptables:latest
    restart: unless-stopped
    network_mode: host
    cap_add:
      - NET_ADMIN
      - NET_RAW
    volumes:
      - ./crowdsec/bouncer.yaml:/etc/crowdsec/bouncers/crowdsec-firewall-bouncer.yaml:ro
    depends_on:
      - crowdsec

volumes:
  caddy_data:
  caddy_config:
  caddy_logs:
  crowdsec_config:
  crowdsec_data:

networks:
  web:
    driver: bridge
```

### 3.3 Crowdsec Configuration Files

**crowdsec/acquis.yaml** - Log sources:

```yaml
# Caddy access logs
filenames:
  - /var/log/caddy/access.log
labels:
  type: caddy
---
# Caddy JSON format
filenames:
  - /var/log/caddy/access.json
labels:
  type: caddy
```

**crowdsec/profiles.yaml** - What to do with detected attacks:

```yaml
name: default_ip_remediation
filters:
  - Alert.Remediation == true && Alert.GetScope() == "Ip"
decisions:
  - type: ban
    duration: 4h
on_success: break
---
name: high_confidence_ban
filters:
  - Alert.Remediation == true && Alert.GetConfidence() >= 3
decisions:
  - type: ban
    duration: 24h
on_success: break
```

**crowdsec/bouncer.yaml** - Firewall bouncer config:

```yaml
mode: iptables
update_frequency: 10s
log_mode: file
log_dir: /var/log/
log_level: info
log_compression: true
log_max_size: 100
log_max_backups: 3
log_max_age: 30
api_url: http://crowdsec:8080/
api_key: ${CROWDSEC_BOUNCER_API_KEY}
insecure_skip_verify: false
disable_ipv6: false
deny_action: DROP
deny_log: true
deny_log_prefix: "crowdsec: "
blacklists_ipv4: crowdsec-blacklists
blacklists_ipv6: crowdsec6-blacklists
iptables_chains:
  - INPUT
  - FORWARD
  - DOCKER-USER
```

### 3.4 Caddyfile with Logging

Update Caddyfile for JSON logging (easier for Crowdsec to parse):

```caddyfile
{
    log {
        output file /var/log/caddy/access.json {
            roll_size 100mb
            roll_keep 5
        }
        format json
    }
}

valentinrogg.de {
    # API routes
    handle /api/* {
        reverse_proxy api:8080
    }
    
    # Health check
    handle /health {
        reverse_proxy api:8080
    }
    
    # Frontend
    handle {
        reverse_proxy frontend:3000
    }
    
    # Security headers
    header {
        X-Content-Type-Options nosniff
        X-Frame-Options DENY
        Referrer-Policy strict-origin-when-cross-origin
        X-XSS-Protection "1; mode=block"
        -Server
    }
    
    encode gzip
}

www.valentinrogg.de {
    redir https://valentinrogg.de{uri} permanent
}
```

### 3.5 Initial Setup Commands

After first `docker compose up`:

```bash
# Generate bouncer API key
docker compose exec crowdsec cscli bouncers add firewall-bouncer

# Copy the API key to crowdsec/bouncer.yaml

# Restart bouncer
docker compose restart crowdsec-firewall-bouncer

# Check Crowdsec status
docker compose exec crowdsec cscli metrics

# View decisions (banned IPs)
docker compose exec crowdsec cscli decisions list

# View alerts
docker compose exec crowdsec cscli alerts list
```

### 3.6 Useful Crowdsec Commands

```bash
# Add IP to whitelist
docker compose exec crowdsec cscli decisions add --ip YOUR_IP --type unban

# Manually ban an IP
docker compose exec crowdsec cscli decisions add --ip BAD_IP --duration 24h --reason "manual ban"

# Remove ban
docker compose exec crowdsec cscli decisions delete --ip BAD_IP

# Update scenarios/parsers
docker compose exec crowdsec cscli hub update
docker compose exec crowdsec cscli hub upgrade

# View installed collections
docker compose exec crowdsec cscli collections list

# Install additional protection
docker compose exec crowdsec cscli collections install crowdsecurity/wordpress  # if needed
docker compose exec crowdsec cscli collections install crowdsecurity/nginx      # if needed
```

### 3.7 Optional: Crowdsec Console

Register at https://app.crowdsec.net/ for:
- Dashboard view of attacks
- Community blocklists (free tier available)
- Alert notifications

```bash
# Enroll in console
docker compose exec crowdsec cscli console enroll YOUR_ENROLLMENT_KEY
```

---

## Part 4: Rate Limiting in Caddy

Additional protection against abuse:

```caddyfile
{
    order rate_limit before reverse_proxy
}

valentinrogg.de {
    # Global rate limit: 100 requests per 10 seconds per IP
    rate_limit {
        zone global {
            key {remote_host}
            events 100
            window 10s
        }
    }
    
    # Stricter limit for API
    handle /api/* {
        rate_limit {
            zone api {
                key {remote_host}
                events 30
                window 10s
            }
        }
        reverse_proxy api:8080
    }
    
    # ... rest
}
```

---

## Part 5: Geo-blocking (Optional)

If you want to block specific countries:

### Option A: In Rust API

```rust
// Add to Cargo.toml
// maxminddb = "0.24"

use maxminddb::{geoip2, Reader};
use std::net::IpAddr;

const BLOCKED_COUNTRIES: &[&str] = &["CN", "RU", "KP", "IR"];

pub fn is_blocked_country(ip: IpAddr, reader: &Reader<Vec<u8>>) -> bool {
    if let Ok(country) = reader.lookup::<geoip2::Country>(ip) {
        if let Some(code) = country.country.and_then(|c| c.iso_code) {
            return BLOCKED_COUNTRIES.contains(&code);
        }
    }
    false
}

// Use in middleware
async fn geo_block_middleware(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    if is_blocked_country(addr.ip(), &state.geoip_reader) {
        return (StatusCode::FORBIDDEN, "Access denied").into_response();
    }
    next.run(request).await
}
```

Download GeoLite2 database (free, requires MaxMind account):
```bash
# Add to Dockerfile or mount as volume
wget "https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-Country&license_key=YOUR_KEY&suffix=tar.gz"
```

### Option B: Block at Firewall Level (More Efficient)

Download country IP ranges and block with iptables:

```bash
# Download country IP ranges
wget -O /tmp/cn.zone https://www.ipdeny.com/ipblocks/data/countries/cn.zone
wget -O /tmp/ru.zone https://www.ipdeny.com/ipblocks/data/countries/ru.zone

# Create ipset
ipset create blocked_countries hash:net

# Add ranges
for ip in $(cat /tmp/cn.zone /tmp/ru.zone); do
    ipset add blocked_countries $ip
done

# Block with iptables
iptables -I INPUT -m set --match-set blocked_countries src -j DROP
```

Or let Crowdsec handle it with GeoIP scenarios.

---

## Security Stack Summary

| Layer | Solution | Purpose |
|-------|----------|---------|
| **DNS** | deSEC | German, privacy-focused DNS |
| **Network** | Hetzner Firewall | Block all except 22/80/443 |
| **Application** | Crowdsec | Attack detection & blocking |
| **Transport** | Caddy | HTTPS, rate limiting |
| **API** | Rust | Geo-blocking, input validation |

---

## Environment Variables Update

Add to `.env`:

```bash
# ... existing vars ...

# Crowdsec
CROWDSEC_BOUNCER_API_KEY=generated-by-cscli
```

---

## Testing Security

```bash
# Test rate limiting
for i in {1..150}; do curl -s -o /dev/null -w "%{http_code}\n" https://valentinrogg.de/; done

# Check if Crowdsec is working
docker compose exec crowdsec cscli metrics

# Simulate attack (from another machine)
nikto -h https://valentinrogg.de/  # Should get banned

# Check ban
docker compose exec crowdsec cscli decisions list
```
