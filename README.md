# Running ft_transendence
---
# Setting up a Development Environment for ft_transendence

This README will guide you through setting up a development environment for ft_transendence, a project utilizing Vue.js and Rust. By following these instructions, you'll be able to clone the repository, set up Docker, and configure Visual Studio Code (VSCode) to work with the development container.

## Prerequisites

Before starting, make sure you have the following installed:

- [Git](https://git-scm.com/downloads)
- [Docker](https://www.docker.com/products/docker-desktop)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Docker](https://www.docker.com/products/docker-desktop/)


## Step 1: Clone the Repository

First, clone the repository to your local machine:

```sh
git clone git@github.com:FelixBrgm/42-ft_transendence.git ft_transendence

```

## Step 2: Start the Dev Container
With Docker installed, navigate to the project directory in your terminal:

```
cd ft_transendence
```
and run:

```
make
```
This will start the development container inside the repository.


## Step 3: Configure Visual Studio Code

1. Open the project folder in VSCode.
2. Install the [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension.
3. Follow the instructions to attach the VSCode session to the development container.

## Recommended Extensions

For a smoother development experience, we recommend installing the following extensions:

### Vue.js
- [Vetur](https://marketplace.visualstudio.com/items?itemName=octref.vetur)
- [Vue VSCode Snippets](https://marketplace.visualstudio.com/items?itemName=sdras.vue-vscode-snippets)

### Rust
- [Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

## Exposed Ports

The following ports are exposed to the host machine for easy access to various services:

- `4242`: server

## Working with Git

To push and pull changes from the repository, use the terminal outside of the development container. This ensures that your Git credentials are correctly set up and your commits are attributed to your account.

```sh
# Pull the latest changes
git pull

# Push your changes
git add .
git commit -m "Your commit message"
git push


### TODO
- automated tests
- 42api answer
- load balancer

