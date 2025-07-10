use crate::database::Database;
use sqlx::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a message in the system.
/// 
/// This struct models a message with all its associated metadata,
/// including sender information, timestamps, and status tracking.
/// 
/// # Fields
/// 
/// * `id` - Unique identifier for the message
/// * `name` - The sender's name
/// * `email` - The sender's email address
/// * `country_region` - The sender's country/region
/// * `phone_number` - The sender's phone number
/// * `company` - The company associated with the sender
/// * `message` - The message content/body
/// * `created_at` - Timestamp when the message was created
/// * `assigned_to` - Optional field for the person assigned to handle the message
/// * `status` - Current status of the message (e.g., "pending", "assigned", "resolved")
/// 
/// # Examples
/// 
/// ```rust
/// use dothtml_backend::models::Message;
/// use uuid::Uuid;
/// use chrono::Utc;
/// 
/// let message = Message {
///     id: Uuid::new_v4(),
///     content: "Hello, world!".to_string(),
///     sender_email: "user@example.com".to_string(),
///     created_at: Utc::now(),
///     assigned_to: None,
///     status: "pending".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,

    pub name: String,
    pub email: String,
    pub country_region: String,
    pub phone_number: String,
    pub company: String,
    pub message: String,

    pub created_at: DateTime<Utc>,
    pub assigned_to: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct PendingMessage {
    pub id: Uuid,

    pub name: String,
    pub email: String,
    pub message: String,
}

/// Database operations for the Message model.
/// 
/// This implementation provides CRUD operations and specialized queries
/// for the Message model, including table creation, insertion, retrieval,
/// updates, and deletion operations.
impl Database {
    /// Creates the 'messages' table if it doesn't exist.
    /// 
    /// This method sets up the database schema for storing messages.
    /// It creates a table with all necessary columns and constraints,
    /// including UUID primary key generation and timestamp defaults.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the table is created successfully or already exists,
    /// or a `sqlx::Error` if the operation fails.
    /// 
    /// # Errors
    /// 
    /// This function returns an error if:
    /// - Database connection issues occur
    /// - Insufficient permissions for table creation
    /// - SQL syntax errors in the CREATE TABLE statement
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use dothtml_backend::database::Database;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     db.create_messages_table().await?;
    ///     println!("Messages table ready!");
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_messages_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(r#"
            CREATE TABLE messages (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                country_region TEXT NOT NULL,
                phone_number TEXT NOT NULL,
                company TEXT NOT NULL,
                message TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                assigned_to TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                CONSTRAINT email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
            );
        "#)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Inserts a new message into the database.
    /// 
    /// This method creates a new message record with the provided content
    /// and sender email. The database
    /// automatically generates the ID, timestamp, and default status.
    /// 
    /// # Arguments
    /// 
    /// * `content` - The message content/body
    /// * `sender_email` - Email address of the message sender
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing the created `Message` on success,
    /// or a `sqlx::Error` on failure.
    /// 
    /// # Errors
    /// 
    /// This function returns an error if:
    /// - Database connection issues occur
    /// - Invalid email format or content
    /// - Database constraints are violated
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use dothtml_backend::database::Database;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     let message = db.insert_message(
    ///         "Hello, world!",
    ///         "user@example.com"
    ///     ).await?;
    ///     println!("Created message with ID: {}", message.id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn insert_message(
        &self, name: &str, email: &str, country_region: &str, phone_number: &str, company: &str, message: &str
    ) -> Result<Message, sqlx::Error> {
        let row = sqlx::query(r#"
            INSERT INTO messages (name, email, country_region, phone_number, company, message)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, email, country_region, phone_number, company, message, created_at, assigned_to, status
        "#)
        .bind(name)
        .bind(email)
        .bind(country_region)
        .bind(phone_number)
        .bind(company)
        .bind(message)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Message {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            country_region: row.get("country_region"),
            phone_number: row.get("phone_number"),
            company: row.get("company"),
            created_at: row.get("created_at"),
            assigned_to: row.get("assigned_to"),
            status: row.get("status"),
            message: row.get("message")
        })
    }
    
    /// Retrieves 20 pending messages from the database.
    /// 
    /// This method fetches the 20 most recent pending messages from the database, ordered by
    /// creation date (newest first) and shuffles them. It returns a vector of Message structs.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing a vector of `Message` instances on success,
    /// or a `sqlx::Error` on failure.
    /// 
    /// # Errors
    /// 
    /// This function returns an error if:
    /// - Database connection issues occur
    /// - Query execution fails
    /// - Row mapping errors occur
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use dothtml_backend::database::Database;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), sqlx::Error> {
    ///     let db = Database::new().await?;
    ///     let pending_messages = db.list_pending_messages().await?;
    ///     println!("Found {} pending messages", pending_messages.len());
    ///     for message in pending_messages {
    ///         println!("From: {} - Message: {}", message.name, message.message);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_pending_messages(&self) -> Result<Vec<PendingMessage>, sqlx::Error> {
        let mut rows = sqlx::query(r#"
            SELECT id, name, email, message
            FROM messages
            WHERE status = 'pending'
            ORDER BY created_at DESC
            LIMIT 20
        "#)
        .fetch_all(&self.pool)
        .await?;

        // Random shuffle of results
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        rows.shuffle(&mut rng);

        let messages = rows.into_iter().map(|row| PendingMessage {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            message: row.get("message"),
        }).collect();
        
        Ok(messages)
    }

    pub async fn get_message_by_id(&self, id: Uuid) -> Result<Message, sqlx::Error> {
        let row = sqlx::query(r#"
            SELECT id, name, email, country_region, phone_number, company, message, created_at, assigned_to, status
            FROM messages
            WHERE id = $1
        "#)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Message {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            country_region: row.get("country_region"),
            phone_number: row.get("phone_number"),
            company: row.get("company"),
            message: row.get("message"),
            created_at: row.get("created_at"),
            assigned_to: row.get("assigned_to"),
            status: row.get("status"),
        })
    }

}
