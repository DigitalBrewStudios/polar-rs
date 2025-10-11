# \FilesApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_create**](FilesApi.md#files_create) | **POST** /v1/files/ | Create File
[**files_delete**](FilesApi.md#files_delete) | **DELETE** /v1/files/{id} | Delete File
[**files_list**](FilesApi.md#files_list) | **GET** /v1/files/ | List Files
[**files_update**](FilesApi.md#files_update) | **PATCH** /v1/files/{id} | Update File
[**files_uploaded**](FilesApi.md#files_uploaded) | **POST** /v1/files/{id}/uploaded | Complete File Upload



## files_create

> models::FileUpload files_create(file_create)
Create File

Create a file.  **Scopes**: `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_create** | [**FileCreate**](FileCreate.md) |  | [required] |

### Return type

[**models::FileUpload**](FileUpload.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_delete

> files_delete(id)
Delete File

Delete a file.  **Scopes**: `files:write`

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


## files_list

> models::ListResourceFileRead files_list(organization_id, ids, page, limit)
List Files

List files.  **Scopes**: `files:read` `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**ids** | Option<[**FileIdFilter**](.md)> | Filter by file ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceFileRead**](ListResource_FileRead_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_update

> models::ResponseFilesUpdate files_update(id, file_patch)
Update File

Update a file.  **Scopes**: `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The file ID. | [required] |
**file_patch** | [**FilePatch**](FilePatch.md) |  | [required] |

### Return type

[**models::ResponseFilesUpdate**](Response_Files_Update.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_uploaded

> models::ResponseFilesUploaded files_uploaded(id, file_upload_completed)
Complete File Upload

Complete a file upload.  **Scopes**: `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The file ID. | [required] |
**file_upload_completed** | [**FileUploadCompleted**](FileUploadCompleted.md) |  | [required] |

### Return type

[**models::ResponseFilesUploaded**](Response_Files_Uploaded.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

