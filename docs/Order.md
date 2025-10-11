# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the object. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**status** | [**models::OrderStatus**](OrderStatus.md) |  | 
**paid** | **bool** | Whether the order has been paid for. | 
**subtotal_amount** | **i32** | Amount in cents, before discounts and taxes. | 
**discount_amount** | **i32** | Discount amount in cents. | 
**net_amount** | **i32** | Amount in cents, after discounts but before taxes. | 
**tax_amount** | **i32** | Sales tax amount in cents. | 
**total_amount** | **i32** | Amount in cents, after discounts and taxes. | 
**applied_balance_amount** | **i32** | Customer's balance amount applied to this invoice. Can increase the total amount paid, if the customer has a negative balance,  or decrease it, if the customer has a positive balance.Amount in cents. | 
**due_amount** | **i32** | Amount in cents that is due for this order. | 
**refunded_amount** | **i32** | Amount refunded in cents. | 
**refunded_tax_amount** | **i32** | Sales tax refunded in cents. | 
**currency** | **String** |  | 
**billing_reason** | [**models::OrderBillingReason**](OrderBillingReason.md) |  | 
**billing_name** | Option<**String**> |  | 
**billing_address** | Option<[**models::Address**](Address.md)> |  | 
**invoice_number** | **String** | The invoice number associated with this order. | 
**is_invoice_generated** | **bool** | Whether an invoice has been generated for this order. | 
**seats** | Option<**i32**> |  | [optional]
**customer_id** | **String** |  | 
**product_id** | Option<**String**> |  | 
**discount_id** | Option<**String**> |  | 
**subscription_id** | Option<**String**> |  | 
**checkout_id** | Option<**String**> |  | 
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**custom_field_data** | Option<[**std::collections::HashMap<String, models::CustomFieldDataValue>**](Custom_Field_Data_value.md)> | Key-value object storing custom field values. | [optional]
**platform_fee_amount** | **i32** | Platform fee amount in cents. | 
**customer** | [**models::OrderCustomer**](OrderCustomer.md) |  | 
**user_id** | **String** |  | 
**product** | Option<[**models::OrderProduct**](OrderProduct.md)> |  | 
**discount** | Option<[**models::OrderDiscount**](OrderDiscount.md)> |  | 
**subscription** | Option<[**models::OrderSubscription**](OrderSubscription.md)> |  | 
**items** | [**Vec<models::OrderItemSchema>**](OrderItemSchema.md) | Line items composing the order. | 
**description** | **String** | A summary description of the order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


