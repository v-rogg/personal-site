# Frontend Migration Guide

## Overview

Two options for the frontend:

1. **Keep SvelteKit** - Remove API routes, keep only rendering
2. **Migrate to Astro** - Static pages + Svelte islands for interactivity

This document covers both approaches.

---

## Option A: Keep SvelteKit (Minimal Changes)

### Files to Remove

These files contain logic that moves to the Rust backend:

```
src/routes/api/                      # All API routes → Rust
├── analytics/+server.ts             # → GET /api/stats
├── signatures/
│   ├── [id]/+server.ts              # → GET /api/signatures/:id
│   └── clear/+server.ts             # → Remove (cache in Rust now)
└── actions/
    ├── request/+page.server.ts      # → POST /api/contact
    └── signatures/+page.server.ts   # → POST /api/signatures

src/lib/d1.ts                        # → Remove (direct API calls)
src/lib/components/Analytics/analytics.ts  # → Simplify (single API call)
```

### Files to Modify

#### src/routes/(landing)/+page.server.ts

**Before:** Fetches from workers, processes blog posts
**After:** Only blog processing, data fetched client-side or via single API call

```typescript
// Keep: Blog metadata loading (build-time)
// Remove: getSignatures(), getStats() calls
// Move signature/analytics fetching to client-side or layout

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
    // Blog loading stays the same
    const modules = import.meta.glob('/src/content/blog/*.mdx', { eager: true });
    
    const posts = Object.entries(modules)
        .map(([path, module]) => ({
            slug: path.split('/').pop()?.replace('.mdx', ''),
            ...module.metadata
        }))
        .filter(post => post.published)
        .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    
    return { posts };
};
```

#### src/lib/components/Analytics/analytics.ts

**Before:** 4 parallel calls to PostHog worker
**After:** Single call to Rust API

```typescript
import type { AnalyticsStats } from './types';

const API_BASE = import.meta.env.VITE_API_URL || '';

export async function getStats(days: number = 180): Promise<AnalyticsStats> {
    try {
        const response = await fetch(`${API_BASE}/api/stats?days=${days}`);
        if (!response.ok) throw new Error('Failed to fetch stats');
        return await response.json();
    } catch (error) {
        console.error('Analytics fetch error:', error);
        return {
            conversion_step_1: 0,
            conversion_step_2: 0,
            conversion_rate: 0,
            drawing_durations: [],
            has_drawing_data: false,
            average_eraser_uses: 0,
            average_drawing_time_seconds: 0,
        };
    }
}
```

#### src/lib/d1.ts → src/lib/api.ts

Replace D1 wrapper with direct API calls:

```typescript
const API_BASE = import.meta.env.VITE_API_URL || '';

export interface Signature {
    id: string;
    name: string;
    signature: string;
    ts_created: number;
}

export async function getSignatures(): Promise<{ id: string; name: string }[]> {
    const response = await fetch(`${API_BASE}/api/signatures`);
    if (!response.ok) throw new Error('Failed to fetch signatures');
    const data = await response.json();
    return data.signatures;
}

export async function getSignature(id: string): Promise<Signature | null> {
    const response = await fetch(`${API_BASE}/api/signatures/${id}`);
    if (response.status === 404) return null;
    if (!response.ok) throw new Error('Failed to fetch signature');
    return await response.json();
}

export async function createSignature(data: {
    session_id: string;
    name: string;
    email?: string;
    signature: string;
}): Promise<{ id: string; url: string }> {
    const response = await fetch(`${API_BASE}/api/signatures`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
    });
    if (!response.ok) throw new Error('Failed to create signature');
    return await response.json();
}
```

#### Session ID Management

Create a new utility for session management:

```typescript
// src/lib/session.ts

let sessionId: string | null = null;

export function getSessionId(): string {
    if (!sessionId) {
        sessionId = crypto.randomUUID();
    }
    return sessionId;
}

// Call this on page load to create session
export async function initSession(): Promise<void> {
    const API_BASE = import.meta.env.VITE_API_URL || '';
    
    await fetch(`${API_BASE}/api/sessions`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ session_id: getSessionId() }),
    });
}
```

#### Event Tracking

Replace PostHog calls:

```typescript
// src/lib/tracking.ts

import { getSessionId } from './session';

const API_BASE = import.meta.env.VITE_API_URL || '';

export async function trackEvent(
    eventName: string,
    properties?: Record<string, unknown>
): Promise<void> {
    try {
        await fetch(`${API_BASE}/api/events`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                session_id: getSessionId(),
                event_name: eventName,
                properties,
            }),
        });
    } catch (error) {
        console.error('Event tracking error:', error);
    }
}
```

#### Update Signature Editor Component

```typescript
// In SignatureEditor.svelte or similar

import { trackEvent } from '$lib/tracking';
import { getSessionId } from '$lib/session';
import { createSignature } from '$lib/api';

// On editor open
function onEditorOpen() {
    trackEvent('editor.open');
}

// On eraser use
function onEraserClick() {
    trackEvent('editor.eraser');
}

// On save
async function onSave(name: string, email: string, signatureData: string) {
    trackEvent('editor.save');
    
    const result = await createSignature({
        session_id: getSessionId(),
        name,
        email: email || undefined,
        signature: signatureData,
    });
    
    // Redirect or show success
    window.location.href = result.url;
}
```

#### Update Contact Form

```typescript
// In contact form component

import { getSessionId } from '$lib/session';

const API_BASE = import.meta.env.VITE_API_URL || '';

async function submitContactForm(
    email: string,
    message: string,
    turnstileToken: string
): Promise<void> {
    const response = await fetch(`${API_BASE}/api/contact`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            session_id: getSessionId(),
            email,
            message,
            turnstile_token: turnstileToken,
        }),
    });
    
    if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || 'Failed to send message');
    }
}
```

### Environment Variables

```bash
# .env
VITE_API_URL=https://api.valentinrogg.de
# or for same-domain setup:
VITE_API_URL=
```

### Build Configuration

Update `svelte.config.js` for static adapter or Node adapter:

```javascript
import adapter from '@sveltejs/adapter-node';
// or
import adapter from '@sveltejs/adapter-static';

export default {
    kit: {
        adapter: adapter({
            // Node adapter for SSR
            out: 'build',
        }),
        // or for static
        adapter: adapter({
            pages: 'build',
            assets: 'build',
            fallback: 'index.html',
        }),
    },
};
```

---

## Option B: Migrate to Astro

### Benefits

- Zero JS by default for static pages
- Keep Svelte for interactive components (islands)
- Better performance for content pages
- Cleaner separation of concerns

### Project Structure

```
frontend/
├── astro.config.mjs
├── package.json
├── src/
│   ├── pages/
│   │   ├── index.astro           # Landing page
│   │   ├── blog/
│   │   │   ├── index.astro       # Blog list
│   │   │   └── [slug].astro      # Blog post
│   │   └── api/                  # Remove - handled by Rust
│   ├── layouts/
│   │   └── BaseLayout.astro
│   ├── components/
│   │   ├── Header.astro          # Static header
│   │   ├── Footer.astro          # Static footer
│   │   ├── BlogCard.astro        # Static blog card
│   │   └── svelte/               # Interactive islands
│   │       ├── SignatureCarousel.svelte
│   │       ├── SignatureEditor.svelte
│   │       ├── ContactForm.svelte
│   │       └── Analytics.svelte
│   ├── content/
│   │   └── blog/                 # MDX blog posts
│   ├── lib/
│   │   ├── api.ts                # API client
│   │   ├── session.ts            # Session management
│   │   └── tracking.ts           # Event tracking
│   └── styles/
│       └── global.css
└── public/
    └── ...
```

### Astro Configuration

```javascript
// astro.config.mjs
import { defineConfig } from 'astro/config';
import svelte from '@astrojs/svelte';
import mdx from '@astrojs/mdx';
import tailwind from '@astrojs/tailwind';

export default defineConfig({
    integrations: [
        svelte(),
        mdx(),
        tailwind(),
    ],
    output: 'static',  // or 'server' for SSR
});
```

### Landing Page Example

```astro
---
// src/pages/index.astro
import BaseLayout from '../layouts/BaseLayout.astro';
import SignatureCarousel from '../components/svelte/SignatureCarousel.svelte';
import SignatureEditor from '../components/svelte/SignatureEditor.svelte';
import Analytics from '../components/svelte/Analytics.svelte';
import { getCollection } from 'astro:content';

// Get blog posts at build time
const posts = await getCollection('blog');
const sortedPosts = posts
    .filter(post => post.data.published)
    .sort((a, b) => b.data.date.getTime() - a.data.date.getTime());
---

<BaseLayout title="Valentin Rogg">
    <main>
        <!-- Static content -->
        <section id="hero">
            <h1>Valentin Rogg</h1>
            <p>Software Engineer</p>
        </section>
        
        <!-- Interactive Svelte island -->
        <section id="signatures">
            <h2>Signatures</h2>
            <SignatureCarousel client:visible />
            <SignatureEditor client:visible />
        </section>
        
        <!-- Static blog list -->
        <section id="blog">
            <h2>Blog</h2>
            {sortedPosts.map(post => (
                <article>
                    <a href={`/blog/${post.slug}`}>{post.data.title}</a>
                </article>
            ))}
        </section>
        
        <!-- Interactive analytics (only loads when visible) -->
        <Analytics client:visible />
    </main>
</BaseLayout>
```

### Svelte Island Example

```svelte
<!-- src/components/svelte/SignatureCarousel.svelte -->
<script lang="ts">
    import { onMount } from 'svelte';
    import { getSignatures, getSignature } from '../../lib/api';
    import { initSession } from '../../lib/session';
    
    let signatures: { id: string; name: string }[] = [];
    let loading = true;
    
    onMount(async () => {
        await initSession();
        signatures = await getSignatures();
        loading = false;
    });
</script>

{#if loading}
    <div class="loading">Loading signatures...</div>
{:else}
    <div class="carousel">
        {#each signatures as sig}
            <div class="signature-card">
                {sig.name}
            </div>
        {/each}
    </div>
{/if}
```

### Client Directives

Astro's client directives control when Svelte components hydrate:

| Directive | When | Use For |
|-----------|------|---------|
| `client:load` | Page load | Critical interactive elements |
| `client:visible` | When visible | Below-fold content |
| `client:idle` | Browser idle | Non-critical enhancements |
| `client:only="svelte"` | Client only | No SSR needed |

Example:
```astro
<!-- Loads immediately -->
<SignatureEditor client:load />

<!-- Loads when scrolled into view -->
<SignatureCarousel client:visible />

<!-- Loads when browser is idle -->
<Analytics client:idle />
```

### Content Collections (Blog)

```typescript
// src/content/config.ts
import { defineCollection, z } from 'astro:content';

const blogCollection = defineCollection({
    type: 'content',
    schema: z.object({
        title: z.string(),
        date: z.date(),
        description: z.string(),
        published: z.boolean().default(false),
        tags: z.array(z.string()).default([]),
    }),
});

export const collections = {
    blog: blogCollection,
};
```

---

## Shared Changes (Both Options)

### Remove PostHog

1. Remove PostHog script from `app.html` or layout
2. Remove any PostHog SDK imports
3. Replace with custom `trackEvent()` calls

### Update Environment Variables

```bash
# Production
VITE_API_URL=https://api.valentinrogg.de

# Development
VITE_API_URL=http://localhost:8080
```

### CORS Configuration

Ensure Rust backend allows frontend origin:

```rust
// In Rust main.rs
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(["https://valentinrogg.de".parse().unwrap()])
    .allow_methods(Any)
    .allow_headers(Any);
```

Or for same-domain setup (frontend and API on same domain via Caddy), CORS isn't needed.

---

## Migration Checklist

### SvelteKit Path

- [ ] Remove `src/routes/api/` directory
- [ ] Remove `src/lib/d1.ts`
- [ ] Create `src/lib/api.ts` with direct API calls
- [ ] Create `src/lib/session.ts` for session management
- [ ] Create `src/lib/tracking.ts` for event tracking
- [ ] Update `src/lib/components/Analytics/analytics.ts`
- [ ] Update signature editor component
- [ ] Update contact form component
- [ ] Update `+page.server.ts` files
- [ ] Remove PostHog integration
- [ ] Update environment variables
- [ ] Update build configuration

### Astro Path

- [ ] Initialize new Astro project
- [ ] Configure Svelte integration
- [ ] Migrate layouts to Astro
- [ ] Migrate static pages to `.astro` files
- [ ] Move interactive components to `components/svelte/`
- [ ] Add appropriate `client:*` directives
- [ ] Set up content collections for blog
- [ ] Create API client utilities
- [ ] Create session management utilities
- [ ] Create event tracking utilities
- [ ] Update environment variables
- [ ] Test all interactive features

---

## Dockerfile (SvelteKit with Node Adapter)

```dockerfile
FROM node:20-alpine AS builder

WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:20-alpine

WORKDIR /app
COPY --from=builder /app/build ./build
COPY --from=builder /app/package*.json ./
RUN npm ci --omit=dev

ENV PORT=3000
EXPOSE 3000

CMD ["node", "build"]
```

## Dockerfile (Astro Static)

```dockerfile
FROM node:20-alpine AS builder

WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
```
