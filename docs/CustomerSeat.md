# CustomerSeat

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The seat ID | 
**subscription_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**order_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**status** | [**models::SeatStatus**](SeatStatus.md) | Status of the seat | 
**customer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**customer_email** | Option<**String**> |  | [optional]
**invitation_token_expires_at** | Option<**String**> |  | [optional]
**claimed_at** | Option<**String**> |  | [optional]
**revoked_at** | Option<**String**> |  | [optional]
**seat_metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


