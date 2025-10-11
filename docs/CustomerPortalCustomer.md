# CustomerPortalCustomer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**email** | **String** |  | 
**email_verified** | **bool** |  | 
**name** | Option<**String**> |  | 
**billing_name** | Option<**String**> |  | 
**billing_address** | Option<[**models::Address**](Address.md)> |  | 
**tax_id** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | 
**oauth_accounts** | [**std::collections::HashMap<String, models::CustomerPortalOAuthAccount>**](CustomerPortalOAuthAccount.md) |  | 
**default_payment_method_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


