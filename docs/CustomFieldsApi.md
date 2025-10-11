# \CustomFieldsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**custom_fields_create**](CustomFieldsApi.md#custom_fields_create) | **POST** /v1/custom-fields/ | Create Custom Field
[**custom_fields_delete**](CustomFieldsApi.md#custom_fields_delete) | **DELETE** /v1/custom-fields/{id} | Delete Custom Field
[**custom_fields_get**](CustomFieldsApi.md#custom_fields_get) | **GET** /v1/custom-fields/{id} | Get Custom Field
[**custom_fields_list**](CustomFieldsApi.md#custom_fields_list) | **GET** /v1/custom-fields/ | List Custom Fields
[**custom_fields_update**](CustomFieldsApi.md#custom_fields_update) | **PATCH** /v1/custom-fields/{id} | Update Custom Field



## custom_fields_create

> models::CustomField custom_fields_create(custom_field_create)
Create Custom Field

Create a custom field.  **Scopes**: `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_field_create** | [**CustomFieldCreate**](CustomFieldCreate.md) |  | [required] |

### Return type

[**models::CustomField**](CustomField.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_delete

> custom_fields_delete(id)
Delete Custom Field

Delete a custom field.  **Scopes**: `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The custom field ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_get

> models::CustomField custom_fields_get(id)
Get Custom Field

Get a custom field by ID.  **Scopes**: `custom_fields:read` `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The custom field ID. | [required] |

### Return type

[**models::CustomField**](CustomField.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_list

> models::ListResourceCustomField custom_fields_list(organization_id, query, r#type, page, limit, sorting)
List Custom Fields

List custom fields.  **Scopes**: `custom_fields:read` `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**query** | Option<**String**> | Filter by custom field name or slug. |  |
**r#type** | Option<[**CustomFieldTypeFilter**](.md)> | Filter by custom field type. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomFieldSortProperty>**](models::CustomFieldSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomField**](ListResource_CustomField_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_update

> models::CustomField custom_fields_update(id, custom_field_update)
Update Custom Field

Update a custom field.  **Scopes**: `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The custom field ID. | [required] |
**custom_field_update** | [**CustomFieldUpdate**](CustomFieldUpdate.md) |  | [required] |

### Return type

[**models::CustomField**](CustomField.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

