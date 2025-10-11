# SubscriptionMeter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**consumed_units** | **f64** | The number of consumed units so far in this billing period. | 
**credited_units** | **i32** | The number of credited units so far in this billing period. | 
**amount** | **i32** | The amount due in cents so far in this billing period. | 
**meter_id** | **String** | The ID of the meter. | 
**meter** | [**models::Meter**](Meter.md) | The meter associated with this subscription. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


