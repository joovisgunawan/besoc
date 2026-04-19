COMPOSE_DB=docker-compose.db.yml
COMPOSE_REDIS=docker-compose.redis.yml
COMPOSE_KAFKA=docker-compose.kafka.yml
COMPOSE_ELK=docker-compose.elk.yml
COMPOSE_JAEGER=docker-compose.jaeger.yml
COMPOSE_APP=docker-compose.app.yml

NETWORK=backend_network

# =========================
# NETWORKS (GLOBAL INFRA)
# =========================
create-networks:
	docker network create backend_network || true

remove-networks:
	docker network rm backend_network || true

# =========================
# UP ALL INFRA
# =========================
up-db:
	docker compose -f $(COMPOSE_DB) up -d

up-redis:
	docker compose -f $(COMPOSE_REDIS) up -d

up-kafka:
	docker compose -f $(COMPOSE_KAFKA) up -d

up-elk:
	docker compose -f $(COMPOSE_ELK) up -d

up-jaeger:
	docker compose -f $(COMPOSE_JAEGER) up -d

up-infra: up-db up-redis up-kafka up-elk up-jaeger

# =========================
# DOWN ALL INFRA
# =========================
down-db:
	docker compose -f $(COMPOSE_DB) down -v

down-redis:
	docker compose -f $(COMPOSE_REDIS) down -v

down-kafka:
	docker compose -f $(COMPOSE_KAFKA) down -v

down-elk:
	docker compose -f $(COMPOSE_ELK) down -v

down-jaeger:
	docker compose -f $(COMPOSE_JAEGER) down -v

down-infra: down-jaeger down-elk down-kafka down-redis down-db

# =========================
# APP
# =========================
up-app:
	docker compose -f $(COMPOSE_APP) up -d --build

down-app:
	docker compose -f $(COMPOSE_APP) down -v

# =========================
# GLOBAL CONTROL
# =========================
up-all: create-networks up-infra up-app

down-all: remove-networks down-app down-infra

restart: down-all remove-networks create-networks up-all

# =========================
# CLEAN SYSTEM (optional)
# =========================
clean:
	docker system prune -f