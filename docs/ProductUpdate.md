# ProductUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | [optional]
**trial_interval_count** | Option<**i32**> |  | [optional]
**name** | Option<**String**> | The name of the product. | [optional]
**description** | Option<**String**> |  | [optional]
**recurring_interval** | Option<[**models::SubscriptionRecurringInterval**](SubscriptionRecurringInterval.md)> |  | [optional]
**recurring_interval_count** | Option<**i32**> |  | [optional]
**is_archived** | Option<**bool**> |  | [optional]
**prices** | Option<[**Vec<models::ProductUpdatePricesInner>**](ProductUpdate_prices_inner.md)> |  | [optional]
**medias** | Option<**Vec<String>**> |  | [optional]
**attached_custom_fields** | Option<[**Vec<models::AttachedCustomFieldCreate>**](AttachedCustomFieldCreate.md)> | List of custom fields to attach. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


