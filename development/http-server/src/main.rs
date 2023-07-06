
use std::env;
use bb8::Pool;
use bb8_postgres::tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;

type DataBasePool = Pool<PostgresConnectionManager<NoTls>>;
type DataBaseConnection<'a> = bb8::PooledConnection<'a, PostgresConnectionManager<NoTls>>;

trait SqlString {
	fn types() -> String{
		String::new()
	}

	fn values(&self) -> String{
		String::new()
	}
}

#[derive(Debug, Default)]
struct User{
	name: String,
	age: i32,
}

impl SqlString for User{
	fn types() -> String {
		format!("(id SERIAL PRIMARY KEY, name TEXT NOT NULL, age int NOT NULL)")
	}

	fn values(&self) -> String {
		format!("(name, age) VALUES (\'{}\', {})", self.name, self.age)
	}
}

#[derive(Debug)]
struct Table<'a, T>
where
	T: SqlString,
{
	name: String,
	pool: &'a DataBasePool,
	_type: std::marker::PhantomData<T>,
}

impl<'a, T> Table<'a, T>
where
	T: SqlString,
{
	async fn new<'b>(name: &str, pool: &'b DataBasePool) -> Result<Table<'b, T>, Box<dyn std::error::Error>>{
		let	connection: DataBaseConnection = pool.get().await?;

		// Create a connection string
		let create_string = format!(
			"CREATE TABLE IF NOT EXISTS {} {}",
			name, T::types()
		);

		connection.execute(&create_string, &[]).await?;

		println!("CREATED: {}", name);

		Ok(Table {
			    name: name.to_string(),
			    pool,
			    _type: std::marker::PhantomData,
			})
	}

	async fn insert(&self, addition: &impl SqlString) -> Result<(), Box<dyn std::error::Error>>{
		let connection: DataBaseConnection = self.pool.get().await?;

		let insert_command = format!(
			"INSERT INTO {}{}",
			self.name, addition.values()
		);

		println!("INSERTED: {} {}", self.name, addition.values());

		connection.execute(&insert_command, &[]).await?;

		Ok(())
	}

	async fn retrieve(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
		let connection: DataBaseConnection = self.pool.get().await?;

		let select_command = format!("SELECT * FROM {}", self.name);

		let rows = connection
			.query(&select_command, &[])
			.await?;

		let mut data = Vec::new();

		for row in rows {
			let name: String = row.try_get("name")?;
			let age: i32 = row.try_get("age")?;

			let user = User { name, age };
			data.push(user);
		}

		println!("DATA: {:?}", data);

		Ok(data)
	}

}

// Connect to database && create Pool
async fn create_pool() -> Result<DataBasePool, Box<dyn std::error::Error>> {
	// Retrieve environment variables for connection details
	let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
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
    let pool:DataBasePool = bb8::Pool::builder()
        .build(manager)
        .await?;

	println!("CREATED POOL");

    Ok(pool)
}

async fn display_table(pool: &DataBasePool, table: &str) -> Result<(), Box<dyn std::error::Error>> {
    let connection: DataBaseConnection = pool.get().await?;

    let rows = connection
        .query("SELECT * FROM another", &[])
        .await?;


    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let age: i32 = row.get("age");
        println!("id: {}, name: {}, age: {}", id, name, age);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
	// get_pool
	let	pool: DataBasePool = create_pool().await?;

	let users:Table<User> = Table::new("another", &pool).await?;

	let anna = User{
		name: "anna".to_string(),
		age: 90,
	};
	
	users.insert(&anna).await?;

	users.retrieve().await?;
	
	// users.display().await?;
	
	// display_table(&pool, "testing").await?;
	
    // // Spawn a task to process the connection in the background
	// let	connection: DataBaseConnection = pool.get().await?;
    // tokio::spawn(async move {
	// 	let _ = connection;
    // }).await?;

    Ok(())
}

