# Checkout

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the object. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**custom_field_data** | Option<[**std::collections::HashMap<String, models::CustomFieldDataValue>**](Custom_Field_Data_value.md)> | Key-value object storing custom field values. | [optional]
**payment_processor** | [**models::PaymentProcessor**](PaymentProcessor.md) | Payment processor used. | 
**status** | [**models::CheckoutStatus**](CheckoutStatus.md) |          Status of the checkout session.          - Open: the checkout session was opened.         - Expired: the checkout session was expired and is no more accessible.         - Confirmed: the user on the checkout session clicked Pay. This is not indicative of the payment's success status.         - Failed: the checkout definitely failed for technical reasons and cannot be retried. In most cases, this state is never reached.         - Succeeded: the payment on the checkout was performed successfully.          | 
**client_secret** | **String** | Client secret used to update and complete the checkout session from the client. | 
**url** | **String** | URL where the customer can access the checkout session. | 
**expires_at** | **String** | Expiration date and time of the checkout session. | 
**success_url** | **String** | URL where the customer will be redirected after a successful payment. | 
**return_url** | Option<**String**> |  | 
**embed_origin** | Option<**String**> |  | 
**amount** | **i32** | Amount in cents, before discounts and taxes. | 
**seats** | Option<**i32**> |  | [optional]
**price_per_seat** | Option<**i32**> |  | [optional]
**discount_amount** | **i32** | Discount amount in cents. | 
**net_amount** | **i32** | Amount in cents, after discounts but before taxes. | 
**tax_amount** | Option<**i32**> |  | 
**total_amount** | **i32** | Amount in cents, after discounts and taxes. | 
**currency** | **String** | Currency code of the checkout session. | 
**active_trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | 
**active_trial_interval_count** | Option<**i32**> |  | 
**trial_end** | Option<**String**> |  | 
**organization_id** | **String** | ID of the organization owning the checkout session. | 
**product_id** | Option<**String**> |  | 
**product_price_id** | Option<**String**> |  | 
**discount_id** | Option<**String**> |  | 
**allow_discount_codes** | **bool** | Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it. | 
**require_billing_address** | **bool** | Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`. | 
**is_discount_applicable** | **bool** | Whether the discount is applicable to the checkout. Typically, free and custom prices are not discountable. | 
**is_free_product_price** | **bool** | Whether the product price is free, regardless of discounts. | 
**is_payment_required** | **bool** | Whether the checkout requires payment, e.g. in case of free products or discounts that cover the total amount. | 
**is_payment_setup_required** | **bool** | Whether the checkout requires setting up a payment method, regardless of the amount, e.g. subscriptions that have first free cycles. | 
**is_payment_form_required** | **bool** | Whether the checkout requires a payment form, whether because of a payment or payment method setup. | 
**customer_id** | Option<**String**> |  | 
**is_business_customer** | **bool** | Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name. | 
**customer_name** | Option<**String**> |  | 
**customer_email** | Option<**String**> |  | 
**customer_ip_address** | Option<**String**> |  | 
**customer_billing_name** | Option<**String**> |  | 
**customer_billing_address** | Option<[**models::Address**](Address.md)> | Billing address of the customer. | 
**customer_tax_id** | Option<**String**> |  | 
**payment_processor_metadata** | **std::collections::HashMap<String, String>** |  | 
**billing_address_fields** | [**models::CheckoutBillingAddressFields**](CheckoutBillingAddressFields.md) | Determine which billing address fields should be disabled, optional or required in the checkout form. | 
**trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | 
**trial_interval_count** | Option<**i32**> |  | 
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**external_customer_id** | Option<**String**> |  | 
**customer_external_id** | Option<**String**> |  | 
**products** | [**Vec<models::CheckoutProduct>**](CheckoutProduct.md) | List of products available to select. | 
**product** | Option<[**models::CheckoutProduct**](CheckoutProduct.md)> |  | 
**product_price** | Option<[**models::CheckoutProductPrice**](Checkout_product_price.md)> |  | 
**discount** | Option<[**models::CheckoutDiscount**](Checkout_discount.md)> |  | 
**subscription_id** | Option<**String**> |  | 
**attached_custom_fields** | Option<[**Vec<models::AttachedCustomField>**](AttachedCustomField.md)> |  | 
**customer_metadata** | [**std::collections::HashMap<String, models::CustomerMetadataValue>**](Customer_Metadata_value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


