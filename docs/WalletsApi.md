# \WalletsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_wallets_get**](WalletsApi.md#customer_portal_wallets_get) | **GET** /v1/customer-portal/wallets/{id} | Get Wallet
[**customer_portal_wallets_list**](WalletsApi.md#customer_portal_wallets_list) | **GET** /v1/customer-portal/wallets/ | List Wallets
[**wallets_get**](WalletsApi.md#wallets_get) | **GET** /v1/wallets/{id} | Get Wallet
[**wallets_list**](WalletsApi.md#wallets_list) | **GET** /v1/wallets/ | List Wallets
[**wallets_top_up**](WalletsApi.md#wallets_top_up) | **POST** /v1/wallets/{id}/top-up | Top-Up Wallet



## customer_portal_wallets_get

> models::CustomerWallet customer_portal_wallets_get(id)
Get Wallet

Get a wallet by ID for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The wallet ID. | [required] |

### Return type

[**models::CustomerWallet**](CustomerWallet.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_wallets_list

> models::ListResourceCustomerWallet customer_portal_wallets_list(page, limit, sorting)
List Wallets

List wallets of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerWalletSortProperty>**](models::CustomerWalletSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerWallet**](ListResource_CustomerWallet_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallets_get

> models::Wallet wallets_get(id)
Get Wallet

Get a wallet by ID.  **Scopes**: `wallets:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The wallet ID. | [required] |

### Return type

[**models::Wallet**](Wallet.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallets_list

> models::ListResourceWallet wallets_list(organization_id, customer_id, page, limit, sorting)
List Wallets

List wallets.  **Scopes**: `wallets:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::WalletSortProperty>**](models::WalletSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceWallet**](ListResource_Wallet_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallets_top_up

> models::Wallet wallets_top_up(id, wallet_top_up_create)
Top-Up Wallet

Top-up a wallet by adding funds to its balance.  The customer should have a valid payment method on file.  **Scopes**: `wallets:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The wallet ID. | [required] |
**wallet_top_up_create** | [**WalletTopUpCreate**](WalletTopUpCreate.md) |  | [required] |

### Return type

[**models::Wallet**](Wallet.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

