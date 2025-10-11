# DiscountUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**name** | Option<**String**> | Name of the discount. Will be displayed to the customer when the discount is applied. | [optional]
**code** | Option<**String**> |  | [optional]
**starts_at** | Option<**String**> |  | [optional]
**ends_at** | Option<**String**> |  | [optional]
**max_redemptions** | Option<**i32**> |  | [optional]
**duration** | Option<[**models::DiscountDuration**](DiscountDuration.md)> |  | [optional]
**duration_in_months** | Option<**i32**> | Number of months the discount should be applied.  For this to work on yearly pricing, you should multiply this by 12. For example, to apply the discount for 2 years, set this to 24. | [optional]
**r#type** | Option<[**models::DiscountType**](DiscountType.md)> |  | [optional]
**amount** | Option<**i32**> | Fixed amount to discount from the invoice total. | [optional]
**currency** | Option<**String**> | The currency. Currently, only `usd` is supported. | [optional]
**basis_points** | Option<**i32**> | Discount percentage in basis points.  A basis point is 1/100th of a percent. For example, to create a 25.5% discount, set this to 2550. | [optional]
**products** | Option<**Vec<String>**> | List of product IDs the discount can be applied to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


