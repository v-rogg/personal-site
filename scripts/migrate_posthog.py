#!/usr/bin/env python3
"""
Migrate PostHog events and pageviews to local SQLite database.
This script fetches the last 365 days of data from PostHog via HogQL queries.
"""

import json
import sqlite3
import time
from datetime import datetime, timezone
from pathlib import Path

import requests

# PostHog configuration
POSTHOG_API_KEY = "phx_wmUciamAGWG5jfX5U9VrEohk6X5wpxzsPxjc7vZeRbt22Up"
POSTHOG_PROJECT_ID = "35470"
POSTHOG_API_URL = (
    f"https://eu.i.posthog.com/api/projects/{POSTHOG_PROJECT_ID}/query/"
)

# Database path
DB_PATH = Path(__file__).parent.parent / "data" / "app.db"


def execute_posthog_query(query: str, limit: int = 10000) -> dict:
    """Execute a HogQL query against PostHog API."""
    headers = {
        "Authorization": f"Bearer {POSTHOG_API_KEY}",
        "Content-Type": "application/json",
    }
    # Add LIMIT to query if not present
    if "LIMIT" not in query.upper():
        query = query.rstrip().rstrip(";") + f" LIMIT {limit}"

    payload = {"query": {"kind": "HogQLQuery", "query": query}}

    response = requests.post(POSTHOG_API_URL, headers=headers, json=payload)
    response.raise_for_status()
    return response.json()


def fetch_events(days: int = 365) -> list:
    """Fetch all click events from the last N days."""
    print(f"Fetching events from the last {days} days...")

    query = f"""
    SELECT
        properties.$session_id AS session_id,
        event AS event_name,
        toUnixTimestamp(timestamp) AS ts,
        properties.$geoip_country_code AS country_code,
        properties.$geoip_subdivision_1_name AS region
    FROM events
    WHERE
        timestamp >= now() - INTERVAL {days} DAY
        AND event LIKE 'click.%'
        AND properties.$session_id IS NOT NULL
        AND properties.$host NOT LIKE 'localhost%'
        AND properties.$host NOT LIKE '127.0.0.1%'
    ORDER BY timestamp ASC
    """

    result = execute_posthog_query(query)
    events = []

    if "results" in result:
        for row in result["results"]:
            if len(row) >= 5:
                events.append(
                    {
                        "session_id": row[0],
                        "event_name": row[1],
                        "timestamp": int(row[2]) if row[2] else None,
                        "properties": None,
                        "country_code": row[3],
                        "region": row[4],
                    }
                )

    print(f"  Found {len(events)} events")
    return events


def fetch_pageviews(days: int = 365) -> list:
    """Fetch all pageviews from the last N days."""
    print(f"Fetching pageviews from the last {days} days...")

    query = f"""
    SELECT
        properties.$session_id AS session_id,
        properties.$pathname AS path,
        properties.$referrer AS referrer,
        toUnixTimestamp(timestamp) AS ts,
        properties.$geoip_country_code AS country_code,
        properties.$geoip_subdivision_1_name AS region
    FROM events
    WHERE
        timestamp >= now() - INTERVAL {days} DAY
        AND event = '$pageview'
        AND properties.$session_id IS NOT NULL
        AND properties.$host NOT LIKE 'localhost%'
        AND properties.$host NOT LIKE '127.0.0.1%'
    ORDER BY timestamp ASC
    """

    result = execute_posthog_query(query)
    pageviews = []

    if "results" in result:
        for row in result["results"]:
            if len(row) >= 6:
                referrer = row[2]
                # Filter out internal referrers
                if referrer and (
                    "valentinrogg.de" in referrer or "localhost" in referrer
                ):
                    referrer = None

                pageviews.append(
                    {
                        "session_id": row[0],
                        "path": row[1] or "/",
                        "referrer": referrer,
                        "timestamp": int(row[3]) if row[3] else None,
                        "country_code": row[4],
                        "region": row[5],
                    }
                )

    print(f"  Found {len(pageviews)} pageviews")
    return pageviews


def fetch_pageleaves(days: int = 365) -> list:
    """Fetch all pageleave events to calculate durations."""
    print(f"Fetching pageleave events from the last {days} days...")

    query = f"""
    SELECT
        properties.$session_id AS session_id,
        properties.$pathname AS path,
        toUnixTimestamp(timestamp) AS ts,
        properties.$prev_pageview_max_scroll AS max_scroll
    FROM events
    WHERE
        timestamp >= now() - INTERVAL {days} DAY
        AND event = '$pageleave'
        AND properties.$session_id IS NOT NULL
        AND properties.$host NOT LIKE 'localhost%'
        AND properties.$host NOT LIKE '127.0.0.1%'
    ORDER BY timestamp ASC
    """

    result = execute_posthog_query(query)
    pageleaves = []

    if "results" in result:
        for row in result["results"]:
            if len(row) >= 4:
                pageleaves.append(
                    {
                        "session_id": row[0],
                        "path": row[1] or "/",
                        "timestamp": int(row[2]) if row[2] else None,
                        "max_scroll": int(row[3]) if row[3] else 0,
                    }
                )

    print(f"  Found {len(pageleaves)} pageleave events")
    return pageleaves


def import_to_database(events: list, pageviews: list, pageleaves: list):
    """Import fetched data into SQLite database."""
    print(f"\nImporting data to {DB_PATH}...")

    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()

    # Collect all unique sessions with their geo data
    sessions = {}

    for event in events:
        sid = event["session_id"]
        if sid and sid not in sessions:
            sessions[sid] = {
                "country_code": event.get("country_code"),
                "region": event.get("region"),
                "ts_created": event["timestamp"],
                "ts_last_seen": event["timestamp"],
            }
        elif sid:
            sessions[sid]["ts_last_seen"] = max(
                sessions[sid]["ts_last_seen"], event["timestamp"]
            )

    for pv in pageviews:
        sid = pv["session_id"]
        if sid and sid not in sessions:
            sessions[sid] = {
                "country_code": pv.get("country_code"),
                "region": pv.get("region"),
                "ts_created": pv["timestamp"],
                "ts_last_seen": pv["timestamp"],
            }
        elif sid:
            if pv["timestamp"]:
                sessions[sid]["ts_last_seen"] = max(
                    sessions[sid]["ts_last_seen"], pv["timestamp"]
                )
                sessions[sid]["ts_created"] = min(
                    sessions[sid]["ts_created"], pv["timestamp"]
                )

    # Insert sessions
    print(f"  Inserting {len(sessions)} sessions...")
    sessions_inserted = 0
    for session_id, data in sessions.items():
        try:
            cursor.execute(
                """
                INSERT INTO sessions (id, country_code, region, ts_created, ts_last_seen)
                VALUES (?, ?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET
                    country_code = COALESCE(excluded.country_code, sessions.country_code),
                    region = COALESCE(excluded.region, sessions.region),
                    ts_created = MIN(excluded.ts_created, sessions.ts_created),
                    ts_last_seen = MAX(excluded.ts_last_seen, sessions.ts_last_seen)
            """,
                (
                    session_id,
                    data["country_code"],
                    data["region"],
                    data["ts_created"],
                    data["ts_last_seen"],
                ),
            )
            sessions_inserted += 1
        except Exception as e:
            print(f"    Error inserting session {session_id}: {e}")

    print(f"    Inserted/updated {sessions_inserted} sessions")

    # Insert events
    print(f"  Inserting {len(events)} events...")
    events_inserted = 0
    for event in events:
        if event["session_id"] and event["timestamp"]:
            try:
                cursor.execute(
                    """
                    INSERT INTO events (session_id, event_name, timestamp, properties)
                    VALUES (?, ?, ?, ?)
                """,
                    (
                        event["session_id"],
                        event["event_name"],
                        event["timestamp"],
                        event["properties"],
                    ),
                )
                events_inserted += 1
            except Exception as e:
                print(f"    Error inserting event: {e}")

    print(f"    Inserted {events_inserted} events")

    # Build pageleave lookup for matching with pageviews
    pageleave_lookup = {}
    for pl in pageleaves:
        key = (pl["session_id"], pl["path"])
        if key not in pageleave_lookup:
            pageleave_lookup[key] = []
        pageleave_lookup[key].append(pl)

    # Insert pageviews with duration calculation
    print(f"  Inserting {len(pageviews)} pageviews...")
    pageviews_inserted = 0
    for pv in pageviews:
        if pv["session_id"] and pv["timestamp"]:
            # Try to find matching pageleave
            key = (pv["session_id"], pv["path"])
            ts_end = None
            duration = None
            max_scroll = 0

            if key in pageleave_lookup:
                # Find the first pageleave after this pageview
                for pl in pageleave_lookup[key]:
                    if pl["timestamp"] and pl["timestamp"] > pv["timestamp"]:
                        ts_end = pl["timestamp"]
                        duration = ts_end - pv["timestamp"]
                        max_scroll = pl.get("max_scroll", 0)
                        break

            try:
                cursor.execute(
                    """
                    INSERT INTO page_views (session_id, path, referrer, ts_start, ts_end, duration_seconds, max_scroll_percent)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                """,
                    (
                        pv["session_id"],
                        pv["path"],
                        pv["referrer"],
                        pv["timestamp"],
                        ts_end,
                        duration,
                        max_scroll,
                    ),
                )
                pageviews_inserted += 1
            except Exception as e:
                print(f"    Error inserting pageview: {e}")

    print(f"    Inserted {pageviews_inserted} pageviews")

    conn.commit()
    conn.close()

    print("\nMigration complete!")


def print_stats():
    """Print current database statistics."""
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()

    print("\n=== Database Statistics ===")

    cursor.execute("SELECT COUNT(*) FROM sessions")
    print(f"Sessions: {cursor.fetchone()[0]}")

    cursor.execute("SELECT COUNT(*) FROM events")
    print(f"Events: {cursor.fetchone()[0]}")

    cursor.execute("SELECT COUNT(*) FROM page_views")
    print(f"Page Views: {cursor.fetchone()[0]}")

    cursor.execute(
        "SELECT event_name, COUNT(*) FROM events GROUP BY event_name ORDER BY COUNT(*) DESC LIMIT 10"
    )
    print("\nTop 10 Event Types:")
    for row in cursor.fetchall():
        print(f"  {row[0]}: {row[1]}")

    cursor.execute(
        "SELECT path, COUNT(*) FROM page_views GROUP BY path ORDER BY COUNT(*) DESC LIMIT 10"
    )
    print("\nTop 10 Pages:")
    for row in cursor.fetchall():
        print(f"  {row[0]}: {row[1]}")

    conn.close()


if __name__ == "__main__":
    print("PostHog to SQLite Migration Script")
    print("=" * 40)

    # Fetch data from PostHog
    events = fetch_events(365)
    pageviews = fetch_pageviews(365)
    pageleaves = fetch_pageleaves(365)

    # Import to database
    import_to_database(events, pageviews, pageleaves)

    # Show stats
    print_stats()
