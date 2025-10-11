# OrderProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | [**std::collections::HashMap<String, models::MetadataValue>**](Metadata_value.md) |  | 
**id** | **String** | The ID of the object. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**trial_interval** | Option<[**models::TrialInterval**](TrialInterval.md)> |  | 
**trial_interval_count** | Option<**i32**> |  | 
**name** | **String** | The name of the product. | 
**description** | Option<**String**> |  | 
**recurring_interval** | Option<[**models::SubscriptionRecurringInterval**](SubscriptionRecurringInterval.md)> |  | 
**recurring_interval_count** | Option<**i32**> |  | 
**is_recurring** | **bool** | Whether the product is a subscription. | 
**is_archived** | **bool** | Whether the product is archived and no longer available. | 
**organization_id** | **String** | The ID of the organization owning the product. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


