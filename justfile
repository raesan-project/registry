DATABASE_FOLDER := "../raesan/database"
DATABASE_FILE := "raesan_base.db"
APP_ENV := "development"
QUESTIONS_FOLDER := "./_registry/questions"
REGISTRY_FOLDER := "./_registry"

default:
	just -l

setup-db:
	mkdir -p {{DATABASE_FOLDER}}
	diesel setup --database-url {{DATABASE_FOLDER}}/{{DATABASE_FILE}}
	diesel migration run --database-url {{DATABASE_FOLDER}}/{{DATABASE_FILE}}
serve:
	cargo run serve-questions --questions-folder {{QUESTIONS_FOLDER}}
gen-db:
	cargo run generate-database-records --database {{DATABASE_FOLDER}}/{{DATABASE_FILE}} --registry {{REGISTRY_FOLDER}}
