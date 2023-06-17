use chrono::{DateTime, Utc};
use log::info;
use sqlx::{
    migrate::MigrateDatabase, sqlite::SqliteQueryResult, Connection, Error, FromRow, Sqlite,
    SqliteConnection,
};

use crate::util::getcwd;

pub struct DB {
    pub conn: SqliteConnection,
}

#[derive(FromRow, Debug)]
pub struct Todo {
    id: i64,
    description: String,
}

impl Todo {
    pub fn new(id: i64, description: String) -> Todo {
        Todo { id, description }
    }
}

#[derive(FromRow, Debug)]
pub struct MoneyTransaction {
    pub id: i64,
    pub title: String,
    pub amount: f32,
    pub date: DateTime<Utc>,
}

impl MoneyTransaction {
    pub fn new(title: String, amount: f32, date: DateTime<Utc>) -> MoneyTransaction {
        MoneyTransaction {
            id: -1,
            title,
            amount,
            date,
        }
    }
}

impl DB {
    pub async fn new() -> DB {
        let conn = SqliteConnection::connect(&DB::get_db_url()).await.unwrap();
        DB { conn }
    }

    pub fn get_db_url() -> String {
        format!("sqlite:///{}/src/db.sqlite3", getcwd())
    }

    // https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
    pub async fn create_tables() {
        let db_url = &DB::get_db_url();
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            match Sqlite::create_database(db_url).await {
                Err(error) => panic!("error: {}", error),
                _ => {}
            }
        }
    }

    pub async fn run_migrations(&mut self) {
        let migrations = std::path::Path::new(&getcwd()).join("src/migrations");
        let migration_results = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&mut self.conn)
            .await;
        match migration_results {
            Err(error) => panic!("Migration error: {}", error),
            _ => {}
        }
    }

    pub async fn get_todos(&mut self) -> Vec<Todo> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos")
            .fetch_all(&mut self.conn)
            .await
            .unwrap()
    }

    pub async fn add_todo(&mut self, todo: &Todo) {
        sqlx::query("INSERT INTO todos (description) VALUES (?)")
            .bind(&todo.description)
            .execute(&mut self.conn)
            .await
            .unwrap();
    }

    pub async fn get_all_transactions(&mut self) -> Vec<MoneyTransaction> {
        sqlx::query_as::<_, MoneyTransaction>(
            "SELECT * FROM transactions ORDER BY julianday(date) DESC",
        )
        .fetch_all(&mut self.conn)
        .await
        .unwrap()
    }

    pub async fn query_transactions(
        &mut self,
        keyword: &str,
        min: u32,
        max: u32,
    ) -> Vec<MoneyTransaction> {
        sqlx::query_as::<_, MoneyTransaction>(&format!(
            r#"
            SELECT
                *
            FROM
                transactions
            WHERE
                title LIKE '%{}%'
                AND amount >= {}
                AND amount <= {}
            ORDER BY
                julianday(date) DESC
            "#,
            keyword, min, max
        ))
        .fetch_all(&mut self.conn)
        .await
        .unwrap()
    }

    pub async fn add_transaction(&mut self, transaction: &MoneyTransaction) {
        sqlx::query("INSERT INTO transactions (title, amount, date) VALUES (?, ?, ?)")
            .bind(&transaction.title)
            .bind(&transaction.amount)
            .bind(&transaction.date)
            .execute(&mut self.conn)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;

    use super::*;

    #[tokio::test]
    async fn test_transaction() {
        let mut db = DB::new().await;
        //         db.add_transaction(&Transaction {
        //             id: -1,
        //             title: "new".to_owned(),
        //             amount: 10.0,
        //             date: Utc::now(),
        //         })
        //         .await;
        //         let a = db.get_transactions_by_keyword("test").await;
        //         println!("{:?}", a);
    }
}
