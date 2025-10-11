# SubscriptionCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**product_id** | **String** | The ID of the recurring product to subscribe to. Must be a free product, otherwise the customer should go through a checkout flow. | 
**customer_id** | **String** | The ID of the customer to create the subscription for. | 
**external_customer_id** | **String** | The ID of the customer in your system to create the subscription for. It must already exist in Polar. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


