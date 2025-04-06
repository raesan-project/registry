DATABASE_FOLDER := "../raesan/database"
DATABASE_FILE := "raesan_base.db"
APP_ENV := "development"

default:
	just -l

setup-db:
	mkdir -p {{DATABASE_FOLDER}}
	diesel setup --database-url {{DATABASE_FOLDER}}/{{DATABASE_FILE}}
	diesel migration run --database-url {{DATABASE_FOLDER}}/{{DATABASE_FILE}}
