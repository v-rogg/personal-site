# PostHog Analytics Worker - Timeframe Support

This Cloudflare Worker provides analytics endpoints for querying PostHog data with dynamic timeframe support.

## Features

- **Dynamic Time Ranges**: All endpoints now support flexible timeframe parameters
- **Multiple Endpoints**: Conversion rates, painting durations, eraser usage, and drawing time analytics
- **Authentication**: Bearer token authentication for secure access
- **Health Check**: Built-in health monitoring endpoint

## Timeframe Parameter

All analytics endpoints now accept a `timeframe` query parameter that allows you to specify the time range for data queries.

### Supported Formats

| Format | Description | Example | SQL Equivalent |
|--------|-------------|---------|----------------|
| `all` | All data since beginning of recording | `?timeframe=all` | `1 = 1` (no filter) |
| `dx` | Last X days | `?timeframe=d30` | `timestamp >= now() - INTERVAL 30 DAY` |
| `mx` | Last X months | `?timeframe=m6` | `timestamp >= now() - INTERVAL 6 MONTH` |
| `qx` | Last X quarters | `?timeframe=q2` | `timestamp >= now() - INTERVAL 6 MONTH` |
| `yx` | Last X years | `?timeframe=y1` | `timestamp >= now() - INTERVAL 1 YEAR` |

### Default Values

Each endpoint has sensible defaults when no timeframe is specified:

- **Conversion Rate**: `d180` (180 days)
- **Painting Durations**: `d180` (180 days)
- **Eraser Uses**: `d180` (180 days)
- **Drawing Time**: `d180` (180 days)

## Endpoints

### GET `/health`
Health check endpoint to verify PostHog API connectivity.

**Response:**
```json
{
  "status": "healthy",
  "posthog_api": "online"
}
```

### GET `/conversion-rate`
Get editor conversion statistics (open → save rate).

**Query Parameters:**
- `timeframe` (optional): Time range for data (default: `d180`)

**Example:**
```bash
curl "https://your-worker.workers.dev/conversion-rate?timeframe=m3" \
  -H "Authorization: Bearer your-secret-key"
```

**Response:**
```json
{
  "conversion_step_1": 150,
  "conversion_step_2": 45,
  "conversion_rate": 30.0
}
```

### GET `/painting-durations`
Get painting session durations with user IDs.

**Query Parameters:**
- `timeframe` (optional): Time range for data (default: `d180`)

**Example:**
```bash
curl "https://your-worker.workers.dev/painting-durations?timeframe=y1" \
  -H "Authorization: Bearer your-secret-key"
```

**Response:**
```json
{
  "painting_durations": [
    {
      "id": "user123",
      "duration_seconds": 245.5,
      "ts_created": 1673456789
    }
  ],
  "has_data": true
}
```

### GET `/eraser-uses`
Get average eraser usage per session.

**Query Parameters:**
- `timeframe` (optional): Time range for data (default: `d180`)

**Example:**
```bash
curl "https://your-worker.workers.dev/eraser-uses?timeframe=all" \
  -H "Authorization: Bearer your-secret-key"
```

**Response:**
```json
{
  "average_eraser_uses": 2.3
}
```

### GET `/drawing-time`
Get average drawing time per signature.

**Query Parameters:**
- `timeframe` (optional): Time range for data (default: `d180`)

**Example:**
```bash
curl "https://your-worker.workers.dev/drawing-time?timeframe=q1" \
  -H "Authorization: Bearer your-secret-key"
```

**Response:**
```json
{
  "average_drawing_time_seconds": 127.8
}
```

## Authentication

All endpoints (except `/health`) require Bearer token authentication:

```bash
curl "https://your-worker.workers.dev/endpoint" \
  -H "Authorization: Bearer your-secret-key"
```

## Environment Variables

The worker requires the following environment variables:

- `SECRET_KEY`: Bearer token for API authentication
- `POSTHOG_PROJECT_ID`: Your PostHog project ID
- `POSTHOG_API_KEY`: Your PostHog API key

## Frontend Integration

The frontend analytics component automatically uses the timeframe parameter:

```typescript
// Fetch stats with 180-day timeframe (default)
const stats = await getStats(platform, "d180");

// Fetch stats with 90-day timeframe
const stats = await getStats(platform, "d90");
```

The UI includes a timeframe selector that updates the URL parameter and refetches data accordingly.

## Error Handling

- Invalid timeframe formats fall back to default values
- Authentication failures return 401 status
- PostHog API errors are handled gracefully
- Malformed requests return appropriate error responses

## Performance Notes

- All database queries use indexed timestamp fields for optimal performance
- Complex funnel queries are optimized for PostHog's ClickHouse backend
- Results are computed in real-time (no caching currently implemented)

## Development

To test locally:

```bash
# Start the worker in development mode
wrangler dev

# Test an endpoint with timeframe
curl "http://localhost:8787/conversion-rate?timeframe=d7" \
  -H "Authorization: Bearer test-key"
```

## Deployment

Deploy to Cloudflare Workers:

```bash
wrangler deploy
```

Make sure all environment variables are configured in the Cloudflare Workers dashboard.
