use bb8::Pool;
use bb8_postgres::tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;
use std::marker::PhantomData;

type DataBasePool = Pool<PostgresConnectionManager<NoTls>>;
type DataBaseConnection<'a> = bb8::PooledConnection<'a, PostgresConnectionManager<NoTls>>;

// TO DO:
/*
    https:server -> ActixWeb to asynchronously recieve and send data

    postgres ->
    > debug function to see content of db			X Done
    > retrieve function to specific data
    > make it secure

    > storing one DataBaseConnection from the pool in each table right now
    ==> should there be more connections for each websocket, or a different approach
    ==> also i could store mutible structs in one table using more columns, but this needs to be implemented

*/

// just a example struct
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: i32,
}

#[derive(Debug)]
struct Table<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    name: String,
    connection: DataBaseConnection<'a>,
    _type: PhantomData<T>,
}

impl<'a, T> Table<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    async fn new(
        name: &str,
        pool: &'a DataBasePool,
    ) -> Result<Table<'a, T>, Box<dyn std::error::Error>> {
        let table = Table {
            name: name.to_string(),
            connection: pool.get().await?,
            _type: std::marker::PhantomData,
        };

        let create_string = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" (data TEXT NOT NULL)",
            name
        );
        table.connection.execute(&create_string, &[]).await?;
        Ok(table)
    }

    async fn insert(&self, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        let insert_command = format!("INSERT INTO \"{}\"(data) VALUES ($1)", self.name);
        let serialized = serde_json::to_string(&data)?;
        self.connection
            .execute(&insert_command, &[&serialized])
            .await?;

        println!("INSERTED {} INTO {}", serialized, self.name);

        Ok(())
    }

    async fn get_table(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let select_command = format!("SELECT * FROM {}", self.name);
        let rows = self.connection.query(&select_command, &[]).await?;

        let mut table: Vec<T> = Vec::new();

        for row in rows {
            let data: String = row.try_get("data")?;
            let deserialized: T = serde_json::from_str(&data)?;
            table.push(deserialized);
        }

        Ok(table)
    }

    async fn display(&self) {
        let select_command = format!("SELECT * FROM {}", self.name);
        let rows = self.connection.query(&select_command, &[]).await.unwrap();

        println!("/--------------------------\\");
        println!("|         {}", self.name);
        println!("\\--------------------------/");

        for row in rows {
            let data: String = row.try_get("data").unwrap();
            println!("|> {}", data);
        }
        println!("\\--------------------------/");
    }
}

// Connect to database && create Pool
async fn create_pool() -> Result<DataBasePool, Box<dyn std::error::Error>> {
    // Retrieve environment variables for connection details
    let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
    println!("{}", host);
    let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    let dbname = env::var("POSTGRES_DB").expect("POSTGRES_DB not set");

    // Create a connection string
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, dbname
    );

    // Create a connection manager
    let manager = PostgresConnectionManager::new_from_stringlike(connection_string, NoTls)?;

    // Create a connection pool
    let pool: DataBasePool = bb8::Pool::builder().build(manager).await?;

    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get_pool
    let pool: DataBasePool = create_pool().await?;

    let users: Table<User> = Table::new("another", &pool).await?;

    let anna = User {
        name: "".to_string(),
        age: 1,
    };

    users.insert(&anna).await?;
    users.display().await;

    // let content = users.get_table().await?;
    // println!("TABLE: {:?}", content);

    // // Spawn a task to process the connection in the background
    // let	connection: DataBaseConnection = pool.get().await?;
    // tokio::spawn(async move {
    // 	let _ = connection;
    // }).await?;

    Ok(())
}
