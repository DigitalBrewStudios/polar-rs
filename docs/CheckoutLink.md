# CheckoutLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the object. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | 
**trial_interval_count** | Option<**i32**> |  | 
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**payment_processor** | [**models::PaymentProcessor**](PaymentProcessor.md) | Payment processor used. | 
**client_secret** | **String** | Client secret used to access the checkout link. | 
**success_url** | Option<**String**> |  | 
**label** | Option<**String**> |  | 
**allow_discount_codes** | **bool** | Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it. | 
**require_billing_address** | **bool** | Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. | 
**discount_id** | Option<**String**> |  | 
**organization_id** | **String** | The organization ID. | 
**products** | [**Vec<models::CheckoutLinkProduct>**](CheckoutLinkProduct.md) |  | 
**discount** | Option<[**models::CheckoutLinkDiscount**](CheckoutLinkDiscount.md)> |  | 
**url** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


