use crate::config::Config;
use crate::page::Paginated;

struct Extractor<T: Paginated> {
    config: Config<T>,
}

impl<P: Paginated> Extractor<P> {
    async fn run(&self) {
        let source_config = &self.config.source;

        let jobs = (0..source_config.size().await as u32)
            .map(|page_id| async move { (*source_config).page(page_id).await })
            .collect::<Vec<_>>()
            .chunks(self.config.job.chunk_size as usize);

        todo!("Run jobs in parallel")
    }
}
