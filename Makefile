NAME:=transcendence 

all: $(NAME) up

$(NAME):
	echo Not implemented yet

postgres:
	cd database && docker-compose up -d

dev_re: dev_stop dev_run

dev_build:
	docker build -t dev ./.devcontainer/

dev_run:
	docker run \
	-p 4242:4242 \
	-p 8080:8080 \
	-v ./source:/usr/src \
	-d --name dev dev \
	docker exec  -it dev /bin/bash

dev_stop:
	docker stop dev
	docker rm dev

http:
	set -a; . ./database/.env; set +a && cd source/server/http && cargo run

re: down up

up:
	@echo "hehe just kiiiidding"
	cd .devcontainer && docker-compose up -d

down:
	cd .devcontainer && docker-compose down 
