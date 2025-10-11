# RefundCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**order_id** | **String** |  | 
**reason** | [**models::RefundReason**](RefundReason.md) |  | 
**amount** | **i32** | Amount to refund in cents. Minimum is 1. | 
**comment** | Option<**String**> |  | [optional]
**revoke_benefits** | Option<**bool**> | Should this refund trigger the associated customer benefits to be revoked?  **Note:** Only allowed in case the `order` is a one-time purchase. Subscriptions automatically revoke customer benefits once the subscription itself is revoked, i.e fully canceled. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


