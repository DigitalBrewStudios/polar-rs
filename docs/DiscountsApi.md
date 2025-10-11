# \DiscountsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**discounts_create**](DiscountsApi.md#discounts_create) | **POST** /v1/discounts/ | Create Discount
[**discounts_delete**](DiscountsApi.md#discounts_delete) | **DELETE** /v1/discounts/{id} | Delete Discount
[**discounts_get**](DiscountsApi.md#discounts_get) | **GET** /v1/discounts/{id} | Get Discount
[**discounts_list**](DiscountsApi.md#discounts_list) | **GET** /v1/discounts/ | List Discounts
[**discounts_update**](DiscountsApi.md#discounts_update) | **PATCH** /v1/discounts/{id} | Update Discount



## discounts_create

> models::Discount discounts_create(discount_create)
Create Discount

Create a discount.  **Scopes**: `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discount_create** | [**DiscountCreate**](DiscountCreate.md) |  | [required] |

### Return type

[**models::Discount**](Discount.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discounts_delete

> discounts_delete(id)
Delete Discount

Delete a discount.  **Scopes**: `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The discount ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discounts_get

> models::Discount discounts_get(id)
Get Discount

Get a discount by ID.  **Scopes**: `discounts:read` `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The discount ID. | [required] |

### Return type

[**models::Discount**](Discount.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discounts_list

> models::ListResourceDiscount discounts_list(organization_id, query, page, limit, sorting)
List Discounts

List discounts.  **Scopes**: `discounts:read` `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**query** | Option<**String**> | Filter by name. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::DiscountSortProperty>**](models::DiscountSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceDiscount**](ListResource_Discount_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discounts_update

> models::Discount discounts_update(id, discount_update)
Update Discount

Update a discount.  **Scopes**: `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The discount ID. | [required] |
**discount_update** | [**DiscountUpdate**](DiscountUpdate.md) |  | [required] |

### Return type

[**models::Discount**](Discount.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

