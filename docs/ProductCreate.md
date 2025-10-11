# ProductCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**name** | **String** | The name of the product. | 
**description** | Option<**String**> |  | [optional]
**prices** | [**Vec<models::ProductCreateOneTimePricesInner>**](ProductCreateOneTime_prices_inner.md) | List of available prices for this product. It should contain at most one static price (fixed, custom or free), and any number of metered prices. Metered prices are not supported on one-time purchase products. | 
**medias** | Option<**Vec<String>**> |  | [optional]
**attached_custom_fields** | Option<[**Vec<models::AttachedCustomFieldCreate>**](AttachedCustomFieldCreate.md)> | List of custom fields to attach. | [optional]
**organization_id** | Option<**String**> | The organization ID. | [optional]
**trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | [optional]
**trial_interval_count** | Option<**i32**> |  | [optional]
**recurring_interval** | [**models::Null**](null.md) | States that the product is a one-time purchase. | 
**recurring_interval_count** | Option<[**models::Null**](null.md)> | One-time products don't have a recurring interval count. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


