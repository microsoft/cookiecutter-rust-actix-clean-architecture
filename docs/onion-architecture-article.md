# Rust onion architecture with actix and diesel
> All the source code of this article can be found [here](https://github.com/coding-kitties/cookiecutter-actix-simple-clean-architecture) and [here](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)

This article describes an onion architecture implementation with Rust using 
actix and diesel. 

Onion Architecture is a software design pattern that organizes the codebase into 
multiple layers. Each layer depends only on the layers inside of it and not on the layers outside of it,
This creates a separation of concerns, which allows for a more maintainable and scalable codebase.

Actix web is a popular and widely used web framework for Rust that provides a high-performance 
and scalable foundation for building web applications. It is built on top of the 
Actix actor framework, which allows for a clear separation of concerns and 
modularity, making it a good fit for implementing the application layer of the onion architecture

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
│   │       └── ...  # Database models
│   ├── domain
│   │   ├── mod.rs
│   │   ├── constants.rs
│   │   ├── errors.rs
│   │   ├── models
│   │   │   └── ...  # Business logic models traits or structs
│   │   ├── services
│   │   │   └── ...  # Service traits
│   │   └── repositories
│   │       └── ...  # Repository traits 
│   ├── services
│   │   └── ...  # Concrete service implementation for interacting with the domain (business logic)
│   ├── container.rs
│   ├── create_app.rs # app factory 
│   ├── lib.rs 
│   └── main.rs
```

* migrations: Alembic's migration scripts are stored here.
* scripts: contains the application's configuration settings.

## Is rust suitable for onion architecture?
The clear separation of concerns and modularity that the onion architecture 
provides aligns well with Rust's design philosophy. Rust's support for 
creating libraries and its strict type system allows for building well-defined,
decoupled and easily testable components, making it a good choice for 
implementing the domain, services, and infrastructure layers of the
onion architecture. Additionally, the Rust ecosystem has a growing number
of libraries and frameworks that support various use cases, such as database 
access, web development, and more, Actix in particular can easily integrate 
with other Rust libraries and frameworks, such as Diesel for database access, 
making it possible to build a full-stack application using the onion architecture with Rust.

The most problems you will face is the sharing your services 
and repositories between the different layers. There is currently for actix no
standardized or commonly used dependency injection framework. 


## Diesel adapter setup in the infrastructure layer
["The database is not the center. Its external"][onion-architecture]

Diesel is a popular and widely used Object Relational Mapping (ORM) 
library for Rust, and can be used in the infrastructure layer of an 
onion architecture. To set up Diesel in the infrastructure layer, the first 
step is to add Diesel as a dependency to the project's Cargo.toml file. Then, 
you need to define the schema for the database tables using Diesel's schema 
macro, which will generate the necessary Rust code to interact with the 
database. After that, you can create a database connection and set up the 
connection pool using the Diesel connection library. The connection pool 
can be used to manage multiple database connections, allowing for efficient 
reuse and sharing of database connections. To perform database operations, 
you can use Diesel's query builder to construct and execute database queries. 
When used in the infrastructure layer of an onion architecture, Diesel provides 
a clean and efficient way to interact with a database, abstracting the 
underlying database technology and allowing the services layer to interact 
with the database through a well-defined interface.

Diesel currently does not support async behaviour. There is now a project being made by
the creators of diesel to address this issue. [async diesel]()

However, to mitigate this issue, we will leverage actix to run our database orm operations in 
a seperate thread. We did this by using the `actix_web::web::Data` library as can be seen in the 
code snippet below: 



## Actix architecture adaptation
Factory method where all dependencies are initialized

Actix web was originally based on Actix and an actor framework. 
Actors are objects which encapsulate state and behavior, they communicate exclusively by exchanging messages. Actix actors are implemented on top of Tokio. Multiple actors can run in same thread.

Actix web still leans heavily on the actor model, therefore you will encounter some concepts that are related to the actor model. 
For example, the `actix_web::web::Data` is a shared state between all the actors.
In other languages, you would use dependency injection to inject the dependencies into the actors. In rust, you would use the `actix_web::web::Data` to share the state between the actors.

Share all services through the app state.

Actix web can be adapted to fit within the onion architecture by using it to 
implement the application layer. In this architecture, the application layer 
is responsible for handling the input and output of the application and 
coordinating the use of services. Actix web provides a simple and intuitive
API for defining endpoints, handling HTTP requests, and returning responses, 
making it a suitable choice for implementing the application layer. It can use 
the services' public APIs to process requests and return responses, and can 
integrate with the infrastructure layer, such as using Diesel for database
access, to perform database operations as needed. By adapting Actix web to 
fit within the onion architecture, it becomes a flexible and scalable tool 
for building web applications with clear separation of concerns and modularity. 
This makes it easier to maintain and evolve the application over time, as well 
as making it easier to test and debug individual components. Additionally, the 
strict modularity of the onion architecture makes it easier to swap out 
components with different implementations as needed.


## What are common pitfalls with a rust onion architecture
While Actix web and the onion architecture can be a powerful combination 
for building web applications in Rust, there are some common pitfalls to 
watch out for. One pitfall is overloading the application layer with too 
much logic, as this can result in a tightly-coupled and difficult-to-maintain 
application. It is important to maintain a clear separation of concerns 
and only perform necessary logic in the application layer, delegating other 
operations to the services layer. 

Another pitfall is not properly defining the APIs between the different layers, 
which can lead to tight coupling and make it difficult to modify or replace components in the future. 
It is important to carefully design and document the APIs between the different 
layers to ensure that they are flexible and maintainable over time. 

One example of this principle is the repository trait that we defined in our 
domain layer. Services that will use a repository for the logic will depend on 
the trait. Services will be unaware of the concrete implementation that they will use such as a diesel 
based repository. 

Additionally, not properly handling errors and exceptions can lead to 
unexpected behavior and crashes in the application. It is important to 
implement proper error handling and logging to ensure that the application 
is robust and can recover gracefully from unexpected events. Overall, careful 
design and implementation are key to avoiding these common pitfalls and 
building a successful Actix web and onion architecture based web application 
in Rust.

## Conclusion


[clean-architecture]: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
[hexagonal-architecture]: https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)
[onion-architecture]: https://jeffreypalermo.com/2008/07/the-onion-architecture-part-1/
[rust]: https://www.rust-lang.org/