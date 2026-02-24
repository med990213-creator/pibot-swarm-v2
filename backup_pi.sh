#!/bin/bash
# 🥧 Pi Swarm Resurrection Script
# وظيفة هذا السكربت: أرشفة كامل كيان Pi مع ذاكرته وخبرته

WORKSPACE="/home/faycel1/.openclaw/workspace/pibot/swarm_v2"
BACKUP_NAME="pi_swarm_backup_$(date +%Y%m%d_%H%M%S).tar.gz"
BACKUP_DIR="/home/faycel1/.openclaw/workspace/backups"

mkdir -p $BACKUP_DIR

echo "--- 🥧 Starting Pi Swarm Backup ---"

# الأرشفة
tar -czf $BACKUP_DIR/$BACKUP_NAME -C $WORKSPACE .

if [ $? -eq 0 ]; then
    echo "✅ Backup Successful!"
    echo "📂 File location: $BACKUP_DIR/$BACKUP_NAME"
    echo "--- 💡 How to Restore ---"
    echo "1. Create a new directory."
    echo "2. Run: tar -xzf $BACKUP_NAME -C /path/to/new/location"
else
    echo "❌ Backup Failed!"
fi
