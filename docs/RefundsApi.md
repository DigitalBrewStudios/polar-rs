# \RefundsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**refunds_create**](RefundsApi.md#refunds_create) | **POST** /v1/refunds/ | Create Refund
[**refunds_list**](RefundsApi.md#refunds_list) | **GET** /v1/refunds/ | List Refunds



## refunds_create

> models::Refund refunds_create(refund_create)
Create Refund

Create a refund.  **Scopes**: `refunds:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refund_create** | [**RefundCreate**](RefundCreate.md) |  | [required] |

### Return type

[**models::Refund**](Refund.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refunds_list

> models::ListResourceRefund refunds_list(id, organization_id, order_id, subscription_id, customer_id, succeeded, page, limit, sorting)
List Refunds

List products.  **Scopes**: `refunds:read` `refunds:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**RefundIdFilter**](.md)> | Filter by refund ID. |  |
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**order_id** | Option<[**OrderIdFilter**](.md)> | Filter by order ID. |  |
**subscription_id** | Option<[**SubscriptionIdFilter**](.md)> | Filter by subscription ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**succeeded** | Option<**bool**> | Filter by `succeeded`. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::RefundSortProperty>**](models::RefundSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceRefund**](ListResource_Refund_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

