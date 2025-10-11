# GenericPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**processor** | [**models::PaymentProcessor**](PaymentProcessor.md) | The payment processor. | 
**status** | [**models::PaymentStatus**](PaymentStatus.md) | The payment status. | 
**amount** | **i32** | The payment amount in cents. | 
**currency** | **String** | The payment currency. Currently, only `usd` is supported. | 
**method** | **String** | The payment method used. | 
**decline_reason** | Option<**String**> |  | 
**decline_message** | Option<**String**> |  | 
**organization_id** | **String** | The ID of the organization that owns the payment. | 
**checkout_id** | Option<**String**> |  | 
**order_id** | Option<**String**> |  | 
**processor_metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Additional metadata from the payment processor for internal use. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


