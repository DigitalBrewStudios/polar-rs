# \MetricsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metrics_get**](MetricsApi.md#metrics_get) | **GET** /v1/metrics/ | Get Metrics
[**metrics_limits**](MetricsApi.md#metrics_limits) | **GET** /v1/metrics/limits | Get Metrics Limits



## metrics_get

> models::MetricsResponse metrics_get(start_date, end_date, interval, timezone, organization_id, product_id, billing_type, customer_id)
Get Metrics

Get metrics about your orders and subscriptions.  Currency values are output in cents.  **Scopes**: `metrics:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | Start date. | [required] |
**end_date** | **String** | End date. | [required] |
**interval** | [**TimeInterval**](.md) | Interval between two timestamps. | [required] |
**timezone** | Option<**String**> | Timezone to use for the timestamps. Default is UTC. |  |[default to UTC]
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**billing_type** | Option<[**ProductBillingTypeFilter1**](.md)> | Filter by billing type. `recurring` will filter data corresponding to subscriptions creations or renewals. `one_time` will filter data corresponding to one-time purchases. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |

### Return type

[**models::MetricsResponse**](MetricsResponse.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metrics_limits

> models::MetricsLimits metrics_limits()
Get Metrics Limits

Get the interval limits for the metrics endpoint.  **Scopes**: `metrics:read`

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MetricsLimits**](MetricsLimits.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

