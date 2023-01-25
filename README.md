# Actix clean architecture

## Architecture 
The application follows the Onion Architecture pattern.
This architecture is a design pattern that organizes the codebase 
of a software application into multiple layers, where the innermost layer 
is the domain layer and the outermost layer is the application layer. 
Each layer depends only on the layers inside of it and not on the layers outside of it, 
creating a separation of concerns, allowing for a more maintainable and scalable codebase.

This architecture can be seen in the following diagram:
    
```
.
├── migrations
├── scripts
│   └── run_postgres.sh # Run postgres in docker locally
├── src
│   ├── api
│   │   ├── controllers
│   │   │   └── ...  # controllers for the api
│   │   ├── dto # Data transfer objects  
│   │   │   └── ... # Individual DTOs
│   │   └── errors.py
│   ├── infrastructure
│   │   ├── services
│   │   │   └── ...  # Services that use third party libraries or services (e.g. email service)
│   │   ├── databases
│   │   │   └── ...  # Database adapaters and initialization
│   │   ├── repositories
│   │   │   └── ...  # Repositories for interacting with the databases
│   │   └── models
│   │   │   └── ...  # Database models
│   ├── domain
│   │   ├── constants.py
│   │   ├── exceptions.py
│   │   ├── models
│   │   │   └── ...  # Business logic models
│   ├── services
│   │    └── ...  # Services for interacting with the domain (business logic)
│   ├── app.py
│   ├── config.py
│   ├── cors.py
│   ├── create_app.py
│   ├── dependency_container.py
│   ├── error_handler.py
│   └── logging.py
```
The application is structured with the following components:


## Database migrations

1) Make sure you have the diesel cli installed. You can install it with the following command:
    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    ```
2) Add your postgres database url to the .env file:
    ```bash
    echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
    ```
3) Setup diesel before creating a migration:
    ```bash
    diesel setup
    ```
4) Create a migration with the following command:
    ```bash
    diesel migration generate <migration_name>
    ```
5) Apply your migrations:
    ```bash
    diesel migration run
    ```

