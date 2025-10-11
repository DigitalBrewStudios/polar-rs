# CheckoutLinkUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | [optional]
**trial_interval_count** | Option<**i32**> |  | [optional]
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**products** | Option<**Vec<String>**> |  | [optional]
**label** | Option<**String**> |  | [optional]
**allow_discount_codes** | Option<**bool**> |  | [optional]
**require_billing_address** | Option<**bool**> |  | [optional]
**discount_id** | Option<**String**> |  | [optional]
**success_url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


