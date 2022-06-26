use std::fmt::format;
use std::env;
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let username: String = match env::var("PSQL_USERNAME") {
        Ok(val) => val,
        Err(err) => panic!("could not find PSQL_USERNAME: {}", err)
    };

    let password: String = match env::var("PSQL_PASSWORD") {
        Ok(val) => val,
        Err(err) => panic!("could not find PSQL_PASSWORD: {}", err)
    };

    let dbname: String = match env::var("PSQL_DBNAME") {
        Ok(val) => val,
        Err(err) => panic!("could not find PSQL_DBNAME: {}", err)
    };

    let dsn: String = format!("postgresql://{username}:{password}@localhost/{dbname}",
        username=username,
        password=password,
        dbname=dbname
    );

    let dsn: &str = &dsn;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    println!("You row: {}", row.0);

    Ok(())
}
