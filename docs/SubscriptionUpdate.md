# SubscriptionUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | **String** | Update subscription to another product. | 
**proration_behavior** | Option<[**models::SubscriptionProrationBehavior**](SubscriptionProrationBehavior.md)> |  | [optional]
**discount_id** | **String** |  | 
**trial_end** | [**models::TrialEnd**](Trial_End.md) |  | 
**seats** | **i32** | Update the number of seats for this subscription. | 
**customer_cancellation_reason** | Option<[**models::CustomerCancellationReason**](CustomerCancellationReason.md)> |  | [optional]
**customer_cancellation_comment** | Option<**String**> |  | [optional]
**cancel_at_period_end** | **bool** | Cancel an active subscription once the current period ends.  Or uncancel a subscription currently set to be revoked at period end. | 
**revoke** | **bool** | Cancel and revoke an active subscription immediately | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


