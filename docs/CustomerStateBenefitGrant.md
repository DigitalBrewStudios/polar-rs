# CustomerStateBenefitGrant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the grant. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**granted_at** | **String** | The timestamp when the benefit was granted. | 
**benefit_id** | **String** | The ID of the benefit concerned by this grant. | 
**benefit_type** | [**models::BenefitType**](BenefitType.md) | The type of the benefit concerned by this grant. | 
**benefit_metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) | The metadata of the benefit concerned by this grant. | 
**properties** | [**models::Properties**](Properties.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


