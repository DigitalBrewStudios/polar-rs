# BenefitGrantedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the object. | 
**timestamp** | **String** | The timestamp of the event. | 
**organization_id** | **String** | The ID of the organization owning the event. | 
**customer_id** | Option<**String**> |  | 
**customer** | Option<[**models::Customer**](Customer.md)> |  | 
**external_customer_id** | Option<**String**> |  | 
**source** | **String** | The source of the event. `system` events are created by Polar. `user` events are the one you create through our ingestion API. | 
**name** | **String** | The name of the event. | 
**metadata** | [**models::BenefitGrantMetadata**](BenefitGrantMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


