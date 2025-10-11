# \CustomerMetersApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_meters_get**](CustomerMetersApi.md#customer_meters_get) | **GET** /v1/customer-meters/{id} | Get Customer Meter
[**customer_meters_list**](CustomerMetersApi.md#customer_meters_list) | **GET** /v1/customer-meters/ | List Customer Meters
[**customer_portal_customer_meters_get**](CustomerMetersApi.md#customer_portal_customer_meters_get) | **GET** /v1/customer-portal/meters/{id} | Get Customer Meter
[**customer_portal_customer_meters_list**](CustomerMetersApi.md#customer_portal_customer_meters_list) | **GET** /v1/customer-portal/meters/ | List Meters



## customer_meters_get

> models::CustomerMeter customer_meters_get(id)
Get Customer Meter

Get a customer meter by ID.  **Scopes**: `customer_meters:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer meter ID. | [required] |

### Return type

[**models::CustomerMeter**](CustomerMeter.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_meters_list

> models::ListResourceCustomerMeter customer_meters_list(organization_id, customer_id, external_customer_id, meter_id, page, limit, sorting)
List Customer Meters

List customer meters.  **Scopes**: `customer_meters:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**external_customer_id** | Option<[**ExternalCustomerIdFilter2**](.md)> | Filter by external customer ID. |  |
**meter_id** | Option<[**MeterIdFilter1**](.md)> | Filter by meter ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerMeterSortProperty>**](models::CustomerMeterSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerMeter**](ListResource_CustomerMeter_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customer_meters_get

> models::CustomerCustomerMeter customer_portal_customer_meters_get(id)
Get Customer Meter

Get a meter by ID for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer meter ID. | [required] |

### Return type

[**models::CustomerCustomerMeter**](CustomerCustomerMeter.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customer_meters_list

> models::ListResourceCustomerCustomerMeter customer_portal_customer_meters_list(meter_id, query, page, limit, sorting)
List Meters

List meters of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meter_id** | Option<[**MeterIdFilter**](.md)> | Filter by meter ID. |  |
**query** | Option<**String**> | Filter by meter name. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerCustomerMeterSortProperty>**](models::CustomerCustomerMeterSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerCustomerMeter**](ListResource_CustomerCustomerMeter_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

