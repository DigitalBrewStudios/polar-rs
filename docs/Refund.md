# Refund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**status** | [**models::RefundStatus**](RefundStatus.md) |  | 
**reason** | [**models::RefundReason**](RefundReason.md) |  | 
**amount** | **i32** |  | 
**tax_amount** | **i32** |  | 
**currency** | **String** |  | 
**organization_id** | **String** |  | 
**order_id** | **String** |  | 
**subscription_id** | Option<**String**> |  | 
**customer_id** | **String** |  | 
**revoke_benefits** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


