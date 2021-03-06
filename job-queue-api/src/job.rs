use serde::{Deserialize, Serialize};

use crate::error::QueueError;

/// A Job stored in the job queue
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    id: Option<usize>,
    #[serde(default)]
    status: Option<Status>,
    #[serde(rename = "type")]
    job_type: JobType,
}

/// The status of a Job
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Queued,
    InProgress,
    Concluded,
}

impl Default for Status {
    fn default() -> Self {
        Status::Queued
    }
}

/// The types of a Job
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobType {
    TimeCritical,
    NotTimeCritical,
}

/// The job queue
#[derive(Debug, Clone)]
pub struct JobQueue {
    queue: queue::Queue<Job>,
}

impl JobQueue {
    /// Creates an empty job queue
    pub fn new() -> Self {
        JobQueue {
            queue: queue::Queue::new(),
        }
    }

    /// Gets the Job with the specified ID
    pub fn get_job(&self, job_id: usize) -> Option<&Job> {
        self.queue.get(job_id)
    }

    /// Enqueues a new Job
    pub fn enqueue_job(&mut self, job: Job) -> usize {
        let id = self.queue.enqueue(job);
        let job = self.queue.get_mut(id).unwrap();
        // Update the ID of the job and status
        job.id = Some(id);
        job.status = Some(Status::Queued);
        id
    }

    /// Dequeues the job at the front of the queue
    pub fn dequeue_job(&mut self) -> Option<&mut Job> {
        self.queue.dequeue_mut().map(|job| {
            // Update the status to in progress since we are dequeuing
            job.status = Some(Status::InProgress);
            job
        })
    }

    /// Concludes an in-progress Job
    pub fn conclude_job(&mut self, job_id: usize) -> Result<(), QueueError> {
        match self.queue.get_mut(job_id) {
            Some(job) => {
                let status = job.status.clone();
                match status {
                    Some(Status::InProgress) => {
                        // The job was in progress so we can conclude it
                        job.status = Some(Status::Concluded);
                        Ok(())
                    }
                    // Otherwise this call is invalid
                    Some(Status::Concluded) => Err(QueueError::JobFinished),
                    Some(Status::Queued) => Err(QueueError::JobNotStarted),
                    _ => Err(QueueError::JobNotFound),
                }
            }
            _ => Err(QueueError::JobNotFound),
        }
    }
}
