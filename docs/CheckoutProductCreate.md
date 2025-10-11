# CheckoutProductCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | [optional]
**trial_interval_count** | Option<**i32**> |  | [optional]
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**custom_field_data** | Option<[**std::collections::HashMap<String, models::CustomFieldDataValue>**](Custom_Field_Data_value.md)> | Key-value object storing custom field values. | [optional]
**discount_id** | Option<**String**> |  | [optional]
**allow_discount_codes** | Option<**bool**> | Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it. | [optional][default to true]
**require_billing_address** | Option<**bool**> | Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`. | [optional][default to false]
**amount** | Option<**i32**> | Amount in cents, before discounts and taxes. Only useful for custom prices, it'll be ignored for fixed and free prices. | [optional]
**seats** | Option<**i32**> |  | [optional]
**customer_id** | Option<**String**> |  | [optional]
**is_business_customer** | Option<**bool**> | Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name. | [optional][default to false]
**external_customer_id** | Option<**String**> |  | [optional]
**customer_name** | Option<**String**> | Name of the customer. | [optional]
**customer_email** | Option<**String**> | Email address of the customer. | [optional]
**customer_ip_address** | Option<**String**> |  | [optional]
**customer_billing_name** | Option<**String**> |  | [optional]
**customer_billing_address** | Option<[**models::AddressInput**](AddressInput.md)> | Billing address of the customer. | [optional]
**customer_tax_id** | Option<**String**> |  | [optional]
**customer_metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information that'll be copied to the created customer.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**subscription_id** | Option<**String**> |  | [optional]
**success_url** | Option<**String**> |  | [optional]
**return_url** | Option<**String**> |  | [optional]
**embed_origin** | Option<**String**> |  | [optional]
**product_id** | **String** | ID of the product to checkout. First available price will be selected. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


