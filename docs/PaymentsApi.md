# \PaymentsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payments_get**](PaymentsApi.md#payments_get) | **GET** /v1/payments/{id} | Get Payment
[**payments_list**](PaymentsApi.md#payments_list) | **GET** /v1/payments/ | List Payments



## payments_get

> models::Payment payments_get(id)
Get Payment

Get a payment by ID.  **Scopes**: `payments:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The payment ID. | [required] |

### Return type

[**models::Payment**](Payment.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_list

> models::ListResource payments_list(organization_id, checkout_id, order_id, status, method, customer_email, page, limit, sorting)
List Payments

List payments.  **Scopes**: `payments:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**checkout_id** | Option<[**CheckoutIdFilter1**](.md)> | Filter by checkout ID. |  |
**order_id** | Option<[**OrderIdFilter1**](.md)> | Filter by order ID. |  |
**status** | Option<[**StatusFilter1**](.md)> | Filter by payment status. |  |
**method** | Option<[**MethodFilter**](.md)> | Filter by payment method. |  |
**customer_email** | Option<[**CustomerEmailFilter**](.md)> | Filter by customer email. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::PaymentSortProperty>**](models::PaymentSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResource**](ListResource__.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

