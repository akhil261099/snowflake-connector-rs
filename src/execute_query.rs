use snowflake_connector_rs::SnowflakeSession;
use std::io;
use std::time::Instant;

pub async fn execute_req_query(session: &SnowflakeSession) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your required query:");
    let start = Instant::now();
    // Prompt user for a custom query
    let mut query = String::new();
    
    io::stdin().read_line(&mut query)?;
 
    // Execute the query
    session.execute(query.trim()).await?;
    let duration = start.elapsed();
    println!("Query executed successfully.");
    println!("time taken for execution is :{:?}",duration);
    Ok(())
}