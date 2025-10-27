use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    benefit_articles::BenefitArticles,
    benefit_base::BenefitBase,
    product_media_file_read::{ProductMediaFileRead, ProductMediaFileReadInput},
    product_price::ProductPrice,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub trial_interval: Option<String>, // TODO: make this a strenum
    pub trial_interval_count: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub recurring_interval: Option<String>, // TODO: make this a strenum
    pub recurring_interval_count: Option<i64>,
    pub is_recurring: bool,
    pub is_archived: bool,
    pub organization_id: String,
    pub metadata: Value,
    pub prices: Vec<Value>,   // TODO: fix this
    pub benefits: Vec<Value>, // TODO: fix this
    pub medias: Vec<Value>,
    pub attached_custom_fields: Vec<Value>, // TODO: what type should this be???
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductInput {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_recurring: bool,
    pub is_archived: bool,
    pub organization_id: String,
    pub prices: Vec<ProductPrice>,
    pub benefits: Vec<ProductBenefit>,
    pub medias: Vec<ProductMediaFileReadInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductBenefit {
    Base(BenefitBase),
    Articles(BenefitArticles),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_from_json() {
        let product_json = r#"    {
      "id": "90a4d525-dfd5-457e-bb52-4bec06b5a022",
      "created_at": "2025-10-26T23:25:54.051586Z",
      "modified_at": "2025-10-26T23:25:54.329184Z",
      "trial_interval": null,
      "trial_interval_count": null,
      "name": "Fancy Stuff",
      "description": "buy me buy me buy me",
      "recurring_interval": "month",
      "recurring_interval_count": 1,
      "is_recurring": true,
      "is_archived": false,
      "organization_id": "7eae7216-658f-4efd-afe3-1c6ad07d58e5",
      "metadata": {},
      "prices": [
        {
          "created_at": "2025-10-26T23:25:54.055799Z",
          "modified_at": null,
          "id": "8160d664-c125-46be-8b6c-eb47b900265e",
          "amount_type": "seat_based",
          "is_archived": false,
          "product_id": "90a4d525-dfd5-457e-bb52-4bec06b5a022",
          "type": "recurring",
          "recurring_interval": "month",
          "price_currency": "usd",
          "seat_tiers": {
            "tiers": [
              {
                "min_seats": 1,
                "max_seats": 4,
                "price_per_seat": 1000
              },
              {
                "min_seats": 5,
                "max_seats": 9,
                "price_per_seat": 900
              },
              {
                "min_seats": 10,
                "max_seats": null,
                "price_per_seat": 800
              }
            ]
          },
          "price_per_seat": 1000
        }
      ],
      "benefits": [
        {
          "id": "3acce7f8-89f6-4df6-a096-82a54dde9a3e",
          "created_at": "2025-10-26T23:23:34.694132Z",
          "modified_at": null,
          "type": "custom",
          "description": "My fancy custom benefit",
          "selectable": true,
          "deletable": true,
          "organization_id": "7eae7216-658f-4efd-afe3-1c6ad07d58e5",
          "metadata": {},
          "properties": {
            "note": "hahaha.... got'em."
          },
          "is_tax_applicable": true
        },
        {
          "id": "b376d7ab-dbff-43c6-ad9f-8dec6fa51512",
          "created_at": "2025-10-26T23:24:33.290457Z",
          "modified_at": null,
          "type": "downloadables",
          "description": "lovely art",
          "selectable": true,
          "deletable": true,
          "organization_id": "7eae7216-658f-4efd-afe3-1c6ad07d58e5",
          "metadata": {},
          "properties": {
            "archived": {},
            "files": [
              "f7e4db01-03ed-4eb1-9f91-16851cbd268c"
            ]
          }
        },
        {
          "id": "c7e11cc6-fa5a-48c1-aeb0-29ecd15a2776",
          "created_at": "2025-10-26T23:25:33.134392Z",
          "modified_at": null,
          "type": "license_keys",
          "description": "Developer access",
          "selectable": true,
          "deletable": true,
          "organization_id": "7eae7216-658f-4efd-afe3-1c6ad07d58e5",
          "metadata": {},
          "properties": {
            "prefix": "ACME_FANCY_LICENSE_KEY",
            "expires": {
              "ttl": 1,
              "timeframe": "year"
            },
            "activations": {
              "limit": 5,
              "enable_customer_admin": true
            },
            "limit_usage": 3
          }
        }
      ],
      "medias": [],
      "attached_custom_fields": []
    }"#;

        let _: Product = serde_json::from_str(product_json).unwrap();
    }
}
