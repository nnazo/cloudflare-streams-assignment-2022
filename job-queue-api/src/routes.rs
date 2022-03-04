use std::sync::{Arc, RwLock};

use crate::{
    error::QueueError,
    header::QueueConsumer,
    job::{Job, JobQueue},
};
use axum::{
    extract::{Extension, Path, TypedHeader},
    routing::{get, post},
    Json, Router,
};

/// Defines the routes for the application
pub fn routes() -> Router {
    // Create routes for each of the endpoints
    Router::new()
        .route("/jobs/:job_id", get(get_job))
        .route("/jobs/:job_id/conclude", get(conclude_job))
        .route("/jobs/enqueue", post(enqueue_job))
        .route("/jobs/dequeue", get(dequeue_job))
}

/// A GET endpoint for getting a Job with a given ID
pub async fn get_job(
    TypedHeader(_consumer): TypedHeader<QueueConsumer>,
    Path(job_id): Path<usize>,
    Extension(queue): Extension<Arc<RwLock<JobQueue>>>,
) -> Result<Json<Job>, QueueError> {
    Ok(Json(
        queue
            .read()
            .map_err(|_| QueueError::QueuePoisoned)?
            .get_job(job_id)
            .ok_or(QueueError::JobNotFound)?
            .clone(),
    ))
}

/// A POST endpoint for enqueueing a new Job
pub async fn enqueue_job(
    TypedHeader(_consumer): TypedHeader<QueueConsumer>,
    Extension(queue): Extension<Arc<RwLock<JobQueue>>>,
    Json(job): Json<Job>,
) -> Result<String, QueueError> {
    let mut queue = queue.write().map_err(|_| QueueError::QueuePoisoned)?;
    let id = queue.enqueue_job(job);
    Ok(format!("{}", id))
}

/// A GET endpoint for dequeueing the Job at the front of the queue
pub async fn dequeue_job(
    TypedHeader(_consumer): TypedHeader<QueueConsumer>,
    Extension(queue): Extension<Arc<RwLock<JobQueue>>>,
) -> Result<Json<Job>, QueueError> {
    let mut queue = queue.write().map_err(|_| QueueError::QueuePoisoned)?;
    let job = queue.dequeue_job().ok_or(QueueError::Empty)?.clone();
    Ok(Json(job))
}

/// A GET endpoint for concluding the specified Job in the URI path
pub async fn conclude_job(
    TypedHeader(_consumer): TypedHeader<QueueConsumer>,
    Path(job_id): Path<usize>,
    Extension(queue): Extension<Arc<RwLock<JobQueue>>>,
) -> Result<(), QueueError> {
    let mut queue = queue.write().map_err(|_| QueueError::QueuePoisoned)?;
    queue.conclude_job(job_id)
}
