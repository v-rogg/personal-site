# Architecture Overview

## Current State (Cloudflare)

```
┌─────────────────────────────────────────────────────────────────┐
│  Cloudflare Pages                                               │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  SvelteKit Frontend                                       │  │
│  │  - SSR pages                                              │  │
│  │  - API routes (proxy to workers)                          │  │
│  │  - Form actions (email, CAPTCHA)                          │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
         │                                    │
         ▼                                    ▼
┌─────────────────────────┐    ┌─────────────────────────────────┐
│  vr-www-signatures-rs   │    │  vr-www-posthog-analytics-rs    │
│  (Cloudflare Worker)    │    │  (Cloudflare Worker)            │
│  - CRUD signatures      │    │  - Query PostHog API            │
│  - D1 database          │    │  - Aggregate stats              │
└───────────┬─────────────┘    └───────────────┬─────────────────┘
            │                                  │
            ▼                                  ▼
┌─────────────────────────┐    ┌─────────────────────────────────┐
│  Cloudflare D1          │    │  PostHog (External)             │
│  - signatures table     │    │  - Event storage                │
└─────────────────────────┘    └─────────────────────────────────┘
```

### Current Pain Points

1. **Scattered logic**: Email sending and CAPTCHA validation in SvelteKit, data in workers
2. **External dependency**: PostHog for simple analytics
3. **Data residency**: Cloudflare infrastructure not guaranteed in Germany
4. **Multiple deployments**: Two workers + Pages deployment
5. **No user tracking**: Events not linked to signatures/users

---

## Target State (Hetzner)

```
┌─────────────────────────────────────────────────────────────────┐
│  Hetzner VPS (Docker Compose)                                   │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Caddy (Reverse Proxy)                                    │  │
│  │  - Automatic HTTPS                                        │  │
│  │  - Route /api/* → Rust service                            │  │
│  │  - Route /* → Frontend                                    │  │
│  └───────────────────────────────────────────────────────────┘  │
│                         │                                       │
│           ┌─────────────┴─────────────┐                         │
│           ▼                           ▼                         │
│  ┌─────────────────────┐  ┌───────────────────────────────────┐ │
│  │  Astro/SvelteKit    │  │  Rust API Service (Axum)          │ │
│  │  Frontend           │  │                                   │ │
│  │  - Static pages     │  │  Endpoints:                       │ │
│  │  - Svelte islands   │  │  - /api/sessions                  │ │
│  │  - No API logic     │  │  - /api/events                    │ │
│  │                     │  │  - /api/signatures                │ │
│  │                     │  │  - /api/contact                   │ │
│  │                     │  │  - /api/stats                     │ │
│  │                     │  │                                   │ │
│  │                     │  │  Integrations:                    │ │
│  │                     │  │  - Resend (email)                 │ │
│  │                     │  │  - Turnstile (CAPTCHA)            │ │
│  └─────────────────────┘  └───────────────┬───────────────────┘ │
│                                           │                     │
│                                           ▼                     │
│                           ┌───────────────────────────────────┐ │
│                           │  SQLite (WAL mode)                │ │
│                           │  - users                          │ │
│                           │  - sessions                       │ │
│                           │  - signatures                     │ │
│                           │  - events                         │ │
│                           │                                   │ │
│                           │  In-Memory Cache:                 │ │
│                           │  - Approved signatures            │ │
│                           └───────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## Key Architectural Decisions

### 1. Single Rust Service

Merge `vr-www-signatures-rs` and `vr-www-posthog-analytics-rs` into one Axum service.

**Rationale:**
- Shared database access
- Single deployment
- No inter-service latency
- Unified error handling and logging

### 2. SQLite with WAL Mode

Replace D1 + PostHog with local SQLite.

**Rationale:**
- Sub-millisecond queries
- No external dependencies
- Data stays in Germany
- Simple backups (file copy)

### 3. In-Memory Signature Cache

Keep approved signatures in memory, sync with SQLite.

**Rationale:**
- Instant reads for carousel
- Small dataset (~100 signatures × ~10KB = ~1MB)
- Write-through caching

### 4. GDPR-Compliant Session Tracking

- Session ID generated in-memory (frontend), not persisted client-side
- User record created only on identifying action (signature, contact form)
- Session linked to user retroactively
- No cookies, no localStorage for tracking

### 5. Astro for Frontend (Optional)

Consider migrating from SvelteKit to Astro.

**Benefits:**
- Zero JS by default for static pages
- Svelte components as islands for interactive parts
- Cleaner separation: Astro = rendering, Rust = logic

**If staying with SvelteKit:**
- Remove all API routes (move to Rust)
- Keep only page rendering and form submissions

---

## Data Flow

### Anonymous Session

```
1. User visits site
   Browser: sessionId = crypto.randomUUID() (in memory only)

2. User interacts (scroll, click editor, etc.)
   Browser: POST /api/events { session_id, event_name, properties }
   Server: Creates session record if not exists, inserts event

3. User leaves
   Session and events remain in DB, no user linked
```

### Identified Session (Signature)

```
1-2. Same as above

3. User creates signature
   Browser: POST /api/signatures { session_id, name, signature_data }
   Server:
     - Create user record
     - Link session to user
     - Create signature linked to session
     - Send confirmation email

4. Full journey now queryable:
   User ← Session ← Events
                 ← Signature
```

### Identified Session (Contact Form)

```
1-2. Same as anonymous

3. User submits contact form
   Browser: POST /api/contact { session_id, email, message, turnstile_token }
   Server:
     - Validate CAPTCHA
     - Create user with email (or update if session already has user)
     - Link session to user
     - Send email via Resend
```

---

## Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Reverse Proxy | Caddy | HTTPS, routing |
| Frontend | Astro + Svelte | Static pages + interactive islands |
| Backend | Rust + Axum | API, business logic |
| Database | SQLite (WAL) | Persistence |
| Cache | In-memory (Rust) | Fast signature reads |
| Email | Resend API | Transactional emails |
| CAPTCHA | Cloudflare Turnstile | Bot protection |
| Hosting | Hetzner VPS | German data residency |
| Containerization | Docker Compose | Deployment |

---

## File Structure (Target)

```
www/
├── frontend/                    # Astro + Svelte
│   ├── src/
│   │   ├── pages/
│   │   ├── components/
│   │   └── layouts/
│   ├── astro.config.mjs
│   └── Dockerfile
│
├── backend/                     # Rust API
│   ├── src/
│   │   ├── main.rs
│   │   ├── db/
│   │   ├── routes/
│   │   ├── models/
│   │   └── services/
│   ├── migrations/
│   ├── Cargo.toml
│   └── Dockerfile
│
├── docker-compose.yml
├── Caddyfile
└── docs/
    └── rewrite/                 # This documentation
```
