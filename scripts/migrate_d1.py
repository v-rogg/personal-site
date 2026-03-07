#!/usr/bin/env python3
"""
Migration script to import D1 signatures into local SQLite database.
"""

import json
import os
import sqlite3
from datetime import datetime

# Paths
D1_EXPORT = os.path.join(os.path.dirname(__file__), "..", "d1_export.json")
SQLITE_DB = os.path.join(os.path.dirname(__file__), "..", "data", "app.db")


def create_database(conn):
    """Schema already created by SQL migration, this is a no-op."""
    pass


def migrate_signatures(conn, data):
    """Import signatures from D1 export."""
    cursor = conn.cursor()

    # The D1 export is wrapped in a results array
    signatures = (
        data[0]["results"]
        if isinstance(data, list) and len(data) > 0 and "results" in data[0]
        else data
    )

    imported = 0
    skipped = 0

    for sig in signatures:
        try:
            # Map D1 fields to new schema
            sig_id = sig["id"]
            name = sig["name"]
            signature = sig["signature"]
            email = sig.get("email")
            approved = sig.get("approved", 0)

            # Convert timestamp format (D1 uses "YYYY-MM-DD HH:MM:SS.ms")
            # New schema expects Unix timestamp (INTEGER)
            now = int(datetime.now().timestamp())

            def parse_timestamp(ts, fallback):
                if not ts:
                    return fallback
                if " " in str(ts):
                    try:
                        dt = datetime.strptime(
                            str(ts).split(".")[0], "%Y-%m-%d %H:%M:%S"
                        )
                        return int(dt.timestamp())
                    except:
                        return fallback
                return fallback

            ts_created = parse_timestamp(sig.get("ts_created"), now)
            ts_modified = parse_timestamp(sig.get("ts_modified"), ts_created)

            # Insert into new database (schema uses ts_created/ts_modified as INTEGER)
            cursor.execute(
                """
                INSERT OR REPLACE INTO signatures (id, session_id, name, signature, approved, ts_created, ts_modified)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    sig_id,
                    None,  # No session_id for migrated data
                    name,
                    signature,
                    approved,
                    ts_created,
                    ts_modified,
                ),
            )

            imported += 1

        except Exception as e:
            print(f"Error importing signature {sig.get('id', 'unknown')}: {e}")
            skipped += 1

    conn.commit()
    return imported, skipped


def main():
    # Ensure data directory exists
    os.makedirs(os.path.dirname(SQLITE_DB), exist_ok=True)

    # Read D1 export
    print(f"Reading D1 export from {D1_EXPORT}...")
    with open(D1_EXPORT, "r") as f:
        data = json.load(f)

    # Connect to SQLite
    print(f"Connecting to SQLite database at {SQLITE_DB}...")
    conn = sqlite3.connect(SQLITE_DB)

    try:
        # Create table
        print("Creating signatures table...")
        create_database(conn)

        # Import signatures
        print("Importing signatures...")
        imported, skipped = migrate_signatures(conn, data)

        print(f"\nMigration complete!")
        print(f"  Imported: {imported}")
        print(f"  Skipped:  {skipped}")

        # Verify
        cursor = conn.cursor()
        cursor.execute("SELECT COUNT(*) FROM signatures")
        total = cursor.fetchone()[0]
        cursor.execute("SELECT COUNT(*) FROM signatures WHERE approved = 1")
        approved = cursor.fetchone()[0]
        print(f"\nDatabase stats:")
        print(f"  Total signatures: {total}")
        print(f"  Approved:         {approved}")

    finally:
        conn.close()


if __name__ == "__main__":
    main()
