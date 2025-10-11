# \CustomerSessionApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_customer_session_introspect**](CustomerSessionApi.md#customer_portal_customer_session_introspect) | **GET** /v1/customer-portal/customer-session/introspect | Introspect Customer Session



## customer_portal_customer_session_introspect

> models::CustomerCustomerSession customer_portal_customer_session_introspect()
Introspect Customer Session

Introspect the current session and return its information.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CustomerCustomerSession**](CustomerCustomerSession.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

