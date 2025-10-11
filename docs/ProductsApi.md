# \ProductsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**products_create**](ProductsApi.md#products_create) | **POST** /v1/products/ | Create Product
[**products_get**](ProductsApi.md#products_get) | **GET** /v1/products/{id} | Get Product
[**products_list**](ProductsApi.md#products_list) | **GET** /v1/products/ | List Products
[**products_update**](ProductsApi.md#products_update) | **PATCH** /v1/products/{id} | Update Product
[**products_update_benefits**](ProductsApi.md#products_update_benefits) | **POST** /v1/products/{id}/benefits | Update Product Benefits



## products_create

> models::Product products_create(product_create)
Create Product

Create a product.  **Scopes**: `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_create** | [**ProductCreate**](ProductCreate.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_get

> models::Product products_get(id)
Get Product

Get a product by ID.  **Scopes**: `products:read` `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_list

> models::ListResourceProduct products_list(id, organization_id, query, is_archived, is_recurring, benefit_id, page, limit, sorting, metadata)
List Products

List products.  **Scopes**: `products:read` `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**query** | Option<**String**> | Filter by product name. |  |
**is_archived** | Option<**bool**> | Filter on archived products. |  |
**is_recurring** | Option<**bool**> | Filter on recurring products. If `true`, only subscriptions tiers are returned. If `false`, only one-time purchase products are returned.  |  |
**benefit_id** | Option<[**BenefitIdFilter**](.md)> | Filter products granting specific benefit. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::ProductSortProperty>**](models::ProductSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceProduct**](ListResource_Product_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_update

> models::Product products_update(id, product_update)
Update Product

Update a product.  **Scopes**: `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**product_update** | [**ProductUpdate**](ProductUpdate.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_update_benefits

> models::Product products_update_benefits(id, product_benefits_update)
Update Product Benefits

Update benefits granted by a product.  **Scopes**: `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**product_benefits_update** | [**ProductBenefitsUpdate**](ProductBenefitsUpdate.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

