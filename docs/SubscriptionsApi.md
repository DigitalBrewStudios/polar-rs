# \SubscriptionsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_subscriptions_cancel**](SubscriptionsApi.md#customer_portal_subscriptions_cancel) | **DELETE** /v1/customer-portal/subscriptions/{id} | Cancel Subscription
[**customer_portal_subscriptions_get**](SubscriptionsApi.md#customer_portal_subscriptions_get) | **GET** /v1/customer-portal/subscriptions/{id} | Get Subscription
[**customer_portal_subscriptions_list**](SubscriptionsApi.md#customer_portal_subscriptions_list) | **GET** /v1/customer-portal/subscriptions/ | List Subscriptions
[**customer_portal_subscriptions_update**](SubscriptionsApi.md#customer_portal_subscriptions_update) | **PATCH** /v1/customer-portal/subscriptions/{id} | Update Subscription
[**subscriptions_create**](SubscriptionsApi.md#subscriptions_create) | **POST** /v1/subscriptions/ | Create Subscription
[**subscriptions_export**](SubscriptionsApi.md#subscriptions_export) | **GET** /v1/subscriptions/export | Export Subscriptions
[**subscriptions_get**](SubscriptionsApi.md#subscriptions_get) | **GET** /v1/subscriptions/{id} | Get Subscription
[**subscriptions_list**](SubscriptionsApi.md#subscriptions_list) | **GET** /v1/subscriptions/ | List Subscriptions
[**subscriptions_revoke**](SubscriptionsApi.md#subscriptions_revoke) | **DELETE** /v1/subscriptions/{id} | Revoke Subscription
[**subscriptions_update**](SubscriptionsApi.md#subscriptions_update) | **PATCH** /v1/subscriptions/{id} | Update Subscription



## customer_portal_subscriptions_cancel

> models::CustomerSubscription customer_portal_subscriptions_cancel(id)
Cancel Subscription

Cancel a subscription of the authenticated customer.  **Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |

### Return type

[**models::CustomerSubscription**](CustomerSubscription.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_subscriptions_get

> models::CustomerSubscription customer_portal_subscriptions_get(id)
Get Subscription

Get a subscription for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |

### Return type

[**models::CustomerSubscription**](CustomerSubscription.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_subscriptions_list

> models::ListResourceCustomerSubscription customer_portal_subscriptions_list(product_id, active, query, page, limit, sorting)
List Subscriptions

List subscriptions of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**active** | Option<**bool**> | Filter by active or cancelled subscription. |  |
**query** | Option<**String**> | Search by product or organization name. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerSubscriptionSortProperty>**](models::CustomerSubscriptionSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerSubscription**](ListResource_CustomerSubscription_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_subscriptions_update

> models::CustomerSubscription customer_portal_subscriptions_update(id, customer_subscription_update)
Update Subscription

Update a subscription of the authenticated customer.  **Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |
**customer_subscription_update** | [**CustomerSubscriptionUpdate**](CustomerSubscriptionUpdate.md) |  | [required] |

### Return type

[**models::CustomerSubscription**](CustomerSubscription.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_create

> models::Subscription subscriptions_create(subscription_create)
Create Subscription

Create a subscription programmatically.  This endpoint only allows to create subscription on free products. For paid products, use the checkout flow.  No initial order will be created and no confirmation email will be sent.  **Scopes**: `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_create** | [**SubscriptionCreate**](SubscriptionCreate.md) |  | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_export

> serde_json::Value subscriptions_export(organization_id)
Export Subscriptions

Export subscriptions as a CSV file.  **Scopes**: `subscriptions:read` `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationId**](.md)> | Filter by organization ID. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_get

> models::Subscription subscriptions_get(id)
Get Subscription

Get a subscription by ID.  **Scopes**: `subscriptions:read` `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_list

> models::ListResourceSubscription subscriptions_list(organization_id, product_id, customer_id, external_customer_id, discount_id, active, page, limit, sorting, metadata)
List Subscriptions

List subscriptions.  **Scopes**: `subscriptions:read` `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**external_customer_id** | Option<[**ExternalCustomerIdFilter**](.md)> | Filter by customer external ID. |  |
**discount_id** | Option<[**DiscountIdFilter**](.md)> | Filter by discount ID. |  |
**active** | Option<**bool**> | Filter by active or inactive subscription. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::SubscriptionSortProperty>**](models::SubscriptionSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceSubscription**](ListResource_Subscription_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_revoke

> models::Subscription subscriptions_revoke(id)
Revoke Subscription

Revoke a subscription, i.e cancel immediately.  **Scopes**: `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_update

> models::Subscription subscriptions_update(id, subscription_update)
Update Subscription

Update a subscription.  **Scopes**: `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |
**subscription_update** | [**SubscriptionUpdate**](SubscriptionUpdate.md) |  | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

