use std::env;

use reqwest::{header, Client};
use types::{PromOrderStatus, PromOrdersResponse};

use crate::types::*;

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

    pub async fn order_refund(
        &self,
        ids: Vec<i32>,
    ) -> Result<PromOrderRefundResponse, Box<dyn std::error::Error>> {
        let payload = PromOrderRefundRequest { ids };
        let response = self
            .client
            .post(format!("{}/orders/refund", BASE_URL))
            .json(&payload)
            .send()
            .await?
            .json::<PromOrderRefundResponse>()
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

    pub async fn get_clients(
        &self,
        limit: Option<i32>,
        last_id: Option<i32>,
        search_term: Option<String>,
    ) -> Result<PromClientListResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}/clients/list", BASE_URL))
            .query(&[
                ("limit", limit.or(Some(0)).unwrap().to_string()),
                ("last_id", last_id.or(Some(0)).unwrap().to_string()),
                ("search_term", search_term.or(Some("".to_string())).unwrap()),
            ])
            .send()
            .await?
            .json::<PromClientListResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_client_by_id(
        &self,
        id: String,
    ) -> Result<PromClientByIdResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}/clients/{}", BASE_URL, id))
            .send()
            .await?
            .json::<PromClientByIdResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_messages(
        &self,
        status: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        limit: Option<i32>,
        last_id: Option<i32>,
    ) -> Result<PromMessagesListResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}/messages/list", BASE_URL))
            .query(&[
                ("status", status.or(Some("".to_string())).unwrap()),
                ("date_from", date_from.or(Some("".to_string())).unwrap()),
                ("date_to", date_to.or(Some("".to_string())).unwrap()),
                ("limit", limit.or(Some(0)).unwrap().to_string()),
                ("last_id", last_id.or(Some(0)).unwrap().to_string()),
            ])
            .send()
            .await?
            .json::<PromMessagesListResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_message_by_id(
        &self,
        id: String,
    ) -> Result<PromMessageByIdResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("{}/messages/{}", BASE_URL, id))
            .send()
            .await?
            .json::<PromMessageByIdResponse>()
            .await?;

        Ok(response)
    }

    pub async fn set_message_status(
        &self,
        ids: Vec<i32>,
        status: PromMessageStatus,
    ) -> Result<PromSetStatusMessageResponse, Box<dyn std::error::Error>> {
        let payload = PromSetStatusMessageRequest { ids, status };
        let response = self
            .client
            .post(format!("{}/messages/set_status", BASE_URL))
            .json(&payload)
            .send()
            .await?
            .json::<PromSetStatusMessageResponse>()
            .await?;

        Ok(response)
    }

    pub async fn reply_to_message(
        &self,
        id: i32,
        message: String,
    ) -> Result<PromMessageReplyResponse, Box<dyn std::error::Error>> {
        let payload = PromMessageReplyRequest { id, message };
        let response = self
            .client
            .post(format!("{}/messages/reply", BASE_URL))
            .json(&payload)
            .send()
            .await?
            .json::<PromMessageReplyResponse>()
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
