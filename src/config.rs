use crate::page::Paginated;

pub struct Config<T: Paginated> {
    pub source: T,
    pub job: JobsConfig,
}

pub struct JobsConfig {
    pub parallels_jobs: u32,
    pub chunk_size: u32,
}
