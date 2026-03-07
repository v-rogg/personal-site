# Secure SQLite Backup to Nextcloud

## Overview

Backup SQLite database to your Nextcloud server via WebDAV with encryption at rest.

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Hetzner VPS    │     │   Encrypted     │     │   Nextcloud     │
│                 │     │                 │     │                 │
│  SQLite DB      │────▶│  GPG Encrypt    │────▶│  WebDAV Upload  │
│                 │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

---

## Setup

### 1. Create Nextcloud App Password

1. Log into Nextcloud
2. Go to **Settings** → **Security** → **Devices & sessions**
3. Create new app password with name `vps-backup`
4. Save the generated password

### 2. Create Backup Directory in Nextcloud

1. Create folder: `/Backups/vr-www/`
2. Note your Nextcloud WebDAV URL:
   ```
   https://your-nextcloud.com/remote.php/dav/files/USERNAME/Backups/vr-www/
   ```

### 3. Install Dependencies on VPS

```bash
apt install gnupg curl sqlite3
```

### 4. Generate GPG Key for Encryption

```bash
# Generate key (as www user)
su - www
gpg --batch --gen-key <<EOF
Key-Type: RSA
Key-Length: 4096
Name-Real: VPS Backup
Name-Email: backup@valentinrogg.de
Expire-Date: 0
%no-protection
EOF

# Export public key (save this somewhere safe!)
gpg --export --armor "backup@valentinrogg.de" > ~/backup-public.key

# Export private key (KEEP THIS SAFE - needed for restore!)
gpg --export-secret-keys --armor "backup@valentinrogg.de" > ~/backup-private.key
```

**Important:** Store `backup-private.key` somewhere safe outside the VPS (e.g., password manager). Without it, you cannot restore backups.

### 5. Create Credentials File

```bash
# /opt/www/.backup-credentials (chmod 600)
NEXTCLOUD_URL="https://your-nextcloud.com/remote.php/dav/files/USERNAME/Backups/vr-www/"
NEXTCLOUD_USER="your-username"
NEXTCLOUD_PASS="app-password-from-step-1"
GPG_RECIPIENT="backup@valentinrogg.de"
```

```bash
chmod 600 /opt/www/.backup-credentials
chown www:www /opt/www/.backup-credentials
```

---

## Backup Script

```bash
#!/bin/bash
# /opt/www/backup.sh

set -euo pipefail

# Load credentials
source /opt/www/.backup-credentials

# Configuration
DB_CONTAINER="www-api-1"
DB_PATH="/data/app.db"
BACKUP_DIR="/opt/www/backups"
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="app_${DATE}.db"
ENCRYPTED_NAME="${BACKUP_NAME}.gpg"
RETENTION_DAYS=30

# Logging
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

log "Starting backup..."

# Create local backup directory
mkdir -p "$BACKUP_DIR"

# Checkpoint WAL for consistency
log "Checkpointing WAL..."
docker exec "$DB_CONTAINER" sqlite3 "$DB_PATH" "PRAGMA wal_checkpoint(TRUNCATE);" || true

# Create backup using SQLite backup API
log "Creating database backup..."
docker exec "$DB_CONTAINER" sqlite3 "$DB_PATH" ".backup '/data/${BACKUP_NAME}'"

# Move backup from container volume to host
docker cp "${DB_CONTAINER}:/data/${BACKUP_NAME}" "${BACKUP_DIR}/${BACKUP_NAME}"

# Remove backup from container
docker exec "$DB_CONTAINER" rm "/data/${BACKUP_NAME}"

# Verify backup integrity
log "Verifying backup integrity..."
if ! sqlite3 "${BACKUP_DIR}/${BACKUP_NAME}" "PRAGMA integrity_check;" | grep -q "ok"; then
    log "ERROR: Backup integrity check failed!"
    rm -f "${BACKUP_DIR}/${BACKUP_NAME}"
    exit 1
fi

# Encrypt backup
log "Encrypting backup..."
gpg --encrypt --recipient "$GPG_RECIPIENT" --output "${BACKUP_DIR}/${ENCRYPTED_NAME}" "${BACKUP_DIR}/${BACKUP_NAME}"

# Remove unencrypted backup
rm -f "${BACKUP_DIR}/${BACKUP_NAME}"

# Upload to Nextcloud
log "Uploading to Nextcloud..."
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" \
    -u "${NEXTCLOUD_USER}:${NEXTCLOUD_PASS}" \
    -T "${BACKUP_DIR}/${ENCRYPTED_NAME}" \
    "${NEXTCLOUD_URL}${ENCRYPTED_NAME}")

if [ "$HTTP_CODE" -eq 201 ] || [ "$HTTP_CODE" -eq 204 ]; then
    log "Upload successful (HTTP ${HTTP_CODE})"
else
    log "ERROR: Upload failed with HTTP ${HTTP_CODE}"
    exit 1
fi

# Clean up local encrypted backups older than retention period
log "Cleaning up old local backups..."
find "$BACKUP_DIR" -name "app_*.db.gpg" -mtime +${RETENTION_DAYS} -delete

# Clean up old Nextcloud backups (optional - keeps last 30)
log "Cleaning up old Nextcloud backups..."
# List files, sort by date, keep only filenames older than retention
OLD_BACKUPS=$(curl -s -u "${NEXTCLOUD_USER}:${NEXTCLOUD_PASS}" \
    -X PROPFIND "${NEXTCLOUD_URL}" \
    -H "Depth: 1" \
    | grep -oP 'app_\d{8}_\d{6}\.db\.gpg' \
    | sort \
    | head -n -${RETENTION_DAYS} || true)

for backup in $OLD_BACKUPS; do
    log "Deleting old backup: $backup"
    curl -s -u "${NEXTCLOUD_USER}:${NEXTCLOUD_PASS}" \
        -X DELETE "${NEXTCLOUD_URL}${backup}"
done

log "Backup completed: ${ENCRYPTED_NAME}"
```

```bash
chmod +x /opt/www/backup.sh
chown www:www /opt/www/backup.sh
```

---

## Restore Script

```bash
#!/bin/bash
# /opt/www/restore.sh

set -euo pipefail

# Load credentials
source /opt/www/.backup-credentials

BACKUP_DIR="/opt/www/backups"
DB_CONTAINER="www-api-1"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Check if backup name provided
if [ -z "${1:-}" ]; then
    echo "Usage: $0 <backup_name.db.gpg>"
    echo ""
    echo "Available backups on Nextcloud:"
    curl -s -u "${NEXTCLOUD_USER}:${NEXTCLOUD_PASS}" \
        -X PROPFIND "${NEXTCLOUD_URL}" \
        -H "Depth: 1" \
        | grep -oP 'app_\d{8}_\d{6}\.db\.gpg' \
        | sort -r \
        | head -20
    exit 1
fi

BACKUP_NAME="$1"
DECRYPTED_NAME="${BACKUP_NAME%.gpg}"

log "Downloading backup from Nextcloud..."
curl -s -u "${NEXTCLOUD_USER}:${NEXTCLOUD_PASS}" \
    -o "${BACKUP_DIR}/${BACKUP_NAME}" \
    "${NEXTCLOUD_URL}${BACKUP_NAME}"

if [ ! -f "${BACKUP_DIR}/${BACKUP_NAME}" ]; then
    log "ERROR: Failed to download backup"
    exit 1
fi

log "Decrypting backup..."
gpg --decrypt --output "${BACKUP_DIR}/${DECRYPTED_NAME}" "${BACKUP_DIR}/${BACKUP_NAME}"

log "Verifying backup integrity..."
if ! sqlite3 "${BACKUP_DIR}/${DECRYPTED_NAME}" "PRAGMA integrity_check;" | grep -q "ok"; then
    log "ERROR: Backup integrity check failed!"
    rm -f "${BACKUP_DIR}/${DECRYPTED_NAME}" "${BACKUP_DIR}/${BACKUP_NAME}"
    exit 1
fi

log "Stopping API container..."
docker stop "$DB_CONTAINER"

log "Restoring database..."
docker cp "${BACKUP_DIR}/${DECRYPTED_NAME}" "${DB_CONTAINER}:/data/app.db"

log "Starting API container..."
docker start "$DB_CONTAINER"

log "Cleaning up..."
rm -f "${BACKUP_DIR}/${DECRYPTED_NAME}" "${BACKUP_DIR}/${BACKUP_NAME}"

log "Restore completed successfully!"
```

```bash
chmod +x /opt/www/restore.sh
chown www:www /opt/www/restore.sh
```

---

## Cron Schedule

```bash
# /etc/cron.d/www-backup

# Daily backup at 3:00 AM
0 3 * * * www /opt/www/backup.sh >> /var/log/www-backup.log 2>&1

# Weekly integrity check on Sundays at 4:00 AM
0 4 * * 0 www /opt/www/verify-backups.sh >> /var/log/www-backup.log 2>&1
```

---

## Verification Script

```bash
#!/bin/bash
# /opt/www/verify-backups.sh

set -euo pipefail

source /opt/www/.backup-credentials

BACKUP_DIR="/opt/www/backups"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

log "Starting weekly backup verification..."

# Get latest backup from Nextcloud
LATEST=$(curl -s -u "${NEXTCLOUD_USER}:${NEXTCLOUD_PASS}" \
    -X PROPFIND "${NEXTCLOUD_URL}" \
    -H "Depth: 1" \
    | grep -oP 'app_\d{8}_\d{6}\.db\.gpg' \
    | sort -r \
    | head -1)

if [ -z "$LATEST" ]; then
    log "ERROR: No backups found on Nextcloud!"
    exit 1
fi

log "Verifying latest backup: $LATEST"

# Download
curl -s -u "${NEXTCLOUD_USER}:${NEXTCLOUD_PASS}" \
    -o "${BACKUP_DIR}/verify_${LATEST}" \
    "${NEXTCLOUD_URL}${LATEST}"

# Decrypt
gpg --decrypt --output "${BACKUP_DIR}/verify_${LATEST%.gpg}" "${BACKUP_DIR}/verify_${LATEST}"

# Verify integrity
if sqlite3 "${BACKUP_DIR}/verify_${LATEST%.gpg}" "PRAGMA integrity_check;" | grep -q "ok"; then
    log "Backup verification PASSED"
    
    # Log some stats
    TABLES=$(sqlite3 "${BACKUP_DIR}/verify_${LATEST%.gpg}" ".tables")
    log "Tables: $TABLES"
    
    SIG_COUNT=$(sqlite3 "${BACKUP_DIR}/verify_${LATEST%.gpg}" "SELECT COUNT(*) FROM signatures;")
    log "Signatures: $SIG_COUNT"
    
    EVENT_COUNT=$(sqlite3 "${BACKUP_DIR}/verify_${LATEST%.gpg}" "SELECT COUNT(*) FROM events;")
    log "Events: $EVENT_COUNT"
else
    log "ERROR: Backup verification FAILED!"
fi

# Cleanup
rm -f "${BACKUP_DIR}/verify_${LATEST}" "${BACKUP_DIR}/verify_${LATEST%.gpg}"

log "Verification complete"
```

```bash
chmod +x /opt/www/verify-backups.sh
chown www:www /opt/www/verify-backups.sh
```

---

## Security Considerations

### Encryption

- **GPG encryption** ensures backups are encrypted at rest
- Even if Nextcloud is compromised, backups are unreadable without the private key
- Use RSA-4096 for strong encryption

### Credential Security

- App password limits access to only WebDAV
- Credentials file is `chmod 600` (owner read/write only)
- Consider using Nextcloud's external storage with separate user for isolation

### Key Management

| Key | Location | Purpose |
|-----|----------|---------|
| GPG Public Key | VPS `/home/www/backup-public.key` | Encrypting backups |
| GPG Private Key | Password manager / offline storage | Decrypting backups |
| Nextcloud App Password | VPS `/opt/www/.backup-credentials` | WebDAV authentication |

**Never store the GPG private key only on the VPS!** If the VPS is lost, you lose ability to restore.

### Network Security

- Use HTTPS for Nextcloud WebDAV
- Consider IP allowlisting in Nextcloud if possible
- App password can be revoked without affecting main account

---

## Monitoring

### Check Backup Status

```bash
# View recent backup logs
tail -50 /var/log/www-backup.log

# Check last backup date on Nextcloud
curl -s -u "${NEXTCLOUD_USER}:${NEXTCLOUD_PASS}" \
    -X PROPFIND "${NEXTCLOUD_URL}" \
    -H "Depth: 1" \
    | grep -oP 'app_\d{8}_\d{6}\.db\.gpg' \
    | sort -r \
    | head -5
```

### Alerting (Optional)

Add to backup script for failure notifications:

```bash
# At end of backup.sh, on failure:
send_alert() {
    curl -s -X POST "https://api.resend.com/emails" \
        -H "Authorization: Bearer ${RESEND_API_KEY}" \
        -H "Content-Type: application/json" \
        -d "{
            \"from\": \"backup@mail.valentinrogg.de\",
            \"to\": \"mail@valentinrogg.de\",
            \"subject\": \"Backup Failed\",
            \"text\": \"Backup failed at $(date). Check /var/log/www-backup.log\"
        }"
}

# Trap errors
trap 'send_alert' ERR
```

---

## Disaster Recovery Procedure

### Complete Server Loss

1. **Provision new VPS** on Hetzner
2. **Install dependencies** (Docker, GPG, curl)
3. **Import GPG private key**
   ```bash
   gpg --import backup-private.key
   ```
4. **Set up credentials**
   ```bash
   # Create /opt/www/.backup-credentials with Nextcloud details
   ```
5. **List available backups**
   ```bash
   ./restore.sh
   ```
6. **Restore latest backup**
   ```bash
   ./restore.sh app_20240115_030000.db.gpg
   ```
7. **Deploy application**
   ```bash
   docker compose up -d
   ```

### Estimated Recovery Time

| Step | Duration |
|------|----------|
| Provision VPS | 5 min |
| Install dependencies | 10 min |
| Import keys & credentials | 5 min |
| Download & restore backup | 5 min |
| Deploy application | 10 min |
| DNS update (if needed) | 5-60 min |
| **Total** | **~40 min + DNS** |

---

## Testing Checklist

- [ ] Backup script runs without errors
- [ ] Backup appears in Nextcloud
- [ ] Backup can be downloaded from Nextcloud
- [ ] Backup can be decrypted with GPG
- [ ] Decrypted backup passes integrity check
- [ ] Restore script works correctly
- [ ] Application works after restore
- [ ] Cron job executes on schedule
- [ ] Old backups are cleaned up correctly
- [ ] Weekly verification runs successfully
