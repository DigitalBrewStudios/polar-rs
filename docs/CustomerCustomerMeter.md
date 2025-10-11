# CustomerCustomerMeter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the object. | 
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**customer_id** | **String** | The ID of the customer. | 
**meter_id** | **String** | The ID of the meter. | 
**consumed_units** | **f64** | The number of consumed units. | 
**credited_units** | **i32** | The number of credited units. | 
**balance** | **f64** | The balance of the meter, i.e. the difference between credited and consumed units. | 
**meter** | [**models::CustomerCustomerMeterMeter**](CustomerCustomerMeterMeter.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


