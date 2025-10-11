# LicenseKeyWithActivations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the object. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**organization_id** | **String** |  | 
**customer_id** | **String** |  | 
**customer** | [**models::LicenseKeyCustomer**](LicenseKeyCustomer.md) |  | 
**benefit_id** | **String** | The benefit ID. | 
**key** | **String** |  | 
**display_key** | **String** |  | 
**status** | [**models::LicenseKeyStatus**](LicenseKeyStatus.md) |  | 
**limit_activations** | Option<**i32**> |  | 
**usage** | **i32** |  | 
**limit_usage** | Option<**i32**> |  | 
**validations** | **i32** |  | 
**last_validated_at** | Option<**String**> |  | 
**expires_at** | Option<**String**> |  | 
**activations** | [**Vec<models::LicenseKeyActivationBase>**](LicenseKeyActivationBase.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


