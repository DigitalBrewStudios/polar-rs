use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCustomerUpdatedPayload {
    #[serde(rename = "type")]
    pub webhook_type: String,
    pub timestamp: DateTime<Utc>,
    pub data: CustomerUpdatedData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerUpdatedData {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub metadata: Value,
    pub email: Option<String>,
    pub email_verified: bool,
    pub name: Option<String>,
    pub billing_address: Option<Value>,
    pub tax_id: Option<Vec<Value>>,
    pub organization_id: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
    pub avatar_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_customer_updated_payload_from_json() {
        let body_json = r#"
    {
    "type": "customer.state_changed",
    "timestamp": "2025-11-01T01:29:35.523473Z",
    "data": {
        "id": "a7c90cb0-6168-4678-a016-7ae09f9d7531",
        "created_at": "2025-11-01T01:29:35.451636Z",
        "modified_at": null,
        "metadata": {},
        "external_id": null,
        "email": "fiahowepfgyhveiwoapyofh8uaiw3eo4rfyhaha4uth4kfa@gmail.com",
        "email_verified": false,
        "name": "Joe Shmo",
        "billing_address": null,
        "tax_id": null,
        "organization_id": "7eae7216-658f-4efd-afe3-1c6ad07d58e5",
        "deleted_at": null,
        "active_subscriptions": [],
        "granted_benefits": [],
        "active_meters": [],
        "avatar_url": "https://www.gravatar.com/avatar/02bfef3af593e88ab64eaf102e9df96a1f812393cc6d84a28853d51d8d0a3a05?d=404"
        }
    }
    "#;

        let body: WebhookCustomerUpdatedPayload = serde_json::from_str(body_json).unwrap();
        println!("{body:?}")
    }
}
