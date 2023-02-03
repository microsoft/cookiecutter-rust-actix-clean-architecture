# Rust onion architecture with actix and diesel
> All the source code of this article can be found [here](https://github.com/coding-kitties/cookiecutter-actix-simple-clean-architecture) 

This article describes an onion architecture implementation with Rust using 
actix and diesel. 

Onion Architecture is a software design pattern that organizes the codebase into 
multiple layers. Each layer depends only on the layers inside of it and not on the layers outside of it,
This creates a separation of concerns, which allows for a more maintainable and scalable codebase.

# Why onion architecture for a rust based web application?
Onion architectures are commonly used by software engineering teams and in a wide amount of program languages. 
It is therefore interesting to see how Rust functions in such an architecture. 
Given that Rust is a new language compared to languages such as C#, java, C++, its is interesting to see how rust can 
adapt a standard such as onion architecture. If rust is suitable for such an architecture it can be a decision driver for 
new applications to adapt rust. 

Rust is know for its speed, however, even if rust is fast compared to other languages, production ready web applications 
typically use technologies such as a databases, caching or ORM's. These systems can slow a language down. However, taking into account
that a standard production ready Rust web application would use these systems, we can still see that 
Rust outperforms with a wide marging other languages.

## Architecture Overview
The onion architecture is a layered architecture that is based on the onion model. 
Where each layer in the onion model is used to define the different layers of an application.

For this rust implementation 4 layers are used. 
* api (app) module: The outermost layer that contains the controllers and the endpoints definition, serialization and deserialization of the data, validation and error handling.
* infrastructure: Layer that typically include database connections, external APIs calls, logging and configuration management.
* services: Layer that contains the application's services, which encapsulate the core business logic and provide a higher-level abstraction for the application to interact with the domain entities.
* domain: The innermost layer that contains the core business logic and entities of the application.


Folder structure:
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
│   ├── create_app.rs # app factory 
│   ├── container.rs.py # Dependency container 
│   └── logging.py
```

* migrations: Alembic's migration scripts are stored here.
* scripts: contains the application's configuration settings.

## Is rust suitable for onion architecture?
The most problems you will face is the sharing your services 
and repositories between the different layers. 


## Diesel adapter setup in the infrastructure layer
["The database is not the center. Its external"][onion-architecture]

## Actix architecture adaptation
Factory method where all dependencies are initialized

Actix web was originally based on Actix and an actor framework. 
Actors are objects which encapsulate state and behavior, they communicate exclusively by exchanging messages. Actix actors are implemented on top of Tokio. Multiple actors can run in same thread.

Actix web still leans heavily on the actor model, therefore you will encounter some concepts that are related to the actor model. 
For example, the `actix_web::web::Data` is a shared state between all the actors.
In other languages, you would use dependency injection to inject the dependencies into the actors. In rust, you would use the `actix_web::web::Data` to share the state between the actors.

Share all services through the app state.

## What are common pitfalls with a rust onion architecture
Let each layer have its own models.

## Conclusion


[clean-architecture]: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
[hexagonal-architecture]: https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)
[onion-architecture]: https://jeffreypalermo.com/2008/07/the-onion-architecture-part-1/
[rust]: https://www.rust-lang.org/