# Variables
NAME=development
IMAGE_NAME=$(NAME)_image
CONTAINER_NAME=$(NAME)_container

# Development container
all: run

build:
	docker build -t $(IMAGE_NAME) .

run: build
	docker run -p 4242:4242 --rm -d -v $(shell pwd)/$(NAME):/usr/src/$(NAME) --name $(CONTAINER_NAME) $(IMAGE_NAME) tail -f /dev/null

stop:
	-docker kill $(CONTAINER_NAME)

clean: stop
	-docker container prune -f
	-docker image prune -f

fclean: clean
	-docker rmi $(IMAGE_NAME)

re: fclean all


# Utils 
exec:
	docker exec -it $(CONTAINER_NAME) /bin/bash


.PHONY: all build run stop clean fclean re exec
