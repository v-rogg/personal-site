# Migration Checklist

## Overview

Step-by-step checklist for migrating from Cloudflare (Pages + Workers + D1) to Hetzner (Docker + SQLite).

---

## Phase 1: Preparation

### 1.1 Export Existing Data

- [ ] Export signatures from D1 database
  ```bash
  wrangler d1 execute personal-site --command "SELECT * FROM signatures" --json > signatures_export.json
  ```

- [ ] Document current PostHog events (for reference, not migration)
  - Event names and properties
  - Current metrics/dashboards

- [ ] Backup current codebase
  ```bash
  git checkout main
  git pull
  git checkout -b pre-migration-backup
  git push origin pre-migration-backup
  ```

### 1.2 Set Up Hetzner

- [ ] Create Hetzner Cloud account
- [ ] Provision CX22 VPS (Falkenstein or Nuremberg datacenter for Germany)
- [ ] Configure SSH access
  - Generate SSH key if needed
  - Add public key to server
  - Disable password authentication
- [ ] Configure Hetzner Cloud Firewall
  - Allow TCP 22 (SSH)
  - Allow TCP 80 (HTTP)
  - Allow TCP 443 (HTTPS)
  - Apply to server
- [ ] Configure UFW on server (secondary firewall)
- [ ] Install Docker and Docker Compose
- [ ] Create directory structure

### 1.3 Set Up DNS (deSEC - German provider)

See `08-dns-and-security.md` for full details.

- [ ] Create deSEC account at https://desec.io/
- [ ] Add domain `valentinrogg.de`
- [ ] Configure DNS records:
  - A records for domain and www
  - MX records (if switching email provider)
  - SPF/DKIM/DMARC for email authentication
  - CAA record for Let's Encrypt
- [ ] At IONOS/Strato: Lower TTL to 300 seconds (wait 24h)
- [ ] At IONOS/Strato: Change nameservers to deSEC
  - `ns1.desec.io`
  - `ns2.desec.org`
- [ ] Wait for DNS propagation
- [ ] Verify with `dig valentinrogg.de NS`

### 1.4 Set Up EU Service Providers

**Email Provider (Optional Switch)**

You can continue using Resend (current) or switch to another provider:

| Provider | Setup |
|----------|-------|
| **Resend (current)** | No change needed, use existing API key |
| **mailbox.org** | Create account (~€3/month), get app-specific password |
| **Other SMTP** | Configure host, port, user, pass |

- [ ] Decide: Keep Resend or switch provider?
- [ ] If switching: Set up new provider and note SMTP credentials

**Friendly Captcha (CAPTCHA - Germany)**
- [ ] Sign up at friendlycaptcha.com
- [ ] Create new site/application
- [ ] Note credentials:
  - `FRIENDLY_CAPTCHA_SITEKEY` (for frontend)
  - `FRIENDLY_CAPTCHA_SECRET` (for backend)

### 1.5 Prepare Environment

- [ ] Create `.env` file on server (not in git) with:
  ```bash
  # SMTP (configure for your chosen provider)
  SMTP_HOST=...
  SMTP_PORT=465
  SMTP_USER=...
  SMTP_PASS=...
  
  FRIENDLY_CAPTCHA_SECRET=...
  FRIENDLY_CAPTCHA_SITEKEY=...
  EMAIL_FROM=...
  EMAIL_TO=...
  BASE_URL=https://valentinrogg.de
  ```

---

## Phase 2: Backend Development

### 2.1 Initialize Rust Project

- [ ] Create `backend/` directory
- [ ] Initialize Cargo project
- [ ] Add dependencies to `Cargo.toml`
- [ ] Set up project structure:
  ```
  backend/src/
  ├── main.rs
  ├── config.rs
  ├── error.rs
  ├── db/
  ├── models/
  ├── routes/
  ├── services/
  └── cache/
  ```

### 2.2 Database Layer

- [ ] Create SQLite migrations (`migrations/001_init.sql`)
- [ ] Implement database connection pool
- [ ] Test migrations run correctly
- [ ] Implement models:
  - [ ] User
  - [ ] Session
  - [ ] Signature
  - [ ] Event

### 2.3 API Routes

- [ ] `POST /api/sessions` - Create session
- [ ] `POST /api/events` - Track event
- [ ] `GET /api/signatures` - List approved signatures
- [ ] `GET /api/signatures/:id` - Get single signature
- [ ] `POST /api/signatures` - Create signature
- [ ] `POST /api/contact` - Submit contact form
- [ ] `GET /api/stats` - Get analytics
- [ ] `GET /health` - Health check

### 2.4 Services

- [ ] Email service (SMTP)
  - [ ] Signature confirmation email
  - [ ] Contact form email
- [ ] CAPTCHA service (Friendly Captcha verification)

### 2.5 Caching

- [ ] In-memory signature cache
- [ ] Cache refresh on signature approval

### 2.6 Testing

- [ ] Unit tests for models
- [ ] Integration tests for routes
- [ ] Test with sample data

### 2.7 Dockerfile

- [ ] Create multi-stage Dockerfile
- [ ] Test build locally
- [ ] Verify image size is reasonable (<100MB)

---

## Phase 3: Frontend Migration

### 3.1 Choose Path

- [ ] Decision: SvelteKit or Astro?
  - SvelteKit: Faster migration, familiar codebase
  - Astro: Better performance, cleaner architecture

### 3.2 SvelteKit Path

- [ ] Remove `src/routes/api/` directory
- [ ] Remove `src/lib/d1.ts`
- [ ] Create new utilities:
  - [ ] `src/lib/api.ts` - API client
  - [ ] `src/lib/session.ts` - Session management
  - [ ] `src/lib/tracking.ts` - Event tracking
- [ ] Update components:
  - [ ] Signature editor (use new API)
  - [ ] Signature carousel (use new API)
  - [ ] Contact form (use new API)
  - [ ] Analytics display (use new API)
- [ ] Update `+page.server.ts` files
- [ ] Remove PostHog integration
- [ ] Replace Turnstile with Friendly Captcha widget
- [ ] Add Friendly Captcha Svelte component
- [ ] Update adapter configuration
- [ ] Create Dockerfile

### 3.3 Astro Path (Alternative)

- [ ] Initialize Astro project
- [ ] Configure Svelte integration
- [ ] Migrate layouts
- [ ] Migrate static pages to `.astro`
- [ ] Move interactive components to islands
- [ ] Set up content collections for blog
- [ ] Create API utilities
- [ ] Create Dockerfile

### 3.4 Testing

- [ ] Test all pages render correctly
- [ ] Test signature creation flow
- [ ] Test contact form submission
- [ ] Test analytics display
- [ ] Test on mobile devices

---

## Phase 4: Data Migration

### 4.1 Prepare Migration Script

- [ ] Create script to transform D1 export to new schema
- [ ] Handle session creation for existing signatures
- [ ] Verify data integrity

### 4.2 Test Migration

- [ ] Run migration on test database
- [ ] Verify all signatures imported
- [ ] Verify queries return expected results

---

## Phase 5: Deployment

### 5.1 Infrastructure

- [ ] Create `docker-compose.yml` (with Crowdsec)
- [ ] Create `Caddyfile` (with JSON logging for Crowdsec)
- [ ] Create `crowdsec/` directory with config files:
  - `acquis.yaml`
  - `profiles.yaml`
  - `bouncer.yaml`
- [ ] Copy files to server
- [ ] Create `.env` with production values

### 5.2 Initial Deployment

- [ ] Build images on server
  ```bash
  docker compose build
  ```
- [ ] Start services
  ```bash
  docker compose up -d
  ```
- [ ] Verify Caddy obtains SSL certificate
- [ ] Check logs for errors
  ```bash
  docker compose logs -f
  ```

### 5.3 Set Up Crowdsec

See `08-dns-and-security.md` for full details.

- [ ] Generate bouncer API key:
  ```bash
  docker compose exec crowdsec cscli bouncers add firewall-bouncer
  ```
- [ ] Add API key to `crowdsec/bouncer.yaml`
- [ ] Restart bouncer:
  ```bash
  docker compose restart crowdsec-firewall-bouncer
  ```
- [ ] Verify Crowdsec is working:
  ```bash
  docker compose exec crowdsec cscli metrics
  ```
- [ ] Optional: Enroll in Crowdsec Console for dashboard

### 5.4 Import Data

- [ ] Copy migration script and data to server
- [ ] Run migration
- [ ] Verify data in new database

### 5.5 Verify Deployment

- [ ] Test all endpoints manually
- [ ] Test signature creation
- [ ] Test contact form
- [ ] Test analytics
- [ ] Check mobile responsiveness

---

## Phase 6: Go Live

### 6.1 Pre-Launch Verification

- [ ] Verify everything works on new server (test with `/etc/hosts` override)
- [ ] Test email sending (signature confirmation, contact form)
- [ ] Test Friendly Captcha integration
- [ ] Verify Crowdsec is blocking test attacks
- [ ] Prepare rollback plan

### 6.2 DNS Propagation

(If nameservers were changed in Phase 1.3)

- [ ] Verify DNS propagation complete: `dig valentinrogg.de NS`
- [ ] Verify A records resolve to Hetzner IP
- [ ] Test from multiple locations/devices

### 6.3 Post-Launch

- [ ] Monitor Crowdsec for attacks:
  ```bash
  docker compose exec crowdsec cscli alerts list
  ```
- [ ] Monitor logs for errors
- [ ] Verify SSL working correctly
- [ ] Check analytics events are being recorded
- [ ] Increase DNS TTL back to 3600+ seconds

---

## Phase 7: Cleanup

### 7.1 Decommission Cloudflare

- [ ] Wait 48-72 hours for DNS propagation
- [ ] Disable Cloudflare Workers
- [ ] Delete D1 database (after confirming migration success)
- [ ] Remove Cloudflare Pages deployment
- [ ] Cancel any paid Cloudflare services

### 7.2 Repository Cleanup

- [ ] Remove `workers/` directory
- [ ] Remove Cloudflare-specific config files
- [ ] Update README with new architecture
- [ ] Archive old deployment scripts

### 7.3 Set Up Backups (Nextcloud)

See `07-backup-to-nextcloud.md` for full details.

- [ ] Create Nextcloud app password for backups
- [ ] Create `/Backups/vr-www/` folder in Nextcloud
- [ ] Generate GPG key pair for encryption
- [ ] Store GPG private key in password manager (NOT only on VPS!)
- [ ] Create `/opt/www/.backup-credentials` file
- [ ] Install backup scripts (`backup.sh`, `restore.sh`, `verify-backups.sh`)
- [ ] Set up cron jobs for daily backup and weekly verification
- [ ] Test full backup → restore cycle
- [ ] Verify backup appears in Nextcloud
- [ ] Document recovery procedure

### 7.4 Set Up Monitoring

- [ ] Configure UptimeRobot or similar
- [ ] Test backup failure alerting (optional)
- [ ] Document operational procedures

---

## Phase 8: CI/CD

### 8.1 GitHub Actions

- [ ] Create deployment workflow
- [ ] Add server secrets to GitHub
- [ ] Test automated deployment
- [ ] Verify rollback process

---

## Rollback Plan

If issues occur after go-live:

1. **Immediate**: At registrar (IONOS/Strato), revert nameservers to Cloudflare
2. **Within 1-48 hours**: DNS propagates, traffic returns to old setup
3. **Investigate**: Check logs on new server
4. **Fix**: Address issues
5. **Retry**: Change nameservers back to deSEC

Keep Cloudflare setup running for at least 1 week after successful migration.

---

## Post-Migration Verification

### Functional Tests

- [ ] Homepage loads
- [ ] Blog posts render
- [ ] Signature carousel displays
- [ ] Signature editor opens
- [ ] Can create new signature
- [ ] Signature confirmation email received
- [ ] Contact form submits
- [ ] Contact form email received
- [ ] Analytics page shows data
- [ ] Mobile responsive

### Performance Tests

- [ ] Homepage TTFB < 200ms
- [ ] Lighthouse score > 90
- [ ] No console errors
- [ ] Images load correctly

### Security Tests

- [ ] HTTPS working
- [ ] Security headers present
- [ ] No sensitive data exposed
- [ ] CAPTCHA working
- [ ] Crowdsec blocking attacks
- [ ] Hetzner firewall active

---

## EU-Only Stack Summary

| Component | Provider | Location | Cost |
|-----------|----------|----------|------|
| **Hosting** | Hetzner | Germany | ~€4/month |
| **DNS** | deSEC | Germany | Free |
| **Firewall** | Hetzner Cloud Firewall | Germany | Free |
| **Security** | Crowdsec | France (self-hosted) | Free |
| **Database** | SQLite (self-hosted) | Germany | €0 |
| **Backups** | Nextcloud (self-hosted) | Your server | €0 |
| **Email** | Resend / mailbox.org / other | Varies | Varies |
| **CAPTCHA** | Friendly Captcha | Germany | Free (<1k/month) |
| **Domain** | IONOS/Strato | Germany | Existing |
| **Total** | | | **~€4-7/month** |

No Cloudflare dependency. GDPR compliance depends on email provider choice.

---

## Timeline Estimate

| Phase | Duration |
|-------|----------|
| Preparation | 1 day |
| Backend Development | 2-3 days |
| Frontend Migration | 1-2 days |
| Data Migration | 0.5 day |
| Deployment | 0.5 day |
| DNS Cutover | 1 day (mostly waiting) |
| Cleanup & CI/CD | 1 day |
| **Total** | **7-9 days** |

---

## Files Created in This Rewrite

```
docs/rewrite/
├── 01-architecture-overview.md
├── 02-database-schema.md
├── 03-rust-api-specification.md
├── 04-frontend-migration.md
├── 05-docker-deployment.md
├── 06-migration-checklist.md    # This file
├── 07-backup-to-nextcloud.md    # Encrypted backups via WebDAV
└── 08-dns-and-security.md       # deSEC, Hetzner Firewall, Crowdsec
```

Use these documents as reference when implementing the migration.
