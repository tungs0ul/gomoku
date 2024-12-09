# Gomoku Online Platform

## Overview

Gomoku, also known as Five in a Row, is a classic board game where players take turns placing their markers on a grid, aiming to align five in a row horizontally, vertically, or diagonally. This platform brings Gomoku into the digital age with exciting features, allowing players to enjoy the game online, whether competing against an AI, playing with friends, or spectating others' matches.

---

## Features

#### [Demo Website](https://sansantech.de)
- **Play with Bot**: Challenge an AI opponent powered by the **Minimax algorithm** and **Threat Space Search** for strategic and competitive gameplay.
- **Play Online**: Compete with other players in real-time multiplayer mode.
- **Watch Games**: Spectate ongoing matches to learn strategies or support your friends.
- **Chat**: Engage in conversations with opponents and spectators using the in-app chat feature.
- **Login Options**:
  - Sign in with Google
  - Sign in with GitHub
  - Play anonymously without an account
- **Internationalization (i18n)**: The platform supports multiple languages, making it accessible to a global audience.

#### Monitoring
- **[Jaeger](https://jaeger.sansantech.de)**
  - account: test
  - password: 123qwe
- **[Grafana](https://grafana.sansantech.de)**
  - account: test
  - password: 123qwe

#### Management
- **[Supabase Studio](https://kong.sansantech.de)**
- **[Nginx Proxy Manager](https://nginx.sansantech.de)**

---

## Technologies Used

#### Backend
- **[Rust](https://www.rust-lang.org/)**: A modern programming language focused on performance and safety.
- **[Axum](https://github.com/tokio-rs/axum)**: A web framework for building APIs with ease and scalability.
- **WebSockets**: Used for real-time communication, allowing players to interact with each other and with the bot in real-time during the game.


#### Frontend
- **[TypeScript](https://www.typescriptlang.org/)**: For strong typing and cleaner, maintainable code.
- **[Svelte](https://svelte.dev/)**: A lightweight, modern frontend framework for building fast and reactive user interfaces.
- **[TailwindCSS](https://tailwindcss.com/)**: Utility-first CSS framework for styling the application with minimal effort.
- **[ShadCN UI](https://www.shadcn-svelte.com/)**: A component library built on top of TailwindCSS, providing customizable UI components for building consistent, user-friendly interfaces.

#### Database
- **[PostgreSQL](https://github.com/postgres/postgres)**: A reliable and robust relational database system.
- **[sqlx](https://github.com/launchbadge/sqlx)**: An asynchronous, pure Rust SQL crate for interacting with the PostgreSQL database, providing compile-time query checking for safety and efficiency.

#### Authentication
- **[Supabase](https://supabase.com/)**: A powerful backend-as-a-service platform for user management.
- **[GoTrue](https://github.com/netlify/gotrue)**: A service for handling authentication workflows.
- **[JWT (JSON Web Tokens)](https://jwt.io/)**: Used for securely transmitting information between the client and server as a JSON object, ensuring safe and stateless user authentication.

#### CI/CD
- **[GitHub Actions](https://github.com/features/actions)**: Automates testing, building, and deployment pipelines.

#### Monitoring
- **[Jaeger](https://www.jaegertracing.io/)**: Distributed tracing to monitor application performance.
- **[Grafana](https://grafana.com/) & [Prometheus](https://prometheus.io/)**: Tools for visualization and metrics monitoring.


### Deployment
- **[Docker](https://www.docker.com/)**: All services are containerized using Docker to ensure easy deployment and scalability.
- **[Nginx](https://nginx.org/)**: Used as a reverse proxy for serving the frontend and backend, handling traffic routing and load balancing.
- **[Kong](https://github.com/Kong/kong)**: An API gateway for managing, securing, and monitoring API traffic, ensuring high performance and extensibility.
- **[Hetzner Cloud](https://www.hetzner.com/)**: The application is deployed on a Hetzner Cloud virtual machine for a reliable, cost-effective hosting solution.


---

## Development

To get the project up and running locally, follow these steps:

#### 1. Create an Environment File
First, create a `.env` file based on the provided example:

```bash
cp .env.example .env
```

#### 2. Start the Database and Services
Use Docker Compose to start the necessary services, including the database, Kong, authentication, and Jaeger for monitoring:

```bash
docker-compose up db kong auth jaeger -d
```

#### 3. Start the Backend
Navigate to the backend project and start the server using Cargo:

```bash
cargo run -p backend
```

#### 4. Start the Frontend
To set up and start the frontend, follow these steps:

```bash
cd frontend
pnpm install
pnpm run dev
```

Now you should be able to access the project locally and begin development!
