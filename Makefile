NAME:=transcendence 

all: $(NAME)

$(NAME):
	echo Not implemented yet

postgres:
	cd source/database && docker-compose up -d

http:
	set -a; . ./source/database/.env; set +a && cd source/server/http && cargo run