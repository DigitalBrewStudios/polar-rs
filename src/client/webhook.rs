use base64::{engine::general_purpose, Engine};
use http::HeaderMap;
use serde::Deserialize;
use standardwebhooks::{Webhook, WebhookError};

use crate::models::components::{
    webhook_order_created_payload::WebhookOrderCreatedPayload,
    webhook_refund_created_payload::WebhookRefundCreatedPayload,
    webhook_refund_updated_payload::WebhookRefundUpdatedPayload,
    webhook_subscription_active_payload::WebhookSubscriptionActivePayload,
    webhook_subscription_canceled_payload::WebhookSubscriptionCanceledPayload,
    webhook_subscription_created_payload::WebhookSubscriptionCreatedPayload,
    webhook_subscription_revoked_payload::WebhookSubscriptionRevokedPayload,
    webhook_subscription_updated_payload::WebhookSubscriptionUpdatedPayload,
};

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
