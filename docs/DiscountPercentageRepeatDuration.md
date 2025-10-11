# DiscountPercentageRepeatDuration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duration** | [**models::DiscountDuration**](DiscountDuration.md) |  | 
**duration_in_months** | **i32** |  | 
**r#type** | [**models::DiscountType**](DiscountType.md) |  | 
**basis_points** | **i32** | Discount percentage in basis points. A basis point is 1/100th of a percent. For example, 1000 basis points equals a 10% discount. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**name** | **String** | Name of the discount. Will be displayed to the customer when the discount is applied. | 
**code** | Option<**String**> |  | 
**starts_at** | Option<**String**> |  | 
**ends_at** | Option<**String**> |  | 
**max_redemptions** | Option<**i32**> |  | 
**redemptions_count** | **i32** | Number of times the discount has been redeemed. | 
**organization_id** | **String** | The organization ID. | 
**products** | [**Vec<models::DiscountProduct>**](DiscountProduct.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


