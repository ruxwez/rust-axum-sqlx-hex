# Rust REST API Example

This project is a demonstration of a REST API built with Rust, Axum, and SQLx, following a hexagonal architecture.

## Project Structure

The project is organized into the following layers:

-   **Domain**: Contains the core business logic and entities.
-   **Application**: Contains the business logic implementations.
-   **Infrastructure**: Contains the database connection and other external services.
-   **Adapters**: Contains the API and repository implementations.

## How to Run

1.  **Install Rust**: If you don't have Rust installed, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2.  **Run the application**:
    ```bash
    cargo run
    ```
3.  **The API will be available at** `http://127.0.0.1:3000`.

## API Endpoints

### Users

-   **Create a user**:
    ```bash
    curl -X POST -H "Content-Type: application/json" -d '{"name": "John Doe", "email": "john.doe@example.com"}' http://127.0.0.1:3000/users
    ```
-   **List all users**:
    ```bash
    curl http://127.0.0.1:3000/users
    ```

### Organizations

-   **Create an organization**:
    ```bash
    curl -X POST -H "Content-Type: application/json" -d '{"name": "My Organization"}' http://127.0.0.1:3000/organizations
    ```
-   **List all organizations**:
    ```bash
    curl http://127.0.0.1:3000/organizations
    ```
