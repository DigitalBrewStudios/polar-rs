# LegacyRecurringProductPriceFixed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the price. | 
**amount_type** | **String** |  | 
**is_archived** | **bool** | Whether the price is archived and no longer available. | 
**product_id** | **String** | The ID of the product owning the price. | 
**r#type** | **String** | The type of the price. | 
**recurring_interval** | [**models::SubscriptionRecurringInterval**](SubscriptionRecurringInterval.md) | The recurring interval of the price. | 
**price_currency** | **String** | The currency. | 
**price_amount** | **i32** | The price in cents. | 
**legacy** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


