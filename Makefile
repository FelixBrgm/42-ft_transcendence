NAME:=transcendence 

all: $(NAME)

$(NAME):
	echo Not implemented yet

postgres:
	cd database && docker-compose up -d

http:
	set -a; . ./database/.env; set +a && cd source/server/http && cargo run

re: down up

up:
	cd .devcontainer && docker-compose up -d

down:
	cd .devcontainer && docker-compose down 
