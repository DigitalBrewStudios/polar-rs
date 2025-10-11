# LicenseKeyCustomer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the customer. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**external_id** | Option<**String**> |  | 
**email** | **String** | The email address of the customer. This must be unique within the organization. | 
**email_verified** | **bool** | Whether the customer email address is verified. The address is automatically verified when the customer accesses the customer portal using their email address. | 
**name** | Option<**String**> |  | 
**billing_address** | Option<[**models::Address**](Address.md)> |  | 
**tax_id** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | 
**organization_id** | **String** | The ID of the organization owning the customer. | 
**deleted_at** | Option<**String**> |  | 
**avatar_url** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


