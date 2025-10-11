# LicenseKeyActivate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** |  | 
**organization_id** | **String** |  | 
**label** | **String** |  | 
**conditions** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to set conditions that must match when validating the license key.  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]
**meta** | Option<[**std::collections::HashMap<String, models::MetadataValue1>**](Metadata_value_1.md)> | Key-value object allowing you to store additional information about the activation  The key must be a string with a maximum length of **40 characters**. The value must be either:  * A string with a maximum length of **500 characters** * An integer * A floating-point number * A boolean  You can store up to **50 key-value pairs**. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


