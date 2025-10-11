# CustomerSubscriptionUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | **String** | Update subscription to another product. | 
**seats** | **i32** | Update the number of seats for this subscription. | 
**proration_behavior** | Option<[**models::SubscriptionProrationBehavior**](SubscriptionProrationBehavior.md)> |  | [optional]
**cancel_at_period_end** | Option<**bool**> |  | [optional]
**cancellation_reason** | Option<[**models::CustomerCancellationReason**](CustomerCancellationReason.md)> |  | [optional]
**cancellation_comment** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


