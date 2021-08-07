use std::env;

use reqwest::{header, Client};
use types::{PromOrderStatus, PromOrdersResponse};

use crate::types::{
    PromChangeStatusRequest, PromChangeStatusResponse, PromOrderByIdResponse,
    PromSaveDeclarationIdRequest,
};

mod types;

const BASE_URL: &str = "https://my.prom.ua/api/v1";

pub struct Prom {
    client: Client,
}

impl Default for Prom {
    fn default() -> Self {
        let api_key = env::var("PROM_API_KEY").unwrap();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(format!("Bearer {}", api_key).as_str()).unwrap(),
        );
        let client = Client::builder()
            .default_headers(headers)
            .https_only(true)
            .build()
            .unwrap();
        Prom { client }
    }
}

impl Prom {
    pub async fn get_orders(
        &self,
        status: Option<PromOrderStatus>,
        limit: Option<i32>,
    ) -> Result<PromOrdersResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}/orders/list", BASE_URL))
            .query(&("status", status.or(Some(PromOrderStatus::pending)).unwrap()))
            .query(&[("limit", limit.or(Some(0)).unwrap().to_string())])
            .send()
            .await?
            .json::<PromOrdersResponse>()
            .await?;
        Ok(response)
    }

    pub async fn get_order_by_id(
        &self,
        id: String,
    ) -> Result<PromOrderByIdResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}/orders/{}", BASE_URL, id))
            .send()
            .await?
            .json::<PromOrderByIdResponse>()
            .await?;

        Ok(response)
    }

    pub async fn set_order_status(
        &self,
        status: String,
        ids: Vec<String>,
        cancellation_reason: Option<String>,
        cancellation_text: Option<String>,
    ) -> Result<PromChangeStatusResponse, Box<dyn std::error::Error>> {
        let payload = PromChangeStatusRequest {
            status,
            ids,
            cancellation_reason,
            cancellation_text,
        };
        let response = self
            .client
            .post(format!("{}/orders/set_status", BASE_URL))
            .json(&payload)
            .send()
            .await?
            .json::<PromChangeStatusResponse>()
            .await?;

        Ok(response)
    }

    pub async fn save_declaration_id(
        &self,
        order_id: i32,
        declaration_id: String,
        delivery_type: String,
    ) -> Result<PromOrderByIdResponse, Box<dyn std::error::Error>> {
        let payload = PromSaveDeclarationIdRequest {
            order_id,
            declaration_id,
            delivery_type,
        };
        let response = self
            .client
            .post(format!("{}/delivery/save_declaration_id", BASE_URL))
            .json(&payload)
            .send()
            .await?
            .json::<PromOrderByIdResponse>()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
