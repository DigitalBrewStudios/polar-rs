# Meter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**name** | **String** | The name of the meter. Will be shown on customer's invoices and usage. | 
**filter** | [**models::Filter**](Filter.md) | The filter to apply on events that'll be used to calculate the meter. | 
**aggregation** | [**models::Aggregation**](Aggregation.md) |  | 
**organization_id** | **String** | The ID of the organization owning the meter. | 
**archived_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


