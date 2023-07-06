
use std::env;
use bb8::Pool;
use bb8_postgres::tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;

type DataBasePool = Pool<PostgresConnectionManager<NoTls>>;
type DataBaseConnection<'a> = bb8::PooledConnection<'a, PostgresConnectionManager<NoTls>>;

trait SqlString{
	fn values(&self) -> String{
		String::new()
	}

	fn types(&self) -> String{
		String::new()
	}

	fn elems(&self) -> Vec<String> {
		Vec::new()
	}
}

#[derive(Debug)]
struct User{
	name: String,
	age: i32,
}

impl SqlString for User{
	fn values(&self) -> String {
		format!("(name, age) VALUES (\'{}\', {})", self.name, self.age)
	}

	fn types(&self) -> String {
		format!("(id SERIAL PRIMARY KEY, name TEXT NOT NULL, age int NOT NULL)")
	}

	fn elems(&self) -> Vec<String> {
		vec!("id".to_string(), "name".to_string(), "age".to_string())
	}
}

#[derive(Debug)]
struct Table<'a>{
	name: String,
	pool: &'a DataBasePool,
	elems: Vec<String>,
}

impl Table<'_>
{
	async fn new<'a>(name: &str, content: &impl SqlString, pool: &'a DataBasePool) -> Result<Table<'a>, Box<dyn std::error::Error>>{
		let	connection: DataBaseConnection = pool.get().await?;

		// Create a connection string
		let create_string = format!(
			"CREATE TABLE IF NOT EXISTS {} {}",
			name, content.types()
		);

		connection.execute(&create_string, &[]).await?;

		println!("CREATE: {}", name);

        Ok(Table { name: name.to_string(), pool, elems: content.elems()})
    }

	async fn insert(&self, addition: &impl SqlString) -> Result<(), Box<dyn std::error::Error>>{
		let connection: DataBaseConnection = self.pool.get().await?;

		let insert_command = format!(
			"INSERT INTO {}{}",
			self.name, addition.values()
		);

		println!("INSERT: {} {}", self.name, addition.values());

		connection.execute(&insert_command, &[]).await?;

		Ok(())
	}

	async fn retrieve_data(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
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

	async fn display(&self) -> Result<(), Box<dyn std::error::Error>> {
		let connection: DataBaseConnection = self.pool.get().await?;

		let select_command = format!("SELECT * FROM {}", self.name);

		let rows = connection
			.query(&select_command, &[])
			.await?;

		println!("did select table");

		for row in rows{
			
		}
	
		Ok(())
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

	let anna = User{
		name: "santa".to_string(),
		age:10000,
	};
	
	let users = Table::new("another", &anna, &pool).await?;
	
	users.insert(&anna).await?;

	users.retrieve_data().await?;
	// users.display().await?;
	
	// display_table(&pool, "testing").await?;
	
    // // Spawn a task to process the connection in the background
	// let	connection: DataBaseConnection = pool.get().await?;
    // tokio::spawn(async move {
	// 	let _ = connection;
    // }).await?;

    Ok(())
}

