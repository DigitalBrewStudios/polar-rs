# CheckoutUpdatePublic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_field_data** | Option<[**std::collections::HashMap<String, models::CustomFieldDataValue>**](Custom_Field_Data_value.md)> | Key-value object storing custom field values. | [optional]
**product_id** | Option<**String**> |  | [optional]
**product_price_id** | Option<**String**> |  | [optional]
**amount** | Option<**i32**> | Amount in cents, before discounts and taxes. Only useful for custom prices, it'll be ignored for fixed and free prices. | [optional]
**seats** | Option<**i32**> |  | [optional]
**is_business_customer** | Option<**bool**> |  | [optional]
**customer_name** | Option<**String**> | Name of the customer. | [optional]
**customer_email** | Option<**String**> | Email address of the customer. | [optional]
**customer_billing_name** | Option<**String**> |  | [optional]
**customer_billing_address** | Option<[**models::AddressInput**](AddressInput.md)> | Billing address of the customer. | [optional]
**customer_tax_id** | Option<**String**> |  | [optional]
**discount_code** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


