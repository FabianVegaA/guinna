use crate::page::Paginated;

struct Job<T, P>
where
    P: Paginated,
    T: Iterator<Item = P>,
{
    pages: T,
}

impl<P: Paginated, T: Iterator<Item = P>> Job<T, P> {
    async fn run(&self) {
        todo!("Run job async")
    }
}
