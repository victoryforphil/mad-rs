use log::*;
use mad_core::geo::GridRegion;
use mad_engine::runner::ECSSystemRunner;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logger
    pretty_env_logger::init();
    
    info!("Starting MAD Server...");
    
    // Create the ECS system runner
    let region = GridRegion::new(0);
    let mut runner = ECSSystemRunner::new(region);
    
    info!("ECS System Runner initialized");
    
    // Spawn the ECS runner loop as a tokio task
    let ecs_handle = tokio::spawn(async move {
        info!("Starting ECS loop...");
        
        loop {
            // Execute one tick of all systems
            match runner.execute() {
                Ok(_) => {
                    trace!("ECS tick completed successfully");
                }
                Err(e) => {
                    error!("Error executing ECS systems: {:?}", e);
                    // Continue running despite errors
                }
            }
            
            // Optional: Add a small yield to prevent blocking the executor
            tokio::task::yield_now().await;
        }
    });
    
    info!("MAD Server is running. Press Ctrl+C to stop.");
    
    // Wait for the ECS task (which runs indefinitely)
    match ecs_handle.await {
        Ok(_) => info!("ECS task completed"),
        Err(e) => error!("ECS task panicked: {:?}", e),
    }
    
    Ok(())
}
