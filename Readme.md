# Running ft_transendence
---
# Setting up a Development Environment for ft_transendence

This README will guide you through setting up a development environment for ft_transendence, a project utilizing Vue.js and Rust. By following these instructions, you'll be able to clone the repository, set up Docker, and configure Visual Studio Code (VSCode) to work with the development container.

## Prerequisites

Before starting, make sure you have the following installed:

- [Git](https://git-scm.com/downloads)
- [Docker](https://www.docker.com/products/docker-desktop)
- [Visual Studio Code](https://code.visualstudio.com/)


## Step 1: Clone the Repository

First, clone the repository to your local machine:

```sh
git clone git@github.com:FelixBrgm/42-ft_transendence.git

```

## Step 2: Start the Docker
Start docker!


## Step 3: Configure Visual Studio Code

1. Open the project folder in VSCode.
2. Install the [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension.
3. Follow the instructions to "reopen" the folder in a container.

## Working with Git

To push and pull changes from the repository, use the terminal outside of the development container. This ensures that your Git credentials are correctly set up and your commits are attributed to your account.

```sh
# Pull the latest changes
git pull

# Push your changes
git add .
git commit -m "Your commit message"
git pus
```
### TODO
- automated tests
- 42api answer
- load balancer

### Start database
Type in this command in the root of the project outside of the development environment
```
make postgres
```

### Additional Notes
- Set your cores higher in the docker desktop app
- 50% more compile time relative to host when 10 cores available to docker
