DATABASE_FOLDER := "../raesan/database"
DATABASE_FILE := "raesan_base.db"
APP_ENV := "development"
REGISTRY_FOLDER := "./_registry"

default:
	just -l

setup-db:
	mkdir -p {{DATABASE_FOLDER}}
	diesel setup --database-url {{DATABASE_FOLDER}}/{{DATABASE_FILE}}
	diesel migration run --database-url {{DATABASE_FOLDER}}/{{DATABASE_FILE}}

serve:
	cargo watch -x "run serve-questions --registry {{REGISTRY_FOLDER}}" -i ./static

gen-db:
	cargo run generate-database-records --database {{DATABASE_FOLDER}}/{{DATABASE_FILE}} --registry {{REGISTRY_FOLDER}}

serve-tw:
	bun run tailwindcss -i ./src/tailwind.css -o ./static/style.css --minify --watch

build-tw:
	bun run tailwindcss -i ./src/tailwind.css -o ./static/style.css --minify

test:
    cargo test -- --nocapture
