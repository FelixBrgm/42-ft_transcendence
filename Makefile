NAME:=transcendence 

all: $(NAME)

$(NAME):
	echo Not implemented yet

postgres:
	cd database && docker-compose up -d

dev_build:
	docker build -t dev ./.devcontainer/
dev_run:
	docker run -v ./source:/usr/src -d --name dev dev
	docker exec  -it dev /bin/bash

dev_stop:
	docker stop dev
	docker rm dev

http:
	set -a; . ./database/.env; set +a && cd source/server/http && cargo run
