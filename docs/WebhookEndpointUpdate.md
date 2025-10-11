# WebhookEndpointUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | The URL where the webhook events will be sent. | [optional]
**secret** | Option<**String**> | The secret used to sign the webhook events. | [optional]
**format** | Option<[**models::WebhookFormat**](WebhookFormat.md)> | The format of the webhook payload. | [optional]
**events** | Option<[**Vec<models::WebhookEventType>**](WebhookEventType.md)> | The events that will trigger the webhook. | [optional]
**enabled** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


