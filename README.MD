# Gomoku Online Platform

## Overview

Gomoku, also known as Five in a Row, is a classic board game where players take turns placing their markers on a grid, aiming to align five in a row horizontally, vertically, or diagonally. This platform brings Gomoku into the digital age with exciting features, allowing players to enjoy the game online, whether competing against an AI, playing with friends, or spectating others' matches.

---

## Features

- **Play with Bot**: Challenge an AI opponent powered by the **Minimax algorithm** and **Threat Space Search** for strategic and competitive gameplay.
- **Play Online**: Compete with other players in real-time multiplayer mode.
- **Watch Games**: Spectate ongoing matches to learn strategies or support your friends.
- **Chat**: Engage in conversations with opponents and spectators using the in-app chat feature.
- **Login Options**:
  - Sign in with Google
  - Sign in with GitHub
  - Play anonymously without an account
- **Internationalization (i18n)**: The platform supports multiple languages, making it accessible to a global audience.

---

## Technologies Used

### Backend
- **Rust**: A modern programming language focused on performance and safety.
- **Axum**: A web framework for building APIs with ease and scalability.
- **WebSockets**: Used for real-time communication, allowing players to interact with each other and with the bot in real-time during the game.


### Frontend
- **TypeScript**: For strong typing and cleaner, maintainable code.
- **Svelte**: A lightweight, modern frontend framework for building fast and reactive user interfaces.
- **TailwindCSS**: Utility-first CSS framework for styling the application with minimal effort.
- **ShadCN UI**: A component library built on top of TailwindCSS, providing customizable UI components for building consistent, user-friendly interfaces.

### Database
- **PostgreSQL**: A reliable and robust relational database system.
- **SQLx**: An asynchronous, pure Rust SQL crate for interacting with the PostgreSQL database, providing compile-time query checking for safety and efficiency.

### Authentication
- **Supabase**: A powerful backend-as-a-service platform for user management.
- **GoTrue**: A service for handling authentication workflows.
- **JWT (JSON Web Tokens)**: Used for securely transmitting information between the client and server as a JSON object, ensuring safe and stateless user authentication.

### CI/CD
- **GitHub Actions**: Automates testing, building, and deployment pipelines.

### Monitoring
- **Jaeger**: Distributed tracing to monitor application performance.
- **Grafana & Prometheus**: Tools for visualization and metrics monitoring.


### Deployment
- **Docker**: All services are containerized using Docker to ensure easy deployment and scalability.
- **Nginx**: Used as a reverse proxy for serving the frontend and backend, handling traffic routing and load balancing.
- **Hetzner Cloud**: The application is deployed on a Hetzner Cloud virtual machine for a reliable, cost-effective hosting solution.

---

## Development

To get the project up and running locally, follow these steps:

### 1. Create an Environment File
First, create a `.env` file based on the provided example:

```bash
cp .env.example .env
```

### 2. Start the Database and Services
Use Docker Compose to start the necessary services, including the database, Kong, authentication, and Jaeger for monitoring:

```bash
docker-compose up db kong auth jaeger -d
```

### 3. Start the Backend
Navigate to the backend project and start the server using Cargo:

```bash
cargo run -p backend
```

### 4. Start the Frontend
To set up and start the frontend, follow these steps:

```bash
cd frontend
pnpm install
pnpm run dev
```

Now you should be able to access the project locally and begin development!
