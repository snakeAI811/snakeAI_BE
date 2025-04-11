mod twitter_job;

use anyhow::Context;
use database::{AppService, DatabasePool};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use utils::env::Env;

pub async fn run() -> Result<(), anyhow::Error> {
    let env = Env::init();
    let connection = DatabasePool::init(&env)
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));
    let db = Arc::new(connection);
    let service = Arc::new(AppService::init(&db, &env));

    let mut scheduler = serve(service, env).await?;

    tokio::signal::ctrl_c().await?;
    scheduler.shutdown().await?;

    Ok(())
}

pub async fn serve(service: Arc<AppService>, env: Env) -> anyhow::Result<JobScheduler> {
    let scheduler = JobScheduler::new()
        .await
        .context("Failed to create job scheduler")?;

    let job_service = service.clone();
    let job_env = env.clone();
    let schedule = env.twitter_job_schedule.clone();

    scheduler
        .add(
            Job::new_async(&schedule, move |_uuid, _l| {
                println!("twitter job run: {}", job_env.now());
                let service = job_service.clone();
                let env = job_env.clone();
                Box::pin(async move {
                    if let Err(err) = twitter_job::run(service, env).await {
                        println!("twitter job failed: {:?}", err);
                    }
                })
            })
            .context("Failed to create twitter job")?,
        )
        .await
        .context("Failed to add twitter job to scheduler")?;

    scheduler
        .start()
        .await
        .context("Failed to start scheduler")?;

    Ok(scheduler)
}
