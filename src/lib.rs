//! # Dothtml Backend
//! 
//! A backend application built with Actix Web and PostgreSQL for managing
//! messages and contact forms. This crate provides database connectivity,
//! message handling, and HTTP API endpoints.
//! 
//! ## Features
//! 
//! - PostgreSQL database integration with connection pooling
//! - Message CRUD operations with UUID support
//! - RESTful API endpoints for message management
//! - Environment-based configuration
//! - Comprehensive error handling
//! 
//! ## Quick Start
//! 
//! ```rust
//! use dothtml_backend::database::Database;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let db = Database::new().await?;
//!     db.test_connection().await?;
//!     println!("Database connected successfully!");
//!     Ok(())
//! }
//! ```
//! 
//! ## Modules
//! 
//! - [`database`] - Database connection and query management
//! - [`models`] - Data models and database operations
//! - [`routes`] - HTTP route configuration
//! - [`handlers`] - HTTP request handlers

/// Database connection and query management
pub mod database;

/// Data models and database operations
pub mod models;

/// HTTP route configuration
pub mod routes;

/// HTTP request handlers
pub mod handlers;
