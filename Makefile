NAME:=transcendence 

all: $(NAME)

$(NAME):
	echo Not implemented yet

postgres:
	cd database && docker-compose up -d

dev:
	docker build -t dev ./.devcontainer/
	docker run -it --name dev dev


http:
	set -a; . ./database/.env; set +a && cd source/server/http && cargo run