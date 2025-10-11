# \CheckoutsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**checkouts_client_confirm**](CheckoutsApi.md#checkouts_client_confirm) | **POST** /v1/checkouts/client/{client_secret}/confirm | Confirm Checkout Session from Client
[**checkouts_client_get**](CheckoutsApi.md#checkouts_client_get) | **GET** /v1/checkouts/client/{client_secret} | Get Checkout Session from Client
[**checkouts_client_update**](CheckoutsApi.md#checkouts_client_update) | **PATCH** /v1/checkouts/client/{client_secret} | Update Checkout Session from Client
[**checkouts_create**](CheckoutsApi.md#checkouts_create) | **POST** /v1/checkouts/ | Create Checkout Session
[**checkouts_get**](CheckoutsApi.md#checkouts_get) | **GET** /v1/checkouts/{id} | Get Checkout Session
[**checkouts_list**](CheckoutsApi.md#checkouts_list) | **GET** /v1/checkouts/ | List Checkout Sessions
[**checkouts_update**](CheckoutsApi.md#checkouts_update) | **PATCH** /v1/checkouts/{id} | Update Checkout Session



## checkouts_client_confirm

> models::CheckoutPublicConfirmed checkouts_client_confirm(client_secret, checkout_confirm_stripe)
Confirm Checkout Session from Client

Confirm a checkout session by client secret.  Orders and subscriptions will be processed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_secret** | **String** | The checkout session client secret. | [required] |
**checkout_confirm_stripe** | [**CheckoutConfirmStripe**](CheckoutConfirmStripe.md) |  | [required] |

### Return type

[**models::CheckoutPublicConfirmed**](CheckoutPublicConfirmed.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_client_get

> models::CheckoutPublic checkouts_client_get(client_secret)
Get Checkout Session from Client

Get a checkout session by client secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_secret** | **String** | The checkout session client secret. | [required] |

### Return type

[**models::CheckoutPublic**](CheckoutPublic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_client_update

> models::CheckoutPublic checkouts_client_update(client_secret, checkout_update_public)
Update Checkout Session from Client

Update a checkout session by client secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_secret** | **String** | The checkout session client secret. | [required] |
**checkout_update_public** | [**CheckoutUpdatePublic**](CheckoutUpdatePublic.md) |  | [required] |

### Return type

[**models::CheckoutPublic**](CheckoutPublic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_create

> models::Checkout checkouts_create(body)
Create Checkout Session

Create a checkout session.  **Scopes**: `checkouts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **models::CheckoutProductsCreate** |  | [required] |

### Return type

[**models::Checkout**](Checkout.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_get

> models::Checkout checkouts_get(id)
Get Checkout Session

Get a checkout session by ID.  **Scopes**: `checkouts:read` `checkouts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout session ID. | [required] |

### Return type

[**models::Checkout**](Checkout.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_list

> models::ListResourceCheckout checkouts_list(organization_id, product_id, customer_id, status, query, page, limit, sorting)
List Checkout Sessions

List checkout sessions.  **Scopes**: `checkouts:read` `checkouts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**status** | Option<[**StatusFilter**](.md)> | Filter by checkout session status. |  |
**query** | Option<**String**> | Filter by customer email. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CheckoutSortProperty>**](models::CheckoutSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCheckout**](ListResource_Checkout_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_update

> models::Checkout checkouts_update(id, checkout_update)
Update Checkout Session

Update a checkout session.  **Scopes**: `checkouts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout session ID. | [required] |
**checkout_update** | [**CheckoutUpdate**](CheckoutUpdate.md) |  | [required] |

### Return type

[**models::Checkout**](Checkout.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

