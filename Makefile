.DEFAULT_GOAL := default
SHELL := /bin/bash


.PHONY: default
default: | help


.PHONY: diesel-migration
diesel-migration:  ## Diesel migration with replace Enum to Varchar
	diesel migration run
	sed -i 's/ -> Enum/ -> Varchar/' src/schema.rs
	sed -i 's/<Enum>/<Varchar>/' src/schema.rs


run:  ## Running a server
	sudo pkill cargo ; RUST_BACKTRACE=1 RUST_LOG=info,actix_web=info,rust_gqls=info cargo watch -x 'run'


run-debug:  ## Running a server with debug logging
	sudo pkill cargo ; RUST_BACKTRACE=1 RUST_LOG=debug,actix_web=debug,rust_gqls=debug cargo watch -x 'run'


run-trace:  ## Running a server with trace logging
	sudo pkill cargo ; RUST_BACKTRACE=1 RUST_LOG=trace,actix_web=trace,rust_gqls=trace cargo watch -x 'run'


.PHONY: help
help:  ## Show all of tasks
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

