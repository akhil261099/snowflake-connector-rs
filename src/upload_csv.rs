use snowflake_connector_rs::SnowflakeSession;
use std::fs::File;
use csv::ReaderBuilder;
use std::time::Instant;
use std::io;
// use crate::io;
// use tokio::io;cls
pub async fn upload_csv_to_snowflake(session: &SnowflakeSession) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    // Create a table in Snowflake
    // let create_table_sql = "
    //     CREATE OR REPLACE TABLE MAIN_TABLE (
    //         SepalLength NUMBER(3,2),
    //         SepalWidth NUMBER(3,2),
    //         PetalLength NUMBER(3,2),
    //         PetalWidth NUMBER(3,2),
    //         Species STRING
    //     );
    // ";
    println!("Enter your create_table Query");
    let mut create_table_sql = String::new();
    io::stdin().read_line(&mut create_table_sql)?;

    session.execute(create_table_sql).await?;
    println!("Table created successfully.");
 
    // File path to the CSV file
    let file_path = r"E:\snowflake-connector-rs\connector\src\iris_dataset.csv";
    let file = File::open(file_path)?;
 
    // Read CSV file and insert records into the Snowflake table
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    // INSERT INTO MAIN_TABLE (SepalLength,SepalWidth,PetalLength,PetalWidth,Species)
    println!("Enter the Insert Query");
    let mut insert_query:String = String::new();
    io::stdin().read_line(&mut insert_query)?;
    let insert_query = insert_query.trim(); 


    for result in rdr.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        
 
        // Format the row values and execute the INSERT query
        let values_str = row.iter()
            .map(|field| format!("'{}'", field.replace("'", "''")))
            .collect::<Vec<String>>()
            .join(", ");

        
        let insert_sql = format!("{} VALUES ({})", insert_query,values_str);
        println!("{:?}",insert_sql);
        session.execute(insert_sql).await?;
    }
    let duration = start.elapsed();
    println!("Time taken to execute the upload_csv Query is :{:?}",duration);
    println!("CSV file uploaded successfully.");
    Ok(())
}


// INSERT INTO MAIN_TABLE (SepalLength,SepalWidth,PetalLength,PetalWidth,Species)