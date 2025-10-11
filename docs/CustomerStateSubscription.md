# CustomerStateSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the subscription. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**custom_field_data** | Option<[**std::collections::HashMap<String, models::CustomFieldDataValue>**](Custom_Field_Data_value.md)> | Key-value object storing custom field values. | [optional]
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**status** | **String** |  | 
**amount** | **i32** | The amount of the subscription. | 
**currency** | **String** | The currency of the subscription. | 
**recurring_interval** | [**models::SubscriptionRecurringInterval**](SubscriptionRecurringInterval.md) | The interval at which the subscription recurs. | 
**current_period_start** | **String** | The start timestamp of the current billing period. | 
**current_period_end** | Option<**String**> |  | 
**trial_start** | Option<**String**> |  | 
**trial_end** | Option<**String**> |  | 
**cancel_at_period_end** | **bool** | Whether the subscription will be canceled at the end of the current period. | 
**canceled_at** | Option<**String**> |  | 
**started_at** | Option<**String**> |  | 
**ends_at** | Option<**String**> |  | 
**product_id** | **String** | The ID of the subscribed product. | 
**discount_id** | Option<**String**> |  | 
**meters** | [**Vec<models::CustomerStateSubscriptionMeter>**](CustomerStateSubscriptionMeter.md) | List of meters associated with the subscription. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


