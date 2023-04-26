NAME=server
IMAGE_NAME=$(NAME)_image
CONTAINER_NAME=$(NAME)_container

all: run

build:
	docker build -t $(IMAGE_NAME) .

run: build
	docker run -p 7878:7878 --rm -d -v $(shell pwd)/server:/usr/src/server --name $(CONTAINER_NAME) $(IMAGE_NAME)

dev: fclean build
	docker run -p 7878:7878 --rm -it -v $(shell pwd)/server:/usr/src/server --name $(CONTAINER_NAME) $(IMAGE_NAME) bash

stop:
	-docker kill $(CONTAINER_NAME)

clean: stop
	-docker container prune -f
	-docker image prune -f

fclean: clean
	-docker rmi $(IMAGE_NAME)

re: fclean run

#SHIT

exec:
	docker exec -it $(CONTAINER_NAME) /bin/bash

.PHONY: all build run dev stop clean fclean re
