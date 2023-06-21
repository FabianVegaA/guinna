use async_trait::async_trait;
use futures::TryFutureExt;
use reqwest::{header::HeaderMap, Response, Result};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Paginated {
    async fn size(&self) -> u32;
    async fn page(&self, page: u32) -> Result<Response>;
}

#[derive(Debug, Clone)]
pub struct SimplePaginated {
    pub base_url: String,
    pub endpoint: String,
    pub headers: HeaderMap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    data: serde_json::Value,
    page: u32,
    size: u32,
}

#[async_trait]
impl Paginated for SimplePaginated {
    async fn size(&self) -> u32 {
        self.page(0)
            .and_then(|r| r.json::<Page>())
            .await
            .unwrap()
            .size
    }

    async fn page(&self, page: u32) -> Result<Response> {
        let url = format!("{}{}/{}", self.base_url, self.endpoint, page);
        let client = reqwest::Client::new();
        client.get(&url).headers(self.headers.clone()).send().await
    }
}

impl SimplePaginated {
    pub async fn data_from_page(&self, page: u32) -> serde_json::Value {
        self.page(page)
            .and_then(|r| r.json::<Page>())
            .await
            .unwrap()
            .data
    }
}
