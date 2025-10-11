# \SeatsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_seats_assign_seat**](SeatsApi.md#customer_portal_seats_assign_seat) | **POST** /v1/customer-portal/seats | Assign Seat
[**customer_portal_seats_list_claimed_subscriptions**](SeatsApi.md#customer_portal_seats_list_claimed_subscriptions) | **GET** /v1/customer-portal/seats/subscriptions | List Claimed Subscriptions
[**customer_portal_seats_list_seats**](SeatsApi.md#customer_portal_seats_list_seats) | **GET** /v1/customer-portal/seats | List Seats
[**customer_portal_seats_resend_invitation**](SeatsApi.md#customer_portal_seats_resend_invitation) | **POST** /v1/customer-portal/seats/{seat_id}/resend | Resend Invitation
[**customer_portal_seats_revoke_seat**](SeatsApi.md#customer_portal_seats_revoke_seat) | **DELETE** /v1/customer-portal/seats/{seat_id} | Revoke Seat



## customer_portal_seats_assign_seat

> models::CustomerSeat customer_portal_seats_assign_seat(seat_assign)
Assign Seat

**Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seat_assign** | [**SeatAssign**](SeatAssign.md) |  | [required] |

### Return type

[**models::CustomerSeat**](CustomerSeat.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_seats_list_claimed_subscriptions

> Vec<models::CustomerSubscription> customer_portal_seats_list_claimed_subscriptions()
List Claimed Subscriptions

List all subscriptions where the authenticated customer has claimed a seat.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CustomerSubscription>**](CustomerSubscription.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_seats_list_seats

> models::SeatsList customer_portal_seats_list_seats(subscription_id, order_id)
List Seats

**Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**String**> | Subscription ID |  |
**order_id** | Option<**String**> | Order ID |  |

### Return type

[**models::SeatsList**](SeatsList.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_seats_resend_invitation

> models::CustomerSeat customer_portal_seats_resend_invitation(seat_id)
Resend Invitation

**Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seat_id** | **String** |  | [required] |

### Return type

[**models::CustomerSeat**](CustomerSeat.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_seats_revoke_seat

> models::CustomerSeat customer_portal_seats_revoke_seat(seat_id)
Revoke Seat

**Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seat_id** | **String** |  | [required] |

### Return type

[**models::CustomerSeat**](CustomerSeat.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

