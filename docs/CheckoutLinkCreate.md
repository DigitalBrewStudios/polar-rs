# CheckoutLinkCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | [optional]
**trial_interval_count** | Option<**i32**> |  | [optional]
**payment_processor** | **String** | Payment processor to use. Currently only Stripe is supported. | 
**label** | Option<**String**> |  | [optional]
**allow_discount_codes** | Option<**bool**> | Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it. | [optional][default to true]
**require_billing_address** | Option<**bool**> | Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. | [optional][default to false]
**discount_id** | Option<**String**> |  | [optional]
**success_url** | Option<**String**> |  | [optional]
**product_price_id** | **String** |  | 
**product_id** | **String** |  | 
**products** | **Vec<String>** | List of products that will be available to select at checkout. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


