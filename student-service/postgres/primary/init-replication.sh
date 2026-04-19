#!/bin/bash
set -e

echo "Configuring replication user..."

psql -U postgres <<EOF
CREATE ROLE replicator WITH REPLICATION LOGIN PASSWORD 'replicator_password';
EOF

echo "Updating pg_hba.conf..."

echo "host replication replicator 0.0.0.0/0 md5" >> "$PGDATA/pg_hba.conf"