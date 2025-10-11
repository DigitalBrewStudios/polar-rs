# ProductUpdatePricesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**amount_type** | **String** |  | 
**price_amount** | **i32** | The price in cents. | 
**price_currency** | Option<**String**> | The currency. Currently, only `usd` is supported. | [optional][default to usd]
**minimum_amount** | Option<**i32**> | The price in cents. | [optional]
**maximum_amount** | Option<**i32**> | The price in cents. | [optional]
**preset_amount** | Option<**i32**> | The price in cents. | [optional]
**seat_tiers** | [**models::ProductPriceSeatTiers**](ProductPriceSeatTiers.md) | Tiered pricing based on seat quantity | 
**meter_id** | **String** | The ID of the meter associated to the price. | 
**unit_amount** | [**models::UnitAmount**](Unit_Amount.md) |  | 
**cap_amount** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


