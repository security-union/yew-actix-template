test:
		make test-api
		make test-ui
test-api:
		docker compose -f docker/docker-compose.yaml run actix-api bash -c "cd app/actix-api && cargo test -- --nocapture"
test-ui:
		docker compose -f docker/docker-compose.yaml run yew-ui bash -c "cd app/yew-ui && cargo test"
up:
		docker compose -f docker/docker-compose.yaml up
down:
		docker compose -f docker/docker-compose.yaml down
build:
		docker compose -f docker/docker-compose.yaml build

connect_to_db:
		docker compose -f docker/docker-compose.yaml run postgres bash -c "psql -h postgres -d actix-api-db -U postgres"


fmt:
		docker compose -f docker/docker-compose.yaml run yew-ui bash -c "cd app/yew-ui && cargo fmt"
		docker compose -f docker/docker-compose.yaml run actix-api bash -c "cd app/actix-api && cargo fmt && cd ../types && cargo fmt"

clippy-fix:
		docker compose -f docker/docker-compose.yaml run yew-ui bash -c "cd app/yew-ui && cargo clippy --fix"
		docker compose -f docker/docker-compose.yaml run actix-api bash -c "cd app/actix-api && cargo clippy --fix && cd ../types && cargo clippy --fix"

check:
		# The ui does not support clippy yet
		#docker compose -f docker/docker-compose.yaml run yew-ui bash -c "cd app/yew-ui && cargo clippy --all  -- --deny warnings && cargo fmt --check"
		docker compose -f docker/docker-compose.yaml run actix-api bash -c "cd app/actix-api && cargo clippy --all  -- --deny warnings && cargo fmt --check"

