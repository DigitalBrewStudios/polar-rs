# EventCreateExternalCustomer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | Option<**String**> | The timestamp of the event. | [optional]
**name** | **String** | The name of the event. | 
**organization_id** | Option<**String**> | The organization ID. | [optional]
**metadata** | Option<[**models::EventMetadataInput**](EventMetadataInput.md)> | Key-value object allowing you to store additional information about the event. Some keys like `_llm` are structured data that are handled specially by Polar.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**external_customer_id** | **String** | ID of the customer in your system associated with the event. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


