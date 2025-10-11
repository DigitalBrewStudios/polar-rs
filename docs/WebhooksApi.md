# \WebhooksApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhooks_create_webhook_endpoint**](WebhooksApi.md#webhooks_create_webhook_endpoint) | **POST** /v1/webhooks/endpoints | Create Webhook Endpoint
[**webhooks_delete_webhook_endpoint**](WebhooksApi.md#webhooks_delete_webhook_endpoint) | **DELETE** /v1/webhooks/endpoints/{id} | Delete Webhook Endpoint
[**webhooks_get_webhook_endpoint**](WebhooksApi.md#webhooks_get_webhook_endpoint) | **GET** /v1/webhooks/endpoints/{id} | Get Webhook Endpoint
[**webhooks_list_webhook_deliveries**](WebhooksApi.md#webhooks_list_webhook_deliveries) | **GET** /v1/webhooks/deliveries | List Webhook Deliveries
[**webhooks_list_webhook_endpoints**](WebhooksApi.md#webhooks_list_webhook_endpoints) | **GET** /v1/webhooks/endpoints | List Webhook Endpoints
[**webhooks_redeliver_webhook_event**](WebhooksApi.md#webhooks_redeliver_webhook_event) | **POST** /v1/webhooks/events/{id}/redeliver | Redeliver Webhook Event
[**webhooks_reset_webhook_endpoint_secret**](WebhooksApi.md#webhooks_reset_webhook_endpoint_secret) | **PATCH** /v1/webhooks/endpoints/{id}/secret | Reset Webhook Endpoint Secret
[**webhooks_update_webhook_endpoint**](WebhooksApi.md#webhooks_update_webhook_endpoint) | **PATCH** /v1/webhooks/endpoints/{id} | Update Webhook Endpoint



## webhooks_create_webhook_endpoint

> models::WebhookEndpoint webhooks_create_webhook_endpoint(webhook_endpoint_create)
Create Webhook Endpoint

Create a webhook endpoint.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_endpoint_create** | [**WebhookEndpointCreate**](WebhookEndpointCreate.md) |  | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_delete_webhook_endpoint

> webhooks_delete_webhook_endpoint(id)
Delete Webhook Endpoint

Delete a webhook endpoint.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook endpoint ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_get_webhook_endpoint

> models::WebhookEndpoint webhooks_get_webhook_endpoint(id)
Get Webhook Endpoint

Get a webhook endpoint by ID.  **Scopes**: `webhooks:read` `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook endpoint ID. | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_list_webhook_deliveries

> models::ListResourceWebhookDelivery webhooks_list_webhook_deliveries(endpoint_id, start_timestamp, end_timestamp, page, limit)
List Webhook Deliveries

List webhook deliveries.  Deliveries are all the attempts to deliver a webhook event to an endpoint.  **Scopes**: `webhooks:read` `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | Option<[**EndpointId**](.md)> | Filter by webhook endpoint ID. |  |
**start_timestamp** | Option<**String**> | Filter deliveries after this timestamp. |  |
**end_timestamp** | Option<**String**> | Filter deliveries before this timestamp. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceWebhookDelivery**](ListResource_WebhookDelivery_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_list_webhook_endpoints

> models::ListResourceWebhookEndpoint webhooks_list_webhook_endpoints(organization_id, page, limit)
List Webhook Endpoints

List webhook endpoints.  **Scopes**: `webhooks:read` `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationId**](.md)> | Filter by organization ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceWebhookEndpoint**](ListResource_WebhookEndpoint_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_redeliver_webhook_event

> serde_json::Value webhooks_redeliver_webhook_event(id)
Redeliver Webhook Event

Schedule the re-delivery of a webhook event.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook event ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_reset_webhook_endpoint_secret

> models::WebhookEndpoint webhooks_reset_webhook_endpoint_secret(id)
Reset Webhook Endpoint Secret

Regenerate a webhook endpoint secret.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook endpoint ID. | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_update_webhook_endpoint

> models::WebhookEndpoint webhooks_update_webhook_endpoint(id, webhook_endpoint_update)
Update Webhook Endpoint

Update a webhook endpoint.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook endpoint ID. | [required] |
**webhook_endpoint_update** | [**WebhookEndpointUpdate**](WebhookEndpointUpdate.md) |  | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

