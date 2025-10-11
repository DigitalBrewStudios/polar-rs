# \CustomerSeatsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_seats_assign_seat**](CustomerSeatsApi.md#customer_seats_assign_seat) | **POST** /v1/customer-seats | Assign Seat
[**customer_seats_claim_seat**](CustomerSeatsApi.md#customer_seats_claim_seat) | **POST** /v1/customer-seats/claim | Claim Seat
[**customer_seats_get_claim_info**](CustomerSeatsApi.md#customer_seats_get_claim_info) | **GET** /v1/customer-seats/claim/{invitation_token} | Get Claim Info
[**customer_seats_list_seats**](CustomerSeatsApi.md#customer_seats_list_seats) | **GET** /v1/customer-seats | List Seats
[**customer_seats_resend_invitation**](CustomerSeatsApi.md#customer_seats_resend_invitation) | **POST** /v1/customer-seats/{seat_id}/resend | Resend Invitation
[**customer_seats_revoke_seat**](CustomerSeatsApi.md#customer_seats_revoke_seat) | **DELETE** /v1/customer-seats/{seat_id} | Revoke Seat



## customer_seats_assign_seat

> models::CustomerSeat customer_seats_assign_seat(seat_assign)
Assign Seat

**Scopes**: `customer_seats:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seat_assign** | [**SeatAssign**](SeatAssign.md) |  | [required] |

### Return type

[**models::CustomerSeat**](CustomerSeat.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_seats_claim_seat

> models::CustomerSeatClaimResponse customer_seats_claim_seat(seat_claim)
Claim Seat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seat_claim** | [**SeatClaim**](SeatClaim.md) |  | [required] |

### Return type

[**models::CustomerSeatClaimResponse**](CustomerSeatClaimResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_seats_get_claim_info

> models::SeatClaimInfo customer_seats_get_claim_info(invitation_token)
Get Claim Info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_token** | **String** |  | [required] |

### Return type

[**models::SeatClaimInfo**](SeatClaimInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_seats_list_seats

> models::SeatsList customer_seats_list_seats(subscription_id, order_id)
List Seats

**Scopes**: `customer_seats:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**String**> |  |  |
**order_id** | Option<**String**> |  |  |

### Return type

[**models::SeatsList**](SeatsList.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_seats_resend_invitation

> models::CustomerSeat customer_seats_resend_invitation(seat_id)
Resend Invitation

**Scopes**: `customer_seats:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seat_id** | **String** |  | [required] |

### Return type

[**models::CustomerSeat**](CustomerSeat.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_seats_revoke_seat

> models::CustomerSeat customer_seats_revoke_seat(seat_id)
Revoke Seat

**Scopes**: `customer_seats:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seat_id** | **String** |  | [required] |

### Return type

[**models::CustomerSeat**](CustomerSeat.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

