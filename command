cargo new student-servie
cd student-service
mkdir -p src/{db,kafka,models,dto,repositories,services,handlers,routes,db}
mkdir -p src/cache

touch .env
touch init.sql
touch src/{config,error,response,state}.rs
touch src/db/mod.rs
touch src/kafka/producer.rs
touch src/models/student.rs
touch src/dto/student_dto.rs
touch src/repositories/student_repository.rs
touch src/services/student_service.rs
touch src/handlers/student_handler.rs
touch src/routes/student_routes.rs
touch src/routes/init.sql
touch src/routes/seed.rs
touch src/cache/mod.rs


docker run -d \
  --name jaeger \
  -p 4317:4317 \
  -p 16686:16686 \
  jaegertracing/all-in-one:latest

http://localhost:16686

docker run -d \
  --name student-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=student_service \
  -p 5432:5432 \
  postgres:16

cargo install sqlx-cli --no-default-features --features postgres
sqlx --version
sqlx database create
mkdir migrations
cp init.sql migrations/20240101000000_init.sql

sqlx migrate run
cargo sqlx prepare
cargo run

docker inspect student-db | grep POSTGRES_PASSWORD

docker run -d \
  --name kafka \
  -p 9092:9092 \
  -e KAFKA_CFG_NODE_ID=0 \
  -e KAFKA_CFG_PROCESS_ROLES=controller,broker \
  -e KAFKA_CFG_LISTENERS=PLAINTEXT://:9092,CONTROLLER://:9093 \
  -e KAFKA_CFG_LISTENER_SECURITY_PROTOCOL_MAP=CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT \
  -e KAFKA_CFG_CONTROLLER_QUORUM_VOTERS=0@localhost:9093 \
  -e KAFKA_CFG_CONTROLLER_LISTENER_NAMES=CONTROLLER \
  bitnami/kafka:latest

docker run -d \
  --name kafka \
  -p 9092:9092 \
  -e KAFKA_NODE_ID=1 \
  -e KAFKA_PROCESS_ROLES=broker,controller \
  -e KAFKA_LISTENERS=PLAINTEXT://:9092,CONTROLLER://:9093 \
  -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092 \
  -e KAFKA_CONTROLLER_LISTENER_NAMES=CONTROLLER \
  -e KAFKA_LISTENER_SECURITY_PROTOCOL_MAP=CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT \
  -e KAFKA_CONTROLLER_QUORUM_VOTERS=1@localhost:9093 \
  -e KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR=1 \
  -e KAFKA_TRANSACTION_STATE_LOG_REPLICATION_FACTOR=1 \
  -e KAFKA_TRANSACTION_STATE_LOG_MIN_ISR=1 \
  -e KAFKA_LOG_DIRS=/tmp/kraft-combined-logs \
  -e CLUSTER_ID=MkU3OEVBNTcwNTJENDM2Qk \
  apache/kafka:latest

# create
curl -s -X POST http://localhost:8080/students \
  -H "Content-Type: application/json" \
  -d '{"student_number": "STU001", "name": "John Doe", "email": "john@example.com"}' | jq

# get all
curl -s http://localhost:8080/students | jq

# get one (replace UUID from create response)
curl -s http://localhost:8080/students/YOUR_UUID | jq

# update (version must match current version)
curl -s -X PATCH http://localhost:8080/students/YOUR_UUID \
  -H "Content-Type: application/json" \
  -d '{"name": "Jane Doe", "version": 1}' | jq

# delete
curl -X DELETE http://localhost:8080/students/YOUR_UUID

docker exec -it student-db psql -U postgres -d student_service -c "EXPLAIN ANALYZE SELECT * FROM students WHERE deleted_at IS NULL ORDER BY created_at DESC LIMIT 20 OFFSET 0;"


docker run -d \
  --name redis \
  -p 6379:6379 \
  redis:7-alpine


curl --version | head -1

# trigger a not found
curl -s http://localhost:8080/students/00000000-0000-0000-0000-000000000000 | jq

# trigger a duplicate
curl -s -X POST http://localhost:8080/students \
  -H "Content-Type: application/json" \
  -d '{"student_number": "STU001", "name": "John", "email": "john@example.com"}' | jq

In Jaeger you'll now see the span marked red with:
```
otel.status_code = ERROR
error = "student not found"

# request brotli
curl -s http://localhost:8080/students \
  -H "Accept-Encoding: br" \
  -o /dev/null \
  -w "Content-Encoding: %header{content-encoding}\nSize: %size_download bytes\n"

# request gzip
curl -s http://localhost:8080/students \
  -H "Accept-Encoding: gzip" \
  -o /dev/null \
  -w "Content-Encoding: %header{content-encoding}\nSize: %size_download bytes\n"

# no compression (baseline)
curl -s http://localhost:8080/students \
  -o /dev/null \
  -w "Size: %size_download bytes\n"


# check brotli
curl -s http://localhost:8080/students \
  -H "Accept-Encoding: br" \
  -D - \
  -o /dev/null | grep -i content-encoding

# check gzip
curl -s http://localhost:8080/students \
  -H "Accept-Encoding: gzip" \
  -D - \
  -o /dev/null | grep -i content-encoding


# no compression
curl -s http://localhost:8080/students | wc -c

# with gzip
curl -s http://localhost:8080/students \
  -H "Accept-Encoding: gzip" \
  --compressed | wc -c

# with brotli (needs curl built with brotli support)
curl -s http://localhost:8080/students \
  -H "Accept-Encoding: br" \
  --compressed | wc -c

# health checks
curl -s http://localhost:8080/health | jq
curl -s http://localhost:8080/health/ready | jq

# metrics
curl -s http://localhost:8080/metrics

# validation error
curl -s -X POST http://localhost:8080/students \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-jwt-token" \
  -d '{"student_number": "", "name": "x", "email": "notanemail"}' | jq

# rate limit — hit it 200+ times quickly
for i in {1..210}; do curl -s http://localhost:8080/health; done

#check request id
#look for x-request-id: 550e8400-e29b-41d4-a716-446655440000 in the header, the id can be changed
curl -i -X POST http://localhost:8080/students \
  -H "Content-Type: application/json" \
  -d '{"student_number": "STU001", "name": "John", "email": "john@example.com"}'

# Run 3 requests and see that each gets a unique ID
for i in {1..3}; do
  echo "Request $i:"
  curl -s -D - -X POST http://localhost:8080/students \
    -H "Content-Type: application/json" \
    -d '{"student_number": "STU001", "name": "John", "email": "john@example.com"}' \
    -o /dev/null \
    -w '  Request ID: %header{x-request-id}\n'
done


docker stats

sqlx database reset
sqlx migrate run
cargo sqlx prepare
cargo build


#FRESH STRAT
#remove cache and sqlx files
rm -rf .sqlx

#fix the checksum error by removing the checksum from the migration file
docker exec -it student-service-postgres-primary-1 \
  psql -U postgres -d student_service \
  -c "DELETE FROM _sqlx_migrations;"

# 3. re-apply migration as if fresh
sqlx migrate run


# 4. regenerate cache
cargo sqlx prepare


cargo check
cargo build
#clean everything
cargo clean
rm -rf target
rm -rf .sqlx

source .env
cargo sqlx prepare

80:8080
dibaca dnegan port 80 dimapkan ke port 8080
right one is the destionation, which meanthe continer, wheter dokcer or k8s, it use same concept
think it's like iptables,=, incoming request to port 80 map/forward into port 8080 of the atger(tagert here is container)


make redis, psotgress main and replica

try to use mkcert insted of slef signed cert

