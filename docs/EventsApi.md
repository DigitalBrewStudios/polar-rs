# \EventsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**events_get**](EventsApi.md#events_get) | **GET** /v1/events/{id} | Get Event
[**events_ingest**](EventsApi.md#events_ingest) | **POST** /v1/events/ingest | Ingest Events
[**events_list**](EventsApi.md#events_list) | **GET** /v1/events/ | List Events
[**events_list_names**](EventsApi.md#events_list_names) | **GET** /v1/events/names | List Event Names



## events_get

> models::Event events_get(id)
Get Event

Get an event by ID.  **Scopes**: `events:read` `events:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The event ID. | [required] |

### Return type

[**models::Event**](Event.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_ingest

> models::EventsIngestResponse events_ingest(events_ingest)
Ingest Events

Ingest batch of events.  **Scopes**: `events:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**events_ingest** | [**EventsIngest**](EventsIngest.md) |  | [required] |

### Return type

[**models::EventsIngestResponse**](EventsIngestResponse.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_list

> models::ListResourceEvent events_list(filter, start_timestamp, end_timestamp, organization_id, customer_id, external_customer_id, meter_id, name, source, query, page, limit, sorting, metadata)
List Events

List events.  **Scopes**: `events:read` `events:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter events following filter clauses. JSON string following the same schema a meter filter clause.  |  |
**start_timestamp** | Option<**String**> | Filter events after this timestamp. |  |
**end_timestamp** | Option<**String**> | Filter events before this timestamp. |  |
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**external_customer_id** | Option<[**ExternalCustomerIdFilter1**](.md)> | Filter by external customer ID. |  |
**meter_id** | Option<**String**> | Filter by a meter filter clause. |  |
**name** | Option<[**NameFilter**](.md)> | Filter by event name. |  |
**source** | Option<[**SourceFilter**](.md)> | Filter by event source. |  |
**query** | Option<**String**> | Query to filter events. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::EventSortProperty>**](models::EventSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceEvent**](ListResource_Event_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_list_names

> models::ListResourceEventName events_list_names(organization_id, customer_id, external_customer_id, source, query, page, limit, sorting)
List Event Names

List event names.  **Scopes**: `events:read` `events:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**external_customer_id** | Option<[**ExternalCustomerIdFilter2**](.md)> | Filter by external customer ID. |  |
**source** | Option<[**SourceFilter**](.md)> | Filter by event source. |  |
**query** | Option<**String**> | Query to filter event names. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::EventNamesSortProperty>**](models::EventNamesSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceEventName**](ListResource_EventName_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

