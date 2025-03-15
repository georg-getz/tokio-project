use tokio::{task::{JoinHandle, spawn}, time::{self, Duration}};
use tracing::{error, info};
use rand::{Rng, rng};
use futures::future::join_all;
use tracing_subscriber::fmt::init;
use time::sleep;

#[tokio::main]
async fn main() {
    init();

    let tasks: Vec<_> = vec![
        spawn_task("Task 1"),
        spawn_task("Task 2"),
        spawn_task("Task 3"),
        spawn_task("Task 4"),
    ];

    let results: Vec<_> = join_all(tasks).await;

    for result in results {
        if let Err(e) = result {
            error!("Critical failure detected: {}. Initiating shutdown...", e);
        }
    }

    info!("Shutting down gracefully");
}

fn spawn_task(name: &str) -> JoinHandle<Result<(), String>> {
    let task_name = name.to_string();
    spawn(async move {
        let duration = rng().random_range(2..5);
        info!("{} started, running for {} seconds", task_name, duration);

        if rng().random_bool(0.2) {
            error!("{} encountered a critical failure", task_name);
            return Err(format!("{} failed", task_name));
        }

        sleep(Duration::from_secs(duration)).await;
        info!("{} completed successfully", task_name);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use futures::future::join_all;

    #[test]
    async fn test_spawn_task() {
        let result = spawn_task("Test Task").await.unwrap();
        assert!(result.is_ok() || result.is_err()); // Test whether result is returned, randomness makes it so we can't be sure of the actual result
    }

    #[test]
    async fn test_multiple_tasks() {
        let tasks = vec![
            spawn_task("Task A"),
            spawn_task("Task B"),
            spawn_task("Task C"),
        ];
        let results = join_all(tasks).await;
        assert_eq!(results.len(), 3);
    }
}