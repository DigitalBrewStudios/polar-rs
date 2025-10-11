# WebhookEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**last_http_code** | Option<**i32**> |  | [optional]
**succeeded** | Option<**bool**> |  | [optional]
**payload** | Option<**String**> |  | 
**r#type** | [**models::WebhookEventType**](WebhookEventType.md) | The type of the webhook event. | 
**is_archived** | **bool** | Whether this event is archived. Archived events can't be redelivered, and the payload is not accessible anymore. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


