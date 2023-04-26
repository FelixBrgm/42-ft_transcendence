NAME=development
IMAGE_NAME=$(NAME)_image
CONTAINER_NAME=$(NAME)_container

all: run

build:
	docker build -t $(IMAGE_NAME) .

run: build
	docker run -p 4242:4242 --rm -d -v $(shell pwd)/$(NAME):/usr/src/$(NAME) --name $(CONTAINER_NAME) $(IMAGE_NAME)

dev: fclean build
	docker run -p 4242:4242 --rm -it -v $(shell pwd)/$(NAME):/usr/src/$(NAME) --name $(CONTAINER_NAME) $(IMAGE_NAME) bash

stop:
	-docker kill $(CONTAINER_NAME)

clean: stop
	-docker container prune -f
	-docker image prune -f

fclean: clean
	-docker rmi $(IMAGE_NAME)

re: fclean dev

#SHIT

exec:
	docker exec -it $(CONTAINER_NAME) /bin/bash

.PHONY: all build run dev stop clean fclean re
