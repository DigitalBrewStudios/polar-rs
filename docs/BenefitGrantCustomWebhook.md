# BenefitGrantCustomWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The ID of the grant. | 
**granted_at** | Option<**String**> |  | [optional]
**is_granted** | **bool** | Whether the benefit is granted. | 
**revoked_at** | Option<**String**> |  | [optional]
**is_revoked** | **bool** | Whether the benefit is revoked. | 
**subscription_id** | Option<**String**> |  | 
**order_id** | Option<**String**> |  | 
**customer_id** | **String** | The ID of the customer concerned by this grant. | 
**benefit_id** | **String** | The ID of the benefit concerned by this grant. | 
**error** | Option<[**models::BenefitGrantError**](BenefitGrantError.md)> |  | [optional]
**customer** | [**models::Customer**](Customer.md) |  | 
**benefit** | [**models::BenefitCustom**](BenefitCustom.md) |  | 
**properties** | [**serde_json::Value**](.md) |  | 
**previous_properties** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


