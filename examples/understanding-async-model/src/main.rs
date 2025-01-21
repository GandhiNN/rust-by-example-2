use futures::future::join_all;
use std::collections::HashMap;

// This is a basic async function that simulates a network request
async fn fetch_user_data(user_id: u64) -> Result<String, Box<dyn std::error::Error>> {
    // Simulate network delay
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Simulate fetching user ata
    Ok(format!("User data for ID: {}", user_id))
}

// Simulating a database query
async fn query_database(query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Simulate database access time
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    Ok(vec![format!("Result for query: {}", query)])
}

// Simulating an API call
async fn call_external_api(endpoint: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Simulate network latency
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
    Ok(vec![format!("API response from {}", endpoint)])
}

// A more complex function that coordinates multiple async operations
async fn process_user_request(
    user_id: u64,
    queries: Vec<String>,
) -> Result<HashMap<String, Vec<Vec<String>>>, Box<dyn std::error::Error>> {
    // Create a vector of futures for our database queries
    let db_futures: Vec<_> = queries.iter().map(|q| query_database(q)).collect();

    // Execute all database queries concurrently
    let db_results = join_all(db_futures).await;

    // While database queries are running, also call the external API
    let api_future_input = format!("/users/{}/preferences", user_id);
    let api_future = call_external_api(&api_future_input);

    // Wait for the API call to complete
    let api_result = api_future.await?;

    // Process all results
    let mut results = HashMap::new();
    results.insert("api_data".to_string(), vec![api_result]);

    // Collect database results
    let db_data: Result<Vec<_>, _> = db_results.into_iter().collect();

    results.insert("db_data".to_string(), db_data?);

    Ok(results)
}

#[tokio::main]
async fn main() {
    let queries = vec![
        "SELECT * FROM users".to_string(),
        "SELECT * FROM preferences".to_string(),
    ];
    match process_user_request(42, queries).await {
        Ok(results) => {
            println!("{:#?}", results);
            println!("API Results: {:?}", results.get("api_data").unwrap());
            println!("DB Results: {:?}", results.get("db_data").unwrap());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
