# WebhookEndpointCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The URL where the webhook events will be sent. | 
**secret** | Option<**String**> | The secret used to sign the webhook events. | [optional]
**format** | [**models::WebhookFormat**](WebhookFormat.md) | The format of the webhook payload. | 
**events** | [**Vec<models::WebhookEventType>**](WebhookEventType.md) | The events that will trigger the webhook. | 
**organization_id** | Option<**String**> | The organization ID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


