# healthytracker_assignment11
Generic Repository Interface

Defines CRUD operations (save, find_by_id, delete)

Uses async_trait for future async database support

Thread-Safe In-Memory Storage

HashMap-backed implementation with RwLock synchronization

Enforces business rules (e.g., max 3 devices per user)

Factory Abstraction

RepositoryFactory switches between storage backends
 
Currently supports in-memory with stubs for filesystem/DB

Test Coverage

Unit tests validate CRUD operations

Concurrent access scenarios verified

Future-Ready Design

Includes stub for JSON file storage

UML diagram shows extensibility points
