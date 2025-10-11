# \BenefitsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**benefits_create**](BenefitsApi.md#benefits_create) | **POST** /v1/benefits/ | Create Benefit
[**benefits_delete**](BenefitsApi.md#benefits_delete) | **DELETE** /v1/benefits/{id} | Delete Benefit
[**benefits_get**](BenefitsApi.md#benefits_get) | **GET** /v1/benefits/{id} | Get Benefit
[**benefits_grants**](BenefitsApi.md#benefits_grants) | **GET** /v1/benefits/{id}/grants | List Benefit Grants
[**benefits_list**](BenefitsApi.md#benefits_list) | **GET** /v1/benefits/ | List Benefits
[**benefits_update**](BenefitsApi.md#benefits_update) | **PATCH** /v1/benefits/{id} | Update Benefit



## benefits_create

> models::Benefit benefits_create(benefit_create)
Create Benefit

Create a benefit.  **Scopes**: `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**benefit_create** | [**BenefitCreate**](BenefitCreate.md) |  | [required] |

### Return type

[**models::Benefit**](Benefit.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_delete

> benefits_delete(id)
Delete Benefit

Delete a benefit.  > [!WARNING] > Every grants associated with the benefit will be revoked. > Users will lose access to the benefit.  **Scopes**: `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_get

> models::Benefit benefits_get(id)
Get Benefit

Get a benefit by ID.  **Scopes**: `benefits:read` `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Benefit**](Benefit.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_grants

> models::ListResourceBenefitGrant benefits_grants(id, is_granted, customer_id, page, limit)
List Benefit Grants

List the individual grants for a benefit.  It's especially useful to check if a user has been granted a benefit.  **Scopes**: `benefits:read` `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**is_granted** | Option<**bool**> | Filter by granted status. If `true`, only granted benefits will be returned. If `false`, only revoked benefits will be returned.  |  |
**customer_id** | Option<**String**> | Filter by customer. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceBenefitGrant**](ListResource_BenefitGrant_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_list

> models::ListResourceBenefit benefits_list(organization_id, r#type, query, page, limit, sorting, metadata)
List Benefits

List benefits.  **Scopes**: `benefits:read` `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**r#type** | Option<[**BenefitTypeFilter**](.md)> | Filter by benefit type. |  |
**query** | Option<**String**> | Filter by description. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::BenefitSortProperty>**](models::BenefitSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceBenefit**](ListResource_Benefit_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_update

> models::Benefit benefits_update(id, benefit_update)
Update Benefit

Update a benefit.  **Scopes**: `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**benefit_update** | [**BenefitUpdate**](BenefitUpdate.md) |  | [required] |

### Return type

[**models::Benefit**](Benefit.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

