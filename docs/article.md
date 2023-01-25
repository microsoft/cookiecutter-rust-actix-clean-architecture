# Actix Web Clean Architecture

## Article
This source code is related to the article [Microsoft CSE Dev blog: Rust Clean Architecture]().

## Overview
This repository contains the source code of a clean architecture implementation
(onion architecture) using rust Actix Web. The goal of this repository is to
provide a clean architecture implementation that can be used as a template for 
future projects. Also, it is a good example of how to use the onion architecture
with rust, what are the benefits and how to use it and the limitations that 
you would encounter with this approach (in a rust context).

## Requirements and installation
* [Rust 1.51.0 or higher](https://www.rust-lang.org/tools/install)
* [Docker](https://docs.docker.com/get-docker/)

## Architecture deep dive
In this section, we will go through the different layers of onion architecture
and some of the decisions that were made. Also, we will go through some common questions to 
help you understand the architecture better and resolve some of the questions that you might have.

The full architecture explanation article can be found [here]()

### Architecture overview
The architecture is based on the onion architecture. The onion architecture is a 
layered architecture that is based on the onion model. Where each layer in the onion
model is used to define the different layers of an application.

Folder structure:
```
├── Cargo.lock
├── Cargo.toml -> Dependencies and project configuration
├── README.md <-- You are here
├── docs -> Documentation related to the project
├── src
│   ├── domain # Domain layer
│   │   ├── models # Domain models/entities
│   │   │  ├── model_a.rs # Example of a domain model
│   │   ├── repositories # Domain repositories interfaces/traits
│   │   └── services # Domain services interfaces/traits
│   ├── services # Services layer (implementations of business logic)
│   │   │   ├── service_a.rs # Service A implementation
│   ├── infrastructure # Infrastructure layer
│   │   ├── db # Infrastructure database adapters and services
│   │   ├── models # Concrete domain models/entities implementations (ORM, database models, concrete implemenations)
│   │   ├── repositories # Infrastructure repositories implementations (ORM, database repositories, concrete implemenations)

```

#### Domain layer

#### Service layer

#### Infrastructure layer

#### Api/Application layer

#### Best practices with onion architecture

##### Dependency inversion principle in combination with dependency injection

##### Data layer abstraction
["The database is not the center. Its external"][onion-architecture]

##### Domain models and business logic abstraction
The _ is pure in the functional sense, i.e. it has no side-effects. This is where our business logic resides. It is exceptionally easy to test because its pure functions only take and return values. In our example, our _core_ is just a single function that takes 2 integers and adds them together. In the _core_, we don't think about IO at all.

##### Adapters and ports (where needed)

### What is clean architecture and especially onion architecture?
Similar to [Hexagonal Architecture][hexagonal-architecture] (also known as "Ports and Adapters") and [Uncle Bob's Clean Architecture][clean-architecture], the [Onion Architecture][onion-architecture] advocates a structure for our application that allows us to segregate core business logic.

Imagine the concentric layers of an onion where you can only call inwards (i.e. from an outer layer to an inner layer). Let's see how this might work by starting at its core.


### Why onion architecture?

### Is rust a good fit for onion architecture?

### How does actix web fit in onion architecture?
Actix web was originally based on Actix and an actor framework. 
Actors are objects which encapsulate state and behavior, they communicate exclusively by exchanging messages. Actix actors are implemented on top of Tokio. Multiple actors can run in same thread.

Actix web still leans heavily on the actor model, therefore you will encounter some concepts that are related to the actor model. 
For example, the `actix_web::web::Data` is a shared state between all the actors.

In other languages, you would use dependency injection to inject the dependencies into the actors. In rust, you would use the `actix_web::web::Data` to share the state between the actors.

### What are common pitfalls when using onion architecture in combination with actix web?


[clean-architecture]: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html
[hexagonal-architecture]: https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)
[onion-architecture]: https://jeffreypalermo.com/2008/07/the-onion-architecture-part-1/
[rust]: https://www.rust-lang.org/