# Axum Backend Learning Lab

Welcome to the Backend Learning Lab! This project is completely isolated from `leetcode-rust` and provides a structured playground for mastering modern Rust backend development using `axum`.

## The Learning Plan

Your goal is to work through each of the following modules, filling in the `todo!()` placeholders with actual implementation logic. 

### 1. Routing & CRUD (`src/routes/crud.rs`)
Understand axum extractors (JSON, Path, Query) by implementing endpoints for Create, Read, Update, and Delete operations on a resource (like a User).

### 2. Database Integration (`src/state/db.rs`)
Connect to PostgreSQL using `sqlx`. Manage connection pools via Axum's `State`. Run actual SQL queries in your CRUD handlers.

### 3. Caching with Redis (`src/state/cache.rs`)
Implement a cache-aside pattern to speed up reads, utilizing the `redis` crate. 

### 4. Authentication (`src/routes/auth.rs`)
Issue JSON Web Tokens (JWT) upon login. Understand how to hash passwords and return secure tokens to the client.

### 5. Custom Middleware (`src/middleware/auth_guard.rs`)
Write a custom `auth_guard` middleware to protect your CRUD routes. The middleware should intercept the request, extract the JWT from the `Authorization` header, and validate it.

### 6. Server-Sent Events (SSE) (`src/routes/sse.rs`)
Stream real-time data to clients using Axum's SSE features. Useful for building real-time notification systems or chat apps.

Good luck! 🦀
