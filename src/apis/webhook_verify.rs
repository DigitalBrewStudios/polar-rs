use crate::models::{
    WebhookOrderCreatedPayload, WebhookRefundCreatedPayload, WebhookRefundUpdatedPayload,
    WebhookSubscriptionActivePayload, WebhookSubscriptionCanceledPayload,
    WebhookSubscriptionCreatedPayload, WebhookSubscriptionRevokedPayload,
    WebhookSubscriptionUpdatedPayload,
};
use base64::{engine::general_purpose, Engine};
use http::HeaderMap;
use serde::Deserialize;
use standardwebhooks::{Webhook, WebhookError};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum WebhookPayload {
    OrderCreated(WebhookOrderCreatedPayload),
    RefundCreated(WebhookRefundCreatedPayload),
    RefundUpdated(WebhookRefundUpdatedPayload),
    SubscriptionActive(WebhookSubscriptionActivePayload),
    SubscriptionCanceled(WebhookSubscriptionCanceledPayload),
    SubscriptionRevoked(WebhookSubscriptionRevokedPayload),
    SubscriptionCreated(WebhookSubscriptionCreatedPayload),
    SubscriptionUpdated(WebhookSubscriptionUpdatedPayload),
}

pub fn validate_event(
    body: &[u8],
    headers: &HeaderMap,
    secret: &str,
) -> Result<WebhookPayload, WebhookError> {
    let base64_secret = general_purpose::STANDARD.encode(secret);
    let webhook = Webhook::new(base64_secret.as_str())?;
    webhook.verify(body, headers)?;

    let payload: WebhookPayload =
        serde_json::from_slice(body).map_err(|_| WebhookError::InvalidSignature)?;

    Ok(payload)
}
