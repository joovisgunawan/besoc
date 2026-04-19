#!/bin/bash
set -e

echo "Waiting for primary..."

until pg_isready -h postgres-primary -p 5432; do
  sleep 2
done

echo "Stopping replica data directory..."
rm -rf "$PGDATA"/*

echo "Starting base backup..."

pg_basebackup -h postgres-primary \
  -D "$PGDATA" \
  -U replicator \
  -Fp -Xs -P -R

echo "Replica setup complete"