# EventName

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the event. | 
**source** | [**models::EventSource**](EventSource.md) | The source of the event. `system` events are created by Polar. `user` events are the one you create through our ingestion API. | 
**occurrences** | **i32** | Number of times the event has occurred. | 
**first_seen** | **String** | The first time the event occurred. | 
**last_seen** | **String** | The last time the event occurred. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


