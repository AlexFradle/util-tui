use chrono::{DateTime, Utc};
use log::info;
use sqlx::{
    migrate::MigrateDatabase, sqlite::SqliteQueryResult, Connection, Error, FromRow, Row, Sqlite,
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
    pub details: String,
    pub date: DateTime<Utc>,
}

impl MoneyTransaction {
    pub fn new(
        title: String,
        amount: f32,
        details: String,
        date: DateTime<Utc>,
    ) -> MoneyTransaction {
        MoneyTransaction {
            id: -1,
            title,
            amount,
            details,
            date,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct Movie {
    pub id: i64,
    pub imdb_id: String,
    pub name: String,
    pub rating: f32,
    pub date_watched: Option<DateTime<Utc>>,
}

#[derive(FromRow, Debug)]
pub struct Series {
    pub id: i64,
    pub movie_id: i64,
    pub title: String,
    pub rating: f32,
}

#[derive(FromRow, Debug)]
pub struct Episode {
    pub id: i64,
    pub movie_id: i64,
    pub series_id: i64,
    pub title: String,
    pub rating: f32,
    pub date_watched: Option<DateTime<Utc>>,
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

    pub async fn get_num_of_transactions(&mut self, keyword: &str, min: f32, max: f32) -> u32 {
        sqlx::query(&format!(
            r#"
            SELECT
                COUNT(*) as count
            FROM
                transactions
            WHERE
                title LIKE '%{}%'
                AND amount >= {}
                AND amount <= {}
            "#,
            keyword, min, max
        ))
        .fetch_all(&mut self.conn)
        .await
        .unwrap()
        .get(0)
        .unwrap()
        .try_get("count")
        .unwrap()
    }

    pub async fn get_num_of_transaction_text_lines(
        &mut self,
        keyword: &str,
        min: f32,
        max: f32,
    ) -> u32 {
        let date_changes = sqlx::query(&format!(
            r#"
            SELECT
                COUNT(*)
            FROM
                transactions
            WHERE
                title LIKE '%{}%'
                AND amount >= {}
                AND amount <= {}
            GROUP BY
                date
            "#,
            keyword, min, max
        ))
        .fetch_all(&mut self.conn)
        .await
        .unwrap()
        .len() as u32;

        let num_of_transactions = self.get_num_of_transactions(keyword, min, max).await;

        num_of_transactions + date_changes
    }

    pub async fn query_transactions(
        &mut self,
        keyword: &str,
        min: f32,
        max: f32,
        limit: u32,
        offset: u32,
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
            LIMIT {} OFFSET {}
            "#,
            keyword, min, max, limit, offset
        ))
        .fetch_all(&mut self.conn)
        .await
        .unwrap()
    }

    pub async fn add_transaction(&mut self, transaction: &MoneyTransaction) {
        sqlx::query("INSERT INTO transactions (title, amount, details, date) VALUES (?, ?, ?, ?)")
            .bind(&transaction.title)
            .bind(&transaction.amount)
            .bind(&transaction.details)
            .bind(&transaction.date)
            .execute(&mut self.conn)
            .await
            .unwrap();
    }

    pub async fn get_all_movies(&mut self) -> Vec<MoneyTransaction> {
        sqlx::query_as::<_, MoneyTransaction>(
            "SELECT * FROM transactions ORDER BY julianday(date) DESC",
        )
        .fetch_all(&mut self.conn)
        .await
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use sqlx::Row;

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
        let keyword = "test";
        let min = 0;
        let max = 10000;
        let a = sqlx::query(&format!(
            r#"
            SELECT
                COUNT(*) as count
            FROM
                transactions
            WHERE
                title LIKE '%{}%'
                AND amount >= {}
                AND amount <= {}
            "#,
            keyword, min, max
        ))
        .fetch_all(&mut db.conn)
        .await
        .unwrap();
        let b: u32 = a.get(0).unwrap().try_get("count").unwrap();
        println!("{:?}", b);
    }
}
