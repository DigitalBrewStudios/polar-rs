# FileUpload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the object. | 
**organization_id** | **String** |  | 
**name** | **String** |  | 
**path** | **String** |  | 
**mime_type** | **String** |  | 
**size** | **i32** |  | 
**storage_version** | Option<**String**> |  | 
**checksum_etag** | Option<**String**> |  | 
**checksum_sha256_base64** | Option<**String**> |  | 
**checksum_sha256_hex** | Option<**String**> |  | 
**last_modified_at** | Option<**String**> |  | 
**upload** | [**models::S3FileUploadMultipart**](S3FileUploadMultipart.md) |  | 
**version** | Option<**String**> |  | 
**is_uploaded** | Option<**bool**> |  | [optional][default to false]
**service** | [**models::FileServiceTypes**](FileServiceTypes.md) |  | 
**size_readable** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


