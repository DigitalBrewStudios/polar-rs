# \DefaultApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**endpointbenefit_created_post**](DefaultApi.md#endpointbenefit_created_post) | **POST** /benefit.created | benefit.created
[**endpointbenefit_grant_created_post**](DefaultApi.md#endpointbenefit_grant_created_post) | **POST** /benefit_grant.created | benefit_grant.created
[**endpointbenefit_grant_cycled_post**](DefaultApi.md#endpointbenefit_grant_cycled_post) | **POST** /benefit_grant.cycled | benefit_grant.cycled
[**endpointbenefit_grant_revoked_post**](DefaultApi.md#endpointbenefit_grant_revoked_post) | **POST** /benefit_grant.revoked | benefit_grant.revoked
[**endpointbenefit_grant_updated_post**](DefaultApi.md#endpointbenefit_grant_updated_post) | **POST** /benefit_grant.updated | benefit_grant.updated
[**endpointbenefit_updated_post**](DefaultApi.md#endpointbenefit_updated_post) | **POST** /benefit.updated | benefit.updated
[**endpointcheckout_created_post**](DefaultApi.md#endpointcheckout_created_post) | **POST** /checkout.created | checkout.created
[**endpointcheckout_updated_post**](DefaultApi.md#endpointcheckout_updated_post) | **POST** /checkout.updated | checkout.updated
[**endpointcustomer_created_post**](DefaultApi.md#endpointcustomer_created_post) | **POST** /customer.created | customer.created
[**endpointcustomer_deleted_post**](DefaultApi.md#endpointcustomer_deleted_post) | **POST** /customer.deleted | customer.deleted
[**endpointcustomer_seat_assigned_post**](DefaultApi.md#endpointcustomer_seat_assigned_post) | **POST** /customer_seat.assigned | customer_seat.assigned
[**endpointcustomer_seat_claimed_post**](DefaultApi.md#endpointcustomer_seat_claimed_post) | **POST** /customer_seat.claimed | customer_seat.claimed
[**endpointcustomer_seat_revoked_post**](DefaultApi.md#endpointcustomer_seat_revoked_post) | **POST** /customer_seat.revoked | customer_seat.revoked
[**endpointcustomer_state_changed_post**](DefaultApi.md#endpointcustomer_state_changed_post) | **POST** /customer.state_changed | customer.state_changed
[**endpointcustomer_updated_post**](DefaultApi.md#endpointcustomer_updated_post) | **POST** /customer.updated | customer.updated
[**endpointorder_created_post**](DefaultApi.md#endpointorder_created_post) | **POST** /order.created | order.created
[**endpointorder_paid_post**](DefaultApi.md#endpointorder_paid_post) | **POST** /order.paid | order.paid
[**endpointorder_refunded_post**](DefaultApi.md#endpointorder_refunded_post) | **POST** /order.refunded | order.refunded
[**endpointorder_updated_post**](DefaultApi.md#endpointorder_updated_post) | **POST** /order.updated | order.updated
[**endpointorganization_updated_post**](DefaultApi.md#endpointorganization_updated_post) | **POST** /organization.updated | organization.updated
[**endpointproduct_created_post**](DefaultApi.md#endpointproduct_created_post) | **POST** /product.created | product.created
[**endpointproduct_updated_post**](DefaultApi.md#endpointproduct_updated_post) | **POST** /product.updated | product.updated
[**endpointrefund_created_post**](DefaultApi.md#endpointrefund_created_post) | **POST** /refund.created | refund.created
[**endpointrefund_updated_post**](DefaultApi.md#endpointrefund_updated_post) | **POST** /refund.updated | refund.updated
[**endpointsubscription_active_post**](DefaultApi.md#endpointsubscription_active_post) | **POST** /subscription.active | subscription.active
[**endpointsubscription_canceled_post**](DefaultApi.md#endpointsubscription_canceled_post) | **POST** /subscription.canceled | subscription.canceled
[**endpointsubscription_created_post**](DefaultApi.md#endpointsubscription_created_post) | **POST** /subscription.created | subscription.created
[**endpointsubscription_revoked_post**](DefaultApi.md#endpointsubscription_revoked_post) | **POST** /subscription.revoked | subscription.revoked
[**endpointsubscription_uncanceled_post**](DefaultApi.md#endpointsubscription_uncanceled_post) | **POST** /subscription.uncanceled | subscription.uncanceled
[**endpointsubscription_updated_post**](DefaultApi.md#endpointsubscription_updated_post) | **POST** /subscription.updated | subscription.updated



## endpointbenefit_created_post

> serde_json::Value endpointbenefit_created_post(webhook_benefit_created_payload)
benefit.created

Sent when a new benefit is created.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_benefit_created_payload** | [**WebhookBenefitCreatedPayload**](WebhookBenefitCreatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointbenefit_grant_created_post

> serde_json::Value endpointbenefit_grant_created_post(webhook_benefit_grant_created_payload)
benefit_grant.created

Sent when a new benefit grant is created.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_benefit_grant_created_payload** | [**WebhookBenefitGrantCreatedPayload**](WebhookBenefitGrantCreatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointbenefit_grant_cycled_post

> serde_json::Value endpointbenefit_grant_cycled_post(webhook_benefit_grant_cycled_payload)
benefit_grant.cycled

Sent when a benefit grant is cycled, meaning the related subscription has been renewed for another period.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_benefit_grant_cycled_payload** | [**WebhookBenefitGrantCycledPayload**](WebhookBenefitGrantCycledPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointbenefit_grant_revoked_post

> serde_json::Value endpointbenefit_grant_revoked_post(webhook_benefit_grant_revoked_payload)
benefit_grant.revoked

Sent when a benefit grant is revoked.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_benefit_grant_revoked_payload** | [**WebhookBenefitGrantRevokedPayload**](WebhookBenefitGrantRevokedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointbenefit_grant_updated_post

> serde_json::Value endpointbenefit_grant_updated_post(webhook_benefit_grant_updated_payload)
benefit_grant.updated

Sent when a benefit grant is updated.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_benefit_grant_updated_payload** | [**WebhookBenefitGrantUpdatedPayload**](WebhookBenefitGrantUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointbenefit_updated_post

> serde_json::Value endpointbenefit_updated_post(webhook_benefit_updated_payload)
benefit.updated

Sent when a benefit is updated.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_benefit_updated_payload** | [**WebhookBenefitUpdatedPayload**](WebhookBenefitUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcheckout_created_post

> serde_json::Value endpointcheckout_created_post(webhook_checkout_created_payload)
checkout.created

Sent when a new checkout is created.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_checkout_created_payload** | [**WebhookCheckoutCreatedPayload**](WebhookCheckoutCreatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcheckout_updated_post

> serde_json::Value endpointcheckout_updated_post(webhook_checkout_updated_payload)
checkout.updated

Sent when a checkout is updated.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_checkout_updated_payload** | [**WebhookCheckoutUpdatedPayload**](WebhookCheckoutUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcustomer_created_post

> serde_json::Value endpointcustomer_created_post(webhook_customer_created_payload)
customer.created

Sent when a new customer is created.  A customer can be created:  * After a successful checkout. * Programmatically via the API.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_customer_created_payload** | [**WebhookCustomerCreatedPayload**](WebhookCustomerCreatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcustomer_deleted_post

> serde_json::Value endpointcustomer_deleted_post(webhook_customer_deleted_payload)
customer.deleted

Sent when a customer is deleted.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_customer_deleted_payload** | [**WebhookCustomerDeletedPayload**](WebhookCustomerDeletedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcustomer_seat_assigned_post

> serde_json::Value endpointcustomer_seat_assigned_post(webhook_customer_seat_assigned_payload)
customer_seat.assigned

Sent when a new customer seat is assigned.  This event is triggered when a seat is assigned to a customer by the organization. The customer will receive an invitation email to claim the seat.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_customer_seat_assigned_payload** | [**WebhookCustomerSeatAssignedPayload**](WebhookCustomerSeatAssignedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcustomer_seat_claimed_post

> serde_json::Value endpointcustomer_seat_claimed_post(webhook_customer_seat_claimed_payload)
customer_seat.claimed

Sent when a customer seat is claimed.  This event is triggered when a customer accepts the seat invitation and claims their access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_customer_seat_claimed_payload** | [**WebhookCustomerSeatClaimedPayload**](WebhookCustomerSeatClaimedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcustomer_seat_revoked_post

> serde_json::Value endpointcustomer_seat_revoked_post(webhook_customer_seat_revoked_payload)
customer_seat.revoked

Sent when a customer seat is revoked.  This event is triggered when access to a seat is revoked, either manually by the organization or automatically when a subscription is canceled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_customer_seat_revoked_payload** | [**WebhookCustomerSeatRevokedPayload**](WebhookCustomerSeatRevokedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcustomer_state_changed_post

> serde_json::Value endpointcustomer_state_changed_post(webhook_customer_state_changed_payload)
customer.state_changed

Sent when a customer state has changed.  It's triggered when:  * Customer is created, updated or deleted. * A subscription is created or updated. * A benefit is granted or revoked.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_customer_state_changed_payload** | [**WebhookCustomerStateChangedPayload**](WebhookCustomerStateChangedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointcustomer_updated_post

> serde_json::Value endpointcustomer_updated_post(webhook_customer_updated_payload)
customer.updated

Sent when a customer is updated.  This event is fired when the customer details are updated.  If you want to be notified when a customer subscription or benefit state changes, you should listen to the `customer_state_changed` event.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_customer_updated_payload** | [**WebhookCustomerUpdatedPayload**](WebhookCustomerUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointorder_created_post

> serde_json::Value endpointorder_created_post(webhook_order_created_payload)
order.created

Sent when a new order is created.  A new order is created when:  * A customer purchases a one-time product. In this case, `billing_reason` is set to `purchase`. * A customer starts a subscription. In this case, `billing_reason` is set to `subscription_create`. * A subscription is renewed. In this case, `billing_reason` is set to `subscription_cycle`. * A subscription is upgraded or downgraded with an immediate proration invoice. In this case, `billing_reason` is set to `subscription_update`.  <Warning>The order might not be paid yet, so the `status` field might be `pending`.</Warning>  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_order_created_payload** | [**WebhookOrderCreatedPayload**](WebhookOrderCreatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointorder_paid_post

> serde_json::Value endpointorder_paid_post(webhook_order_paid_payload)
order.paid

Sent when an order is paid.  When you receive this event, the order is fully processed and payment has been received.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_order_paid_payload** | [**WebhookOrderPaidPayload**](WebhookOrderPaidPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointorder_refunded_post

> serde_json::Value endpointorder_refunded_post(webhook_order_refunded_payload)
order.refunded

Sent when an order is fully or partially refunded.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_order_refunded_payload** | [**WebhookOrderRefundedPayload**](WebhookOrderRefundedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointorder_updated_post

> serde_json::Value endpointorder_updated_post(webhook_order_updated_payload)
order.updated

Sent when an order is updated.  An order is updated when:  * Its status changes, e.g. from `pending` to `paid`. * It's refunded, partially or fully.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_order_updated_payload** | [**WebhookOrderUpdatedPayload**](WebhookOrderUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointorganization_updated_post

> serde_json::Value endpointorganization_updated_post(webhook_organization_updated_payload)
organization.updated

Sent when a organization is updated.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_organization_updated_payload** | [**WebhookOrganizationUpdatedPayload**](WebhookOrganizationUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointproduct_created_post

> serde_json::Value endpointproduct_created_post(webhook_product_created_payload)
product.created

Sent when a new product is created.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_product_created_payload** | [**WebhookProductCreatedPayload**](WebhookProductCreatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointproduct_updated_post

> serde_json::Value endpointproduct_updated_post(webhook_product_updated_payload)
product.updated

Sent when a product is updated.  **Discord & Slack support:** Basic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_product_updated_payload** | [**WebhookProductUpdatedPayload**](WebhookProductUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointrefund_created_post

> serde_json::Value endpointrefund_created_post(webhook_refund_created_payload)
refund.created

Sent when a refund is created regardless of status.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_refund_created_payload** | [**WebhookRefundCreatedPayload**](WebhookRefundCreatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointrefund_updated_post

> serde_json::Value endpointrefund_updated_post(webhook_refund_updated_payload)
refund.updated

Sent when a refund is updated.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_refund_updated_payload** | [**WebhookRefundUpdatedPayload**](WebhookRefundUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointsubscription_active_post

> serde_json::Value endpointsubscription_active_post(webhook_subscription_active_payload)
subscription.active

Sent when a subscription becomes active, whether because it's a new paid subscription or because payment was recovered.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_subscription_active_payload** | [**WebhookSubscriptionActivePayload**](WebhookSubscriptionActivePayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointsubscription_canceled_post

> serde_json::Value endpointsubscription_canceled_post(webhook_subscription_canceled_payload)
subscription.canceled

Sent when a subscription is canceled. Customers might still have access until the end of the current period.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_subscription_canceled_payload** | [**WebhookSubscriptionCanceledPayload**](WebhookSubscriptionCanceledPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointsubscription_created_post

> serde_json::Value endpointsubscription_created_post(webhook_subscription_created_payload)
subscription.created

Sent when a new subscription is created.  When this event occurs, the subscription `status` might not be `active` yet, as we can still have to wait for the first payment to be processed.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_subscription_created_payload** | [**WebhookSubscriptionCreatedPayload**](WebhookSubscriptionCreatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointsubscription_revoked_post

> serde_json::Value endpointsubscription_revoked_post(webhook_subscription_revoked_payload)
subscription.revoked

Sent when a subscription is revoked, the user loses access immediately. Happens when the subscription is canceled, or payment is past due.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_subscription_revoked_payload** | [**WebhookSubscriptionRevokedPayload**](WebhookSubscriptionRevokedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointsubscription_uncanceled_post

> serde_json::Value endpointsubscription_uncanceled_post(webhook_subscription_uncanceled_payload)
subscription.uncanceled

Sent when a subscription is uncanceled.  **Discord & Slack support:** Full

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_subscription_uncanceled_payload** | [**WebhookSubscriptionUncanceledPayload**](WebhookSubscriptionUncanceledPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpointsubscription_updated_post

> serde_json::Value endpointsubscription_updated_post(webhook_subscription_updated_payload)
subscription.updated

Sent when a subscription is updated. This event fires for all changes to the subscription, including renewals.  If you want more specific events, you can listen to `subscription.active`, `subscription.canceled`, and `subscription.revoked`.  To listen specifically for renewals, you can listen to `order.created` events and check the `billing_reason` field.  **Discord & Slack support:** On cancellation and revocation. Renewals are skipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_subscription_updated_payload** | [**WebhookSubscriptionUpdatedPayload**](WebhookSubscriptionUpdatedPayload.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

