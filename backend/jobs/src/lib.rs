mod solana_job;
mod twitter_job;
mod reward_sync_job;

use anyhow::Context;
use database::{AppService, DatabasePool};
use std::sync::Arc;
use tokio::sync::Mutex;
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

    let is_twitter_job_running = Arc::new(Mutex::new(false));
    let is_solana_job_running = Arc::new(Mutex::new(false));
    let is_reward_sync_job_running = Arc::new(Mutex::new(false));

    let job_service = service.clone();
    let job_env = env.clone();
    let job_is_running = is_twitter_job_running.clone();
    let schedule = env.twitter_job_schedule.clone();

    scheduler
        .add(
            Job::new_async(&schedule, move |_uuid, _l| {
                println!("twitter job run: {}", job_env.now());
                let service = job_service.clone();
                let env = job_env.clone();
                let running_flag = job_is_running.clone();
                Box::pin(async move {
                    let mut running = running_flag.lock().await;
                    if *running == false {
                        *running = true;
                        drop(running);
                        if let Err(err) = twitter_job::run(service, env).await {
                            println!("twitter job failed: {:?}", err);
                        }
                        let mut running = running_flag.lock().await;
                        *running = false;
                        drop(running);
                    } else {
                        println!("twitter_job::run() already in progress, skipping");
                    }
                })
            })
            .context("Failed to create twitter job")?,
        )
        .await
        .context("Failed to add twitter job to scheduler")?;

    let job_service = service.clone();
    let job_env = env.clone();
    let job_is_running = is_solana_job_running.clone();
    let schedule = env.solana_job_schedule.clone();

    scheduler
        .add(
            Job::new_async(&schedule, move |_uuid, _l| {
                println!("solana job run: {}", job_env.now());
                let service = job_service.clone();
                let env = job_env.clone();
                let running_flag = job_is_running.clone();
                Box::pin(async move {
                    let mut running = running_flag.lock().await;
                    if *running == false {
                        *running = true;
                        drop(running);
                        if let Err(err) = solana_job::run(service, env).await {
                            println!("twitter job failed: {:?}", err);
                        }
                        let mut running = running_flag.lock().await;
                        *running = false;
                        drop(running);
                    } else {
                        println!("solana_job::run() already in progress, skipping");
                    }
                })
            })
            .context("Failed to create solana job")?,
        )
        .await
        .context("Failed to add solana job to scheduler")?;

    // Add reward sync job
    let job_service = service.clone();
    let job_env = env.clone();
    let job_is_running = is_reward_sync_job_running.clone();
    // let schedule = env.reward_sync_job_schedule.clone();

    // scheduler
    //     .add(
    //         Job::new_async(&schedule, move |_uuid, _l| {
    //             println!("reward sync job run: {}", job_env.now());
    //             let service = job_service.clone();
    //             let env = job_env.clone();
    //             let running_flag = job_is_running.clone();
    //             Box::pin(async move {
    //                 let mut running = running_flag.lock().await;
    //                 if *running == false {
    //                     *running = true;
    //                     drop(running);
    //                     if let Err(err) = reward_sync_job::run(service, env).await {
    //                         println!("reward sync job failed: {:?}", err);
    //                     }
    //                     let mut running = running_flag.lock().await;
    //                     *running = false;
    //                     drop(running);
    //                 } else {
    //                     println!("reward_sync_job::run() already in progress, skipping");
    //                 }
    //             })
    //         })
    //         .context("Failed to create reward sync job")?,
    //     )
    //     .await
    //     .context("Failed to add reward sync job to scheduler")?;

    scheduler
        .start()
        .await
        .context("Failed to start scheduler")?;

    Ok(scheduler)
}
