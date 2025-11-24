mod connection;
mod matchmaking;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = "0.0.0.0:9001";
    let listener = TcpListener::bind(address).await?;
    println!("Server running on ws://{}", address);

    // Shared state initialization using the new helper functions
    let connections = connection::new_connection_map();
    let matches = connection::new_match_map();
    let waiting_queue = connection::new_waiting_queue();

    // Start matchmaking background task
    let matchmaker = matchmaking::MatchmakingSystem::new(
        connections.clone(),
        matches.clone(),
        waiting_queue.clone(),
    );
    tokio::spawn(async move {
        matchmaker.run().await;
    });

    // Main connection loop
    while let Ok((stream, _)) = listener.accept().await {
        let connections = connections.clone();
        let matches = matches.clone();
        let waiting_queue = waiting_queue.clone();

        tokio::spawn(async move {
            if let Err(e) =
                connection::handle_connection(stream, connections, matches, waiting_queue).await
            {
                eprintln!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}
