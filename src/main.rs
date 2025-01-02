
mod upload_csv;
mod select_query;
mod execute_query;
 
use snowflake_connector_rs::{SnowflakeClient, SnowflakeAuthMethod, SnowflakeClientConfig};
use std::io;
use std::env;
use dotenv::dotenv;
use std::time::Duration;



 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    //Retrive Environment Variables
    let user = env::var("SNOWFLAKE_USER")?;
    let password = env::var("SNOWFLAKE_PASSWORD")?;
    let account = env::var("SNOWFLAKE_ACCOUNT")?;
    let role = env::var("SNOWFLAKE_ROLE").ok();
    let warehouse = env::var("SNOWFLAKE_WAREHOUSE").ok();
    let database = env::var("SNOWFLAKE_DATABASE").ok();
    let schema = env::var("SNOWFLAKE_SCHEMA").ok();
    let timeout = env::var("SNOWFLAKE_TIMEOUT")?.parse::<u64>()?;

    // Initialize Snowflake client
    let client = SnowflakeClient::new(
        &user,
        SnowflakeAuthMethod::Password(password),
        SnowflakeClientConfig {
            account,
            role,
            warehouse,
            database,
            schema,
            timeout: Some(Duration::from_secs(timeout)),
        },
    )?;
    let session = client.create_session().await?;
 
    // Menu-driven interface options. User should select one among the following options
    loop {
        println!("Choose an option:");
        println!("1. Upload CSV file to Snowflake");
        println!("2. Execute SELECT query");
        println!("3. Execute given query");
        println!("4. Exit");
 
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
 
        match choice.trim() {
            "1" => upload_csv::upload_csv_to_snowflake(&session).await?,
            "2" => select_query::execute_select_query(&session).await?,
            "3" => execute_query::execute_req_query(&session).await?,
            "4" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
    Ok(())
}