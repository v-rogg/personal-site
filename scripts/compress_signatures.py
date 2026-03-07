#!/usr/bin/env python3
"""
Compress all uncompressed signatures in the database to gzip+base64 format.
"""

import base64
import gzip
import json
import sqlite3

DB_PATH = "../data/app.db"


def is_compressed(data: str) -> bool:
    """Check if data is already gzip+base64 compressed."""
    try:
        decoded = base64.b64decode(data)
        # Check for gzip magic number
        return decoded[:2] == b"\x1f\x8b"
    except:
        return False


def compress_signature(data: str) -> str:
    """Compress signature data to gzip+base64."""
    compressed = gzip.compress(data.encode("utf-8"))
    return base64.b64encode(compressed).decode("utf-8")


def main():
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()

    # Get all signatures
    cursor.execute("SELECT id, signature FROM signatures")
    signatures = cursor.fetchall()

    compressed_count = 0
    already_compressed = 0

    for sig_id, signature in signatures:
        if is_compressed(signature):
            already_compressed += 1
            continue

        # Compress the signature
        compressed = compress_signature(signature)

        # Update in database
        cursor.execute(
            "UPDATE signatures SET signature = ? WHERE id = ?",
            (compressed, sig_id),
        )
        compressed_count += 1
        print(f"Compressed: {sig_id}")

    conn.commit()
    conn.close()

    print(f"\nDone!")
    print(f"  Compressed: {compressed_count}")
    print(f"  Already compressed: {already_compressed}")
    print(f"  Total: {len(signatures)}")


if __name__ == "__main__":
    main()
