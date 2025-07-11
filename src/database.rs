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
}
