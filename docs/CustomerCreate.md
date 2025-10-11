# CustomerCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**external_id** | Option<**String**> |  | [optional]
**email** | **String** | The email address of the customer. This must be unique within the organization. | 
**name** | Option<**String**> |  | [optional]
**billing_address** | Option<[**models::AddressInput**](AddressInput.md)> |  | [optional]
**tax_id** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**organization_id** | Option<**String**> | The organization ID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


