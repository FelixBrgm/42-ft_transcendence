use postgres::{Client, Error, NoTls};
use std::env;

fn main() -> Result<(), Error> {

    let mut client = Client::connect(
        "postgresql://theOperator:IHaveAllThePower@postgres_db:5432/postgres",
        NoTls,
    )?;

	println!("hey na:)");
	
    Ok(())
}