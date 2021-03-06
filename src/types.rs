use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PromOrdersResponse {
    pub odrers: Vec<PromOrder>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromOrderByIdResponse {
    pub odrer: PromOrder,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromOrder {
    pub id: i32,
    pub date_created: String,
    pub client_first_name: String,
    pub client_second_name: String,
    pub client_last_name: String,
    pub client_id: i32,
    pub client_notes: String,
    pub products: Vec<PromProduct>,
    pub phone: String,
    pub email: String,
    pub price: String,
    pub delivery_option: PromDeliveryOption,
    pub delivery_provider_data: Option<PromDeliveryProviderData>,
    pub delivery_address: String,
    pub payment_option: PromPaymentOption,
    pub payment_data: Option<PromPaymentData>,
    pub status: PromOrderStatus,
    pub source: String,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
pub enum PromOrderStatus {
    pending,
    received,
    delivered,
    canceled,
    draft,
    paid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromProduct {
    pub id: i32,
    pub external_id: String,
    pub image: String,
    pub quantity: i32,
    pub price: String,
    pub url: String,
    pub name: String,
    pub total_price: String,
    pub measure_unit: String,
    pub sku: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromDeliveryOption {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromDeliveryProviderData {
    pub provider: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub sender_warehouse_id: Option<String>,
    pub recipient_warehouse_id: Option<String>,
    pub declaration_number: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromPaymentOption {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromPaymentData {
    #[serde(rename = "type")]
    pub type_: String,
    pub status: String,
    pub status_modified: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct PromChangeStatusRequest {
    pub status: String,
    pub ids: Vec<String>,
    pub cancellation_reason: Option<String>,
    pub cancellation_text: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromChangeStatusResponse {
    pub processed_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromSaveDeclarationIdRequest {
    pub order_id: i32,
    pub declaration_id: String,
    pub delivery_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromSaveDeclarationIdResponse {
    pub status: String,
    pub message: Option<String>,
    pub errors: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromOrderRefundRequest {
    pub ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromOrderRefundResponse {
    pub processed_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromClientListResponse {
    pub clients: Vec<PromClientCard>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromClientByIdResponse {
    pub client: PromClientCard,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromClientCard {
    pub id: i32,
    pub client_full_name: String,
    pub phones: Vec<String>,
    pub emails: Vec<String>,
    pub comment: Option<String>,
    pub orders_count: i32,
    pub total_payout: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromMessagesListResponse {
    pub messages: Vec<PromMessage>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromMessageByIdResponse {
    pub message: PromMessage,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
pub enum PromMessageStatus {
    unread,
    read,
    deleted,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromSetStatusMessageRequest {
    pub ids: Vec<i32>,
    pub status: PromMessageStatus,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromSetStatusMessageResponse {
    pub processed_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromMessageReplyRequest {
    pub id: i32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromMessageReplyResponse {
    pub processed_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PromMessage {
    pub id: i32,
    pub client_full_name: String,
    pub date_created: String,
    pub phone: String,
    pub message: String,
    pub subject: String,
    pub status: String,
    pub product_id: String,
}
