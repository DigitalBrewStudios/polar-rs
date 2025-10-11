# OrganizationAvatarFileCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | The organization ID. | [optional]
**name** | **String** |  | 
**mime_type** | **String** | MIME type of the file. Only images are supported for this type of file. | 
**size** | **i32** | Size of the file. A maximum of 1 MB is allowed for this type of file. | 
**checksum_sha256_base64** | Option<**String**> |  | [optional]
**upload** | [**models::S3FileCreateMultipart**](S3FileCreateMultipart.md) |  | 
**service** | **String** |  | 
**version** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


