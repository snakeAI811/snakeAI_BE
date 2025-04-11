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
    serve(service, env).await?;
    Ok(())
}

pub async fn serve(service: Arc<AppService>, env: Env) -> anyhow::Result<JobScheduler> {
    let scheduler = JobScheduler::new()
        .await
        .context("Failed to create job scheduler")?;

    scheduler
        .add(
            Job::new_async(env.twitter_job_schedule.clone(), move |_uuid, _l| {
                println!("twitter job run: {}", env.now());
                let service0 = service.clone();
                let env0 = env.clone();
                Box::pin(async move {
                    if let Err(err) = twitter_job::run(service0, env0).await {
                        println!("twitter job failed: {:?}", err);
                    }
                })
            })
            .context("Failed to create twitter job")?,
        )
        .await
        .context("Failed to add twitter job to scheduler")?;

    Ok(scheduler)
}
