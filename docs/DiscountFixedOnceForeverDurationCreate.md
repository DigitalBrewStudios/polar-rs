# DiscountFixedOnceForeverDurationCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duration** | [**models::DiscountDuration**](DiscountDuration.md) |  | 
**r#type** | [**models::DiscountType**](DiscountType.md) | Type of the discount. | 
**amount** | **i32** | Fixed amount to discount from the invoice total. | 
**currency** | Option<**String**> | The currency. Currently, only `usd` is supported. | [optional][default to usd]
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**name** | **String** | Name of the discount. Will be displayed to the customer when the discount is applied. | 
**code** | Option<**String**> |  | [optional]
**starts_at** | Option<**String**> |  | [optional]
**ends_at** | Option<**String**> |  | [optional]
**max_redemptions** | Option<**i32**> |  | [optional]
**products** | Option<**Vec<String>**> | List of product IDs the discount can be applied to. | [optional]
**organization_id** | Option<**String**> | The organization ID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


