use sqlx::PgPool;
use std::env;

/// Database wrapper that handles PostgreSQL connections and provides
/// a high-level interface for database operations.
///
/// This struct manages a connection pool and provides methods for
/// executing queries, managing transactions, and handling database lifecycle.
///
/// # Examples
///
/// ```rust
/// use dothtml_backend::database::Database;
///
/// #[tokio::main]
/// async fn main() -> Result<(), sqlx::Error> {
///     let db = Database::new().await?;
///     db.test_connection().await?;
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Database {
    /// PostgreSQL connection pool
    pub pool: PgPool,
}

impl Database {
    /// Creates a new Database instance and establishes a connection pool.
    ///
    /// This method loads the database configuration from environment variables
    /// (specifically `DATABASE_URL`) and creates a connection pool to PostgreSQL.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Database` instance on success,
    /// or a `sqlx::Error` if the connection fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The `DATABASE_URL` environment variable is not set
    /// - The database connection cannot be established
    /// - The database URL format is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dothtml_backend::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     println!("Database connected successfully!");
    ///     Ok(())
    /// }
    /// ```
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Load environment variables from .env file
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

        let pool = PgPool::connect(&database_url).await?;

        Ok(Database { pool })
    }

    /// Returns a reference to the underlying PostgreSQL connection pool.
    ///
    /// This method provides direct access to the SQLx PgPool for advanced
    /// database operations that require direct pool access.
    ///
    /// # Returns
    ///
    /// A reference to the `PgPool` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dothtml_backend::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     let pool = db.get_pool();
    ///     // Use pool for advanced operations
    ///     Ok(())
    /// }
    /// ```
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    /// Tests the database connection by executing a simple query.
    ///
    /// This method performs a basic connectivity test by executing a
    /// `SELECT 1` query against the database. It's useful for health checks
    /// and verifying that the database is accessible.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the connection test succeeds, or a `sqlx::Error`
    /// if the test fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The database connection is not available
    /// - The query execution fails
    /// - Network connectivity issues occur
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dothtml_backend::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     db.test_connection().await?;
    ///     println!("Database is healthy!");
    ///     Ok(())
    /// }
    /// ```
    pub async fn test_connection(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;

        println!("✅ PostgreSQL connection successful!");
        Ok(())
    }

    /// Closes all connections in the connection pool.
    ///
    /// This method gracefully shuts down the connection pool, closing all
    /// active connections. It should be called when the application is
    /// shutting down to ensure proper cleanup.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dothtml_backend::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     // ... use database
    ///     db.close().await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn close(&self) {
        self.pool.close().await;
    }
}

/// Database query methods for executing common SQL operations.
///
/// This implementation block provides convenient methods for executing
/// various types of database queries with proper error handling and
/// type safety.
impl Database {
    /// Executes a custom SQL query and returns the query result.
    ///
    /// This method is useful for INSERT, UPDATE, DELETE operations
    /// where you need information about the number of affected rows.
    ///
    /// # Arguments
    ///
    /// * `query` - The SQL query string to execute
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `PgQueryResult` on success,
    /// or a `sqlx::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The SQL query is malformed
    /// - Database connection issues occur
    /// - The query violates database constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dothtml_backend::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     let result = db.execute_query("UPDATE users SET active = true").await?;
    ///     println!("Affected rows: {}", result.rows_affected());
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute_query(
        &self,
        query: &str,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query(query).execute(&self.pool).await
    }

    /// Executes a query and returns all matching rows.
    ///
    /// This method is ideal for SELECT queries where you expect
    /// multiple rows to be returned.
    ///
    /// # Arguments
    ///
    /// * `query` - The SQL query string to execute
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `PgRow` on success,
    /// or a `sqlx::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The SQL query is malformed
    /// - Database connection issues occur
    /// - Column access errors occur
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dothtml_backend::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     let rows = db.fetch_all("SELECT id, name FROM users").await?;
    ///     for row in rows {
    ///         let id: i32 = row.get("id");
    ///         let name: String = row.get("name");
    ///         println!("User: {} - {}", id, name);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_all(&self, query: &str) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
        sqlx::query(query).fetch_all(&self.pool).await
    }

    /// Executes a query and returns exactly one row.
    ///
    /// This method is used when you expect exactly one row to be returned.
    /// It will fail if no rows or more than one row is returned.
    ///
    /// # Arguments
    ///
    /// * `query` - The SQL query string to execute
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a single `PgRow` on success,
    /// or a `sqlx::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The SQL query is malformed
    /// - No rows are returned
    /// - More than one row is returned
    /// - Database connection issues occur
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dothtml_backend::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     let row = db.fetch_one("SELECT COUNT(*) as count FROM users").await?;
    ///     let count: i64 = row.get("count");
    ///     println!("Total users: {}", count);
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_one(&self, query: &str) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
        sqlx::query(query).fetch_one(&self.pool).await
    }

    /// Executes a query and returns an optional row.
    ///
    /// This method is used when you expect zero or one row to be returned.
    /// It returns `None` if no rows are found, or `Some(row)` if exactly
    /// one row is found.
    ///
    /// # Arguments
    ///
    /// * `query` - The SQL query string to execute
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<PgRow>` on success,
    /// or a `sqlx::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The SQL query is malformed
    /// - More than one row is returned
    /// - Database connection issues occur
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dothtml_backend::database::Database;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     let row = db.fetch_optional("SELECT * FROM users WHERE id = 1").await?;
    ///     match row {
    ///         Some(user) => {
    ///             let name: String = user.get("name");
    ///             println!("Found user: {}", name);
    ///         }
    ///         None => println!("User not found"),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_optional(
        &self,
        query: &str,
    ) -> Result<Option<sqlx::postgres::PgRow>, sqlx::Error> {
        sqlx::query(query).fetch_optional(&self.pool).await
    }
}
