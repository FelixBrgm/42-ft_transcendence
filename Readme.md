# 42-ft_transcendence

This is the final project of the [42Schools](https://www.42heilbronn.de/en/) curriculum.

Welcome to the Multiplayer Pong Contest Website project by [pgorner](github.com/Pgorner), [afenzl](github.com/annafenzl) and me!
This project aims to create a web platform where users can participate in live multiplayer Pong games and tournaments. The project is a complex undertaking with various mandatory requirements and optional modules to enhance functionality. 

## Table of Contents
1. [Introduction](#introduction)
2. [Features](#features)
3. [Technologies Used](#technologies-used)
4. [Installation](#installation)
5. [Usage](#usage)


## Introduction
The Multiplayer Pong Contest Website allows users to play Pong against each other in real-time and participate in tournaments. The project follows strict guidelines for development, including specific requirements for backend, frontend, gameplay, security, and deployment.

## Features
- Local and remote Pong games with real-time interaction.
- User registration using oauth.
- Tournament system for organizing matches between multiple players.
- Matchmaking system to pair players for games.
- Security measures against common web vulnerabilities like SQL injections and XSS.
- Docker-based deployment for easy setup and scalability.

## Technologies Used
- Backend: Rust, with actix_web and diesel.
- Frontend: Vue.
- Database: Postgres.
- Docker: Used for containerization and deployment.

## Installation
**1.** Clone the repository to your local machine.\
**2.** Navigate to the project directory.\
**3.** Ensure Docker is installed and running on your system.\
**4.**  Add a `.env` file to the root directory of the project:

```plaintext
POSTGRES_HOST=
POSTGRES_PORT=
POSTGRES_USER=
POSTGRES_PASSWORD=
POSTGRES_DB=
DATABASE_URL=
CLIENT_ID=
CLIENT_SECRET=
SESSION_KEY=
REDIRECT_URI=https://$IP_ADDR/api/auth/callback
INEND_URL=https://$IP_ADDR
VUE_APP_IP=$IP_ADDR
```

Replace the empty values with your specific configuration.

**5.** Run `docker-compose up` to launch the application.

## Usage
- Access the website through your browser at `https://$YOUR_IP_ADDRESS`.
- Register with your intra account play tournaments and Pong games against other players.
- Enjoy the classic Pong experience with modern multiplayer features!

## Additional Notes

- Increase the number of cores in the Docker desktop app settings for better performance.
- Expect approximately 50% more compile time compared to host when allocating 10 cores to Docker.
