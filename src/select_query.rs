use snowflake_connector_rs::SnowflakeSession;
use std::io;
use std::time::Instant;

pub async fn execute_select_query(session: &SnowflakeSession) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    // Prompt user for a SELECT query
    let mut query = String::new();
    println!("Enter your SELECT query:");
    io::stdin().read_line(&mut query)?;
 
    // Execute the query and display results
    let result = session.execute(query.trim()).await?;
    let duration = start.elapsed();

    /*
    println!("Query Result:");
    for row in result.iter() {
        // // Extract values by column names or indices
        // let name: Option<String> = row.get("NAME")?;
        // let number: Option<String> = row.get("NUMBER")?;
        // let value: Option<String> = row.get("VALUE")?;
 
        // // Display extracted values
        // println!(
        //     "Name: {}, Number: {}, Value: {}",
        //     name.unwrap_or_default(),
        //     number.unwrap_or_default(),
        //     value.unwrap_or_default()
        // );



        for column_name in row.column_names() {
            let value: Option<String> = row.get(column_name).ok();
            if let Some(val) = value {
                println!("{}: {}", column_name, val);
            }
        }
    }

    */
    println!("Time taken for execution is: {:?}",duration);
    println!("Select Query exexuted Successfully");
    Ok(())
}