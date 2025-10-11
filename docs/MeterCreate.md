# MeterCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**name** | **String** | The name of the meter. Will be shown on customer's invoices and usage. | 
**filter** | [**models::Filter**](Filter.md) | The filter to apply on events that'll be used to calculate the meter. | 
**aggregation** | [**models::Aggregation**](Aggregation.md) |  | 
**organization_id** | Option<**String**> | The organization ID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


