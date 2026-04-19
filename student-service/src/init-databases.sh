#!/bin/bash
set -e

psql -U postgres <<-EOSQL
    SELECT 'CREATE DATABASE student_service' WHERE NOT EXISTS (
        SELECT FROM pg_database WHERE datname = 'student_service'
    )\gexec

    SELECT 'CREATE DATABASE supervision_service' WHERE NOT EXISTS (
        SELECT FROM pg_database WHERE datname = 'supervision_service'
    )\gexec
EOSQL