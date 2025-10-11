# \MetersApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**meters_create**](MetersApi.md#meters_create) | **POST** /v1/meters/ | Create Meter
[**meters_get**](MetersApi.md#meters_get) | **GET** /v1/meters/{id} | Get Meter
[**meters_list**](MetersApi.md#meters_list) | **GET** /v1/meters/ | List Meters
[**meters_quantities**](MetersApi.md#meters_quantities) | **GET** /v1/meters/{id}/quantities | Get Meter Quantities
[**meters_update**](MetersApi.md#meters_update) | **PATCH** /v1/meters/{id} | Update Meter



## meters_create

> models::Meter meters_create(meter_create)
Create Meter

Create a meter.  **Scopes**: `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meter_create** | [**MeterCreate**](MeterCreate.md) |  | [required] |

### Return type

[**models::Meter**](Meter.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_get

> models::Meter meters_get(id)
Get Meter

Get a meter by ID.  **Scopes**: `meters:read` `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The meter ID. | [required] |

### Return type

[**models::Meter**](Meter.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_list

> models::ListResourceMeter meters_list(organization_id, query, is_archived, page, limit, sorting, metadata)
List Meters

List meters.  **Scopes**: `meters:read` `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**query** | Option<**String**> | Filter by name. |  |
**is_archived** | Option<**bool**> | Filter on archived meters. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::MeterSortProperty>**](models::MeterSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceMeter**](ListResource_Meter_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_quantities

> models::MeterQuantities meters_quantities(id, start_timestamp, end_timestamp, interval, customer_id, external_customer_id, customer_aggregation_function, metadata)
Get Meter Quantities

Get quantities of a meter over a time period.  **Scopes**: `meters:read` `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The meter ID. | [required] |
**start_timestamp** | **String** | Start timestamp. | [required] |
**end_timestamp** | **String** | End timestamp. | [required] |
**interval** | [**TimeInterval**](.md) | Interval between two timestamps. | [required] |
**customer_id** | Option<**String**> | Filter by customer ID. |  |
**external_customer_id** | Option<**String**> | Filter by external customer ID. |  |
**customer_aggregation_function** | Option<[**AggregationFunction**](.md)> | If set, will first compute the quantities per customer before aggregating them using the given function. If not set, the quantities will be aggregated across all events. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::MeterQuantities**](MeterQuantities.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_update

> models::Meter meters_update(id, meter_update)
Update Meter

Update a meter.  **Scopes**: `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The meter ID. | [required] |
**meter_update** | [**MeterUpdate**](MeterUpdate.md) |  | [required] |

### Return type

[**models::Meter**](Meter.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

