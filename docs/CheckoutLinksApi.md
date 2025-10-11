# \CheckoutLinksApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**checkout_links_create**](CheckoutLinksApi.md#checkout_links_create) | **POST** /v1/checkout-links/ | Create Checkout Link
[**checkout_links_delete**](CheckoutLinksApi.md#checkout_links_delete) | **DELETE** /v1/checkout-links/{id} | Delete Checkout Link
[**checkout_links_get**](CheckoutLinksApi.md#checkout_links_get) | **GET** /v1/checkout-links/{id} | Get Checkout Link
[**checkout_links_list**](CheckoutLinksApi.md#checkout_links_list) | **GET** /v1/checkout-links/ | List Checkout Links
[**checkout_links_update**](CheckoutLinksApi.md#checkout_links_update) | **PATCH** /v1/checkout-links/{id} | Update Checkout Link



## checkout_links_create

> models::CheckoutLink checkout_links_create(checkout_link_create)
Create Checkout Link

Create a checkout link.  **Scopes**: `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checkout_link_create** | [**CheckoutLinkCreate**](CheckoutLinkCreate.md) |  | [required] |

### Return type

[**models::CheckoutLink**](CheckoutLink.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_delete

> checkout_links_delete(id)
Delete Checkout Link

Delete a checkout link.  **Scopes**: `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout link ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_get

> models::CheckoutLink checkout_links_get(id)
Get Checkout Link

Get a checkout link by ID.  **Scopes**: `checkout_links:read` `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout link ID. | [required] |

### Return type

[**models::CheckoutLink**](CheckoutLink.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_list

> models::ListResourceCheckoutLink checkout_links_list(organization_id, product_id, page, limit, sorting)
List Checkout Links

List checkout links.  **Scopes**: `checkout_links:read` `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CheckoutLinkSortProperty>**](models::CheckoutLinkSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCheckoutLink**](ListResource_CheckoutLink_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_update

> models::CheckoutLink checkout_links_update(id, checkout_link_update)
Update Checkout Link

Update a checkout link.  **Scopes**: `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout link ID. | [required] |
**checkout_link_update** | [**CheckoutLinkUpdate**](CheckoutLinkUpdate.md) |  | [required] |

### Return type

[**models::CheckoutLink**](CheckoutLink.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

