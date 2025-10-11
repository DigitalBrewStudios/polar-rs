# SeatAssign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subscription_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**checkout_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**order_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**email** | Option<**String**> |  | [optional]
**external_customer_id** | Option<**String**> |  | [optional]
**customer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**immediate_claim** | Option<**bool**> | If true, the seat will be immediately claimed without sending an invitation email. API-only feature. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


