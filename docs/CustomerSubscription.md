# CustomerSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the object. | 
**amount** | **i32** | The amount of the subscription. | 
**currency** | **String** | The currency of the subscription. | 
**recurring_interval** | [**models::SubscriptionRecurringInterval**](SubscriptionRecurringInterval.md) | The interval at which the subscription recurs. | 
**recurring_interval_count** | **i32** | Number of interval units of the subscription. If this is set to 1 the charge will happen every interval (e.g. every month), if set to 2 it will be every other month, and so on. | 
**status** | [**models::SubscriptionStatus**](SubscriptionStatus.md) | The status of the subscription. | 
**current_period_start** | **String** | The start timestamp of the current billing period. | 
**current_period_end** | Option<**String**> |  | 
**trial_start** | Option<**String**> |  | 
**trial_end** | Option<**String**> |  | 
**cancel_at_period_end** | **bool** | Whether the subscription will be canceled at the end of the current period. | 
**canceled_at** | Option<**String**> |  | 
**started_at** | Option<**String**> |  | 
**ends_at** | Option<**String**> |  | 
**ended_at** | Option<**String**> |  | 
**customer_id** | **String** | The ID of the subscribed customer. | 
**product_id** | **String** | The ID of the subscribed product. | 
**discount_id** | Option<**String**> |  | 
**checkout_id** | Option<**String**> |  | 
**seats** | Option<**i32**> |  | [optional]
**customer_cancellation_reason** | Option<[**models::CustomerCancellationReason**](CustomerCancellationReason.md)> |  | 
**customer_cancellation_comment** | Option<**String**> |  | 
**product** | [**models::CustomerSubscriptionProduct**](CustomerSubscriptionProduct.md) |  | 
**prices** | [**Vec<models::CheckoutLinkProductPricesInner>**](CheckoutLinkProduct_prices_inner.md) | List of enabled prices for the subscription. | 
**meters** | [**Vec<models::CustomerSubscriptionMeter>**](CustomerSubscriptionMeter.md) | List of meters associated with the subscription. | 
**is_polar_managed** | **bool** | Whether the subscription is managed by Polar. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


