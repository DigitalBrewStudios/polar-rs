# \CustomerSessionsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_sessions_create**](CustomerSessionsApi.md#customer_sessions_create) | **POST** /v1/customer-sessions/ | Create Customer Session



## customer_sessions_create

> models::CustomerSession customer_sessions_create(customer_session_create)
Create Customer Session

Create a customer session.  **Scopes**: `customer_sessions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_session_create** | [**CustomerSessionCreate**](CustomerSessionCreate.md) |  | [required] |

### Return type

[**models::CustomerSession**](CustomerSession.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

