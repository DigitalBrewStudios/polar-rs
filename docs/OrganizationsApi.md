# \OrganizationsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_organizations_get**](OrganizationsApi.md#customer_portal_organizations_get) | **GET** /v1/customer-portal/organizations/{slug} | Get Organization
[**organizations_create**](OrganizationsApi.md#organizations_create) | **POST** /v1/organizations/ | Create Organization
[**organizations_get**](OrganizationsApi.md#organizations_get) | **GET** /v1/organizations/{id} | Get Organization
[**organizations_list**](OrganizationsApi.md#organizations_list) | **GET** /v1/organizations/ | List Organizations
[**organizations_update**](OrganizationsApi.md#organizations_update) | **PATCH** /v1/organizations/{id} | Update Organization



## customer_portal_organizations_get

> models::CustomerOrganization customer_portal_organizations_get(slug)
Get Organization

Get a customer portal's organization by slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The organization slug. | [required] |

### Return type

[**models::CustomerOrganization**](CustomerOrganization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_create

> models::Organization organizations_create(organization_create)
Create Organization

Create an organization.  **Scopes**: `organizations:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_create** | [**OrganizationCreate**](OrganizationCreate.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_get

> models::Organization organizations_get(id)
Get Organization

Get an organization by ID.  **Scopes**: `organizations:read` `organizations:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_list

> models::ListResourceOrganization organizations_list(slug, page, limit, sorting)
List Organizations

List organizations.  **Scopes**: `organizations:read` `organizations:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | Option<**String**> | Filter by slug. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::OrganizationSortProperty>**](models::OrganizationSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceOrganization**](ListResource_Organization_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_update

> models::Organization organizations_update(id, organization_update)
Update Organization

Update an organization.  **Scopes**: `organizations:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**organization_update** | [**OrganizationUpdate**](OrganizationUpdate.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

