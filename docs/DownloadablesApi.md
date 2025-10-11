# \DownloadablesApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_downloadables_list**](DownloadablesApi.md#customer_portal_downloadables_list) | **GET** /v1/customer-portal/downloadables/ | List Downloadables



## customer_portal_downloadables_list

> models::ListResourceDownloadableRead customer_portal_downloadables_list(benefit_id, page, limit)
List Downloadables

**Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**benefit_id** | Option<[**BenefitIdFilter3**](.md)> | Filter by benefit ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceDownloadableRead**](ListResource_DownloadableRead_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

