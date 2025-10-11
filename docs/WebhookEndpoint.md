# WebhookEndpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**url** | **String** | The URL where the webhook events will be sent. | 
**format** | [**models::WebhookFormat**](WebhookFormat.md) | The format of the webhook payload. | 
**secret** | **String** | The secret used to sign the webhook events. | 
**organization_id** | **String** | The organization ID associated with the webhook endpoint. | 
**events** | [**Vec<models::WebhookEventType>**](WebhookEventType.md) | The events that will trigger the webhook. | 
**enabled** | **bool** | Whether the webhook endpoint is enabled and will receive events. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


