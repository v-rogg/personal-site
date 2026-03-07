#!/usr/bin/env python3
"""
Import PostHog users and link them to existing signatures.
"""

import json
import os
import sqlite3
from datetime import datetime

POSTHOG_EXPORT = os.path.join(
    os.path.dirname(__file__), "..", "posthog_persons.json"
)
SQLITE_DB = os.path.join(os.path.dirname(__file__), "..", "data", "app.db")


def parse_iso_timestamp(ts_str):
    """Convert ISO timestamp string to Unix timestamp."""
    if not ts_str:
        return int(datetime.now().timestamp())
    try:
        # Handle the PostHog format: "2025-10-10T12:34:37.429000Z"
        dt = datetime.fromisoformat(ts_str.replace("Z", "+00:00"))
        return int(dt.timestamp())
    except:
        return int(datetime.now().timestamp())


def import_users(conn, data):
    """Import users from PostHog export."""
    cursor = conn.cursor()

    results = data.get("results", [])

    users_created = 0
    sessions_created = 0
    signatures_linked = 0

    for person in results:
        props = person.get("properties", {})

        # Get email if available
        email = props.get("email", "")
        if email == "":
            email = None

        # Get signature ID if this user created a signature
        signature_id = props.get("id")  # This is the signature ID

        # Get geo data
        country_code = props.get("$geoip_country_code")
        region = props.get("$geoip_subdivision_1_name")

        # Get timestamps
        created_at = parse_iso_timestamp(person.get("created_at"))

        # Get the first distinct_id as the session ID
        distinct_ids = person.get("distinct_ids", [])
        session_id = distinct_ids[0] if distinct_ids else None

        if not session_id:
            continue

        # Create user if we have an email
        user_id = None
        if email:
            try:
                cursor.execute(
                    """
                    INSERT OR IGNORE INTO users (email, ts_created)
                    VALUES (?, ?)
                """,
                    (email, created_at),
                )

                if cursor.rowcount > 0:
                    users_created += 1

                cursor.execute("SELECT id FROM users WHERE email = ?", (email,))
                row = cursor.fetchone()
                if row:
                    user_id = row[0]
            except Exception as e:
                print(f"Error creating user {email}: {e}")

        # Create session
        try:
            cursor.execute(
                """
                INSERT OR REPLACE INTO sessions (id, user_id, country_code, region, ts_created, ts_last_seen)
                VALUES (?, ?, ?, ?, ?, ?)
            """,
                (
                    session_id,
                    user_id,
                    country_code,
                    region,
                    created_at,
                    created_at,
                ),
            )
            sessions_created += 1
        except Exception as e:
            print(f"Error creating session {session_id}: {e}")
            continue

        # Link signature if this user created one
        if signature_id:
            try:
                cursor.execute(
                    """
                    UPDATE signatures SET session_id = ? WHERE id = ?
                """,
                    (session_id, signature_id),
                )

                if cursor.rowcount > 0:
                    signatures_linked += 1
                    print(
                        f"  Linked signature {signature_id} to session {session_id}"
                    )
            except Exception as e:
                print(f"Error linking signature {signature_id}: {e}")

    conn.commit()
    return users_created, sessions_created, signatures_linked


def main():
    # Read PostHog export
    print(f"Reading PostHog export from {POSTHOG_EXPORT}...")
    with open(POSTHOG_EXPORT, "r") as f:
        data = json.load(f)

    print(f"Found {len(data.get('results', []))} persons")

    # Connect to SQLite
    print(f"Connecting to SQLite database at {SQLITE_DB}...")
    conn = sqlite3.connect(SQLITE_DB)

    try:
        # Import users
        print("Importing users and sessions...")
        users_created, sessions_created, signatures_linked = import_users(
            conn, data
        )

        print(f"\nImport complete!")
        print(f"  Users created:      {users_created}")
        print(f"  Sessions created:   {sessions_created}")
        print(f"  Signatures linked:  {signatures_linked}")

        # Verify
        cursor = conn.cursor()
        cursor.execute("SELECT COUNT(*) FROM users")
        total_users = cursor.fetchone()[0]
        cursor.execute("SELECT COUNT(*) FROM sessions")
        total_sessions = cursor.fetchone()[0]
        cursor.execute(
            "SELECT COUNT(*) FROM signatures WHERE session_id IS NOT NULL"
        )
        linked_sigs = cursor.fetchone()[0]

        print(f"\nDatabase stats:")
        print(f"  Total users:             {total_users}")
        print(f"  Total sessions:          {total_sessions}")
        print(f"  Signatures with session: {linked_sigs}")

    finally:
        conn.close()


if __name__ == "__main__":
    main()
