test:
		docker compose -f docker/docker-compose.yaml run yew-ui bash -c "cd app/yew-ui && cargo test"
		docker compose -f docker/docker-compose.yaml run actix-api bash -c "cd app/actix-api && cargo test"

up:
		docker compose -f docker/docker-compose.yaml up
down:
		docker compose -f docker/docker-compose.yaml down
build:
		docker compose -f docker/docker-compose.yaml build