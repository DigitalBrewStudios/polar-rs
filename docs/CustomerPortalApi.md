# \CustomerPortalApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_benefit_grants_get**](CustomerPortalApi.md#customer_portal_benefit_grants_get) | **GET** /v1/customer-portal/benefit-grants/{id} | Get Benefit Grant
[**customer_portal_benefit_grants_list**](CustomerPortalApi.md#customer_portal_benefit_grants_list) | **GET** /v1/customer-portal/benefit-grants/ | List Benefit Grants
[**customer_portal_benefit_grants_update**](CustomerPortalApi.md#customer_portal_benefit_grants_update) | **PATCH** /v1/customer-portal/benefit-grants/{id} | Update Benefit Grant
[**customer_portal_customer_meters_get**](CustomerPortalApi.md#customer_portal_customer_meters_get) | **GET** /v1/customer-portal/meters/{id} | Get Customer Meter
[**customer_portal_customer_meters_list**](CustomerPortalApi.md#customer_portal_customer_meters_list) | **GET** /v1/customer-portal/meters/ | List Meters
[**customer_portal_customer_session_introspect**](CustomerPortalApi.md#customer_portal_customer_session_introspect) | **GET** /v1/customer-portal/customer-session/introspect | Introspect Customer Session
[**customer_portal_customers_add_payment_method**](CustomerPortalApi.md#customer_portal_customers_add_payment_method) | **POST** /v1/customer-portal/customers/me/payment-methods | Add Customer Payment Method
[**customer_portal_customers_confirm_payment_method**](CustomerPortalApi.md#customer_portal_customers_confirm_payment_method) | **POST** /v1/customer-portal/customers/me/payment-methods/confirm | Confirm Customer Payment Method
[**customer_portal_customers_delete_payment_method**](CustomerPortalApi.md#customer_portal_customers_delete_payment_method) | **DELETE** /v1/customer-portal/customers/me/payment-methods/{id} | Delete Customer Payment Method
[**customer_portal_customers_get**](CustomerPortalApi.md#customer_portal_customers_get) | **GET** /v1/customer-portal/customers/me | Get Customer
[**customer_portal_customers_list_payment_methods**](CustomerPortalApi.md#customer_portal_customers_list_payment_methods) | **GET** /v1/customer-portal/customers/me/payment-methods | List Customer Payment Methods
[**customer_portal_customers_update**](CustomerPortalApi.md#customer_portal_customers_update) | **PATCH** /v1/customer-portal/customers/me | Update Customer
[**customer_portal_downloadables_list**](CustomerPortalApi.md#customer_portal_downloadables_list) | **GET** /v1/customer-portal/downloadables/ | List Downloadables
[**customer_portal_license_keys_activate**](CustomerPortalApi.md#customer_portal_license_keys_activate) | **POST** /v1/customer-portal/license-keys/activate | Activate License Key
[**customer_portal_license_keys_deactivate**](CustomerPortalApi.md#customer_portal_license_keys_deactivate) | **POST** /v1/customer-portal/license-keys/deactivate | Deactivate License Key
[**customer_portal_license_keys_get**](CustomerPortalApi.md#customer_portal_license_keys_get) | **GET** /v1/customer-portal/license-keys/{id} | Get License Key
[**customer_portal_license_keys_list**](CustomerPortalApi.md#customer_portal_license_keys_list) | **GET** /v1/customer-portal/license-keys/ | List License Keys
[**customer_portal_license_keys_validate**](CustomerPortalApi.md#customer_portal_license_keys_validate) | **POST** /v1/customer-portal/license-keys/validate | Validate License Key
[**customer_portal_orders_confirm_retry_payment**](CustomerPortalApi.md#customer_portal_orders_confirm_retry_payment) | **POST** /v1/customer-portal/orders/{id}/confirm-payment | Confirm Retry Payment
[**customer_portal_orders_generate_invoice**](CustomerPortalApi.md#customer_portal_orders_generate_invoice) | **POST** /v1/customer-portal/orders/{id}/invoice | Generate Order Invoice
[**customer_portal_orders_get**](CustomerPortalApi.md#customer_portal_orders_get) | **GET** /v1/customer-portal/orders/{id} | Get Order
[**customer_portal_orders_get_payment_status**](CustomerPortalApi.md#customer_portal_orders_get_payment_status) | **GET** /v1/customer-portal/orders/{id}/payment-status | Get Order Payment Status
[**customer_portal_orders_invoice**](CustomerPortalApi.md#customer_portal_orders_invoice) | **GET** /v1/customer-portal/orders/{id}/invoice | Get Order Invoice
[**customer_portal_orders_list**](CustomerPortalApi.md#customer_portal_orders_list) | **GET** /v1/customer-portal/orders/ | List Orders
[**customer_portal_orders_update**](CustomerPortalApi.md#customer_portal_orders_update) | **PATCH** /v1/customer-portal/orders/{id} | Update Order
[**customer_portal_organizations_get**](CustomerPortalApi.md#customer_portal_organizations_get) | **GET** /v1/customer-portal/organizations/{slug} | Get Organization
[**customer_portal_seats_assign_seat**](CustomerPortalApi.md#customer_portal_seats_assign_seat) | **POST** /v1/customer-portal/seats | Assign Seat
[**customer_portal_seats_list_claimed_subscriptions**](CustomerPortalApi.md#customer_portal_seats_list_claimed_subscriptions) | **GET** /v1/customer-portal/seats/subscriptions | List Claimed Subscriptions
[**customer_portal_seats_list_seats**](CustomerPortalApi.md#customer_portal_seats_list_seats) | **GET** /v1/customer-portal/seats | List Seats
[**customer_portal_seats_resend_invitation**](CustomerPortalApi.md#customer_portal_seats_resend_invitation) | **POST** /v1/customer-portal/seats/{seat_id}/resend | Resend Invitation
[**customer_portal_seats_revoke_seat**](CustomerPortalApi.md#customer_portal_seats_revoke_seat) | **DELETE** /v1/customer-portal/seats/{seat_id} | Revoke Seat
[**customer_portal_subscriptions_cancel**](CustomerPortalApi.md#customer_portal_subscriptions_cancel) | **DELETE** /v1/customer-portal/subscriptions/{id} | Cancel Subscription
[**customer_portal_subscriptions_get**](CustomerPortalApi.md#customer_portal_subscriptions_get) | **GET** /v1/customer-portal/subscriptions/{id} | Get Subscription
[**customer_portal_subscriptions_list**](CustomerPortalApi.md#customer_portal_subscriptions_list) | **GET** /v1/customer-portal/subscriptions/ | List Subscriptions
[**customer_portal_subscriptions_update**](CustomerPortalApi.md#customer_portal_subscriptions_update) | **PATCH** /v1/customer-portal/subscriptions/{id} | Update Subscription
[**customer_portal_wallets_get**](CustomerPortalApi.md#customer_portal_wallets_get) | **GET** /v1/customer-portal/wallets/{id} | Get Wallet
[**customer_portal_wallets_list**](CustomerPortalApi.md#customer_portal_wallets_list) | **GET** /v1/customer-portal/wallets/ | List Wallets



## customer_portal_benefit_grants_get

> models::CustomerBenefitGrant customer_portal_benefit_grants_get(id)
Get Benefit Grant

Get a benefit grant by ID for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The benefit grant ID. | [required] |

### Return type

[**models::CustomerBenefitGrant**](CustomerBenefitGrant.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_benefit_grants_list

> models::ListResourceCustomerBenefitGrant customer_portal_benefit_grants_list(r#type, benefit_id, checkout_id, order_id, subscription_id, page, limit, sorting)
List Benefit Grants

List benefits grants of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<[**BenefitTypeFilter**](.md)> | Filter by benefit type. |  |
**benefit_id** | Option<[**BenefitIdFilter2**](.md)> | Filter by benefit ID. |  |
**checkout_id** | Option<[**CheckoutIdFilter1**](.md)> | Filter by checkout ID. |  |
**order_id** | Option<[**OrderIdFilter**](.md)> | Filter by order ID. |  |
**subscription_id** | Option<[**SubscriptionIdFilter**](.md)> | Filter by subscription ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerBenefitGrantSortProperty>**](models::CustomerBenefitGrantSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerBenefitGrant**](ListResource_CustomerBenefitGrant_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_benefit_grants_update

> models::CustomerBenefitGrant customer_portal_benefit_grants_update(id, customer_benefit_grant_update)
Update Benefit Grant

Update a benefit grant for the authenticated customer.  **Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The benefit grant ID. | [required] |
**customer_benefit_grant_update** | [**CustomerBenefitGrantUpdate**](CustomerBenefitGrantUpdate.md) |  | [required] |

### Return type

[**models::CustomerBenefitGrant**](CustomerBenefitGrant.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customer_meters_get

> models::CustomerCustomerMeter customer_portal_customer_meters_get(id)
Get Customer Meter

Get a meter by ID for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer meter ID. | [required] |

### Return type

[**models::CustomerCustomerMeter**](CustomerCustomerMeter.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customer_meters_list

> models::ListResourceCustomerCustomerMeter customer_portal_customer_meters_list(meter_id, query, page, limit, sorting)
List Meters

List meters of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meter_id** | Option<[**MeterIdFilter**](.md)> | Filter by meter ID. |  |
**query** | Option<**String**> | Filter by meter name. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerCustomerMeterSortProperty>**](models::CustomerCustomerMeterSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerCustomerMeter**](ListResource_CustomerCustomerMeter_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customer_session_introspect

> models::CustomerCustomerSession customer_portal_customer_session_introspect()
Introspect Customer Session

Introspect the current session and return its information.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CustomerCustomerSession**](CustomerCustomerSession.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customers_add_payment_method

> models::CustomerPaymentMethodCreateResponse customer_portal_customers_add_payment_method(customer_payment_method_create)
Add Customer Payment Method

Add a payment method to the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_payment_method_create** | [**CustomerPaymentMethodCreate**](CustomerPaymentMethodCreate.md) |  | [required] |

### Return type

[**models::CustomerPaymentMethodCreateResponse**](CustomerPaymentMethodCreateResponse.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customers_confirm_payment_method

> models::CustomerPaymentMethodCreateResponse customer_portal_customers_confirm_payment_method(customer_payment_method_confirm)
Confirm Customer Payment Method

Confirm a payment method for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_payment_method_confirm** | [**CustomerPaymentMethodConfirm**](CustomerPaymentMethodConfirm.md) |  | [required] |

### Return type

[**models::CustomerPaymentMethodCreateResponse**](CustomerPaymentMethodCreateResponse.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customers_delete_payment_method

> customer_portal_customers_delete_payment_method(id)
Delete Customer Payment Method

Delete a payment method from the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customers_get

> models::CustomerPortalCustomer customer_portal_customers_get()
Get Customer

Get authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CustomerPortalCustomer**](CustomerPortalCustomer.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customers_list_payment_methods

> models::ListResourceCustomerPaymentMethod customer_portal_customers_list_payment_methods(page, limit)
List Customer Payment Methods

Get saved payment methods of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceCustomerPaymentMethod**](ListResource_CustomerPaymentMethod_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_customers_update

> models::CustomerPortalCustomer customer_portal_customers_update(customer_portal_customer_update)
Update Customer

Update authenticated customer.  **Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_portal_customer_update** | [**CustomerPortalCustomerUpdate**](CustomerPortalCustomerUpdate.md) |  | [required] |

### Return type

[**models::CustomerPortalCustomer**](CustomerPortalCustomer.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_downloadables_list

> models::ListResourceDownloadableRead customer_portal_downloadables_list(benefit_id, page, limit)
List Downloadables

**Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**benefit_id** | Option<[**BenefitIdFilter3**](.md)> | Filter by benefit ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceDownloadableRead**](ListResource_DownloadableRead_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_license_keys_activate

> models::LicenseKeyActivationRead customer_portal_license_keys_activate(license_key_activate)
Activate License Key

Activate a license key instance.  > This endpoint doesn't require authentication and can be safely used on a public > client, like a desktop application or a mobile app. > If you plan to validate a license key on a server, use the `/v1/license-keys/activate` > endpoint instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key_activate** | [**LicenseKeyActivate**](LicenseKeyActivate.md) |  | [required] |

### Return type

[**models::LicenseKeyActivationRead**](LicenseKeyActivationRead.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_license_keys_deactivate

> customer_portal_license_keys_deactivate(license_key_deactivate)
Deactivate License Key

Deactivate a license key instance.  > This endpoint doesn't require authentication and can be safely used on a public > client, like a desktop application or a mobile app. > If you plan to validate a license key on a server, use the `/v1/license-keys/deactivate` > endpoint instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key_deactivate** | [**LicenseKeyDeactivate**](LicenseKeyDeactivate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_license_keys_get

> models::LicenseKeyWithActivations customer_portal_license_keys_get(id)
Get License Key

Get a license key.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::LicenseKeyWithActivations**](LicenseKeyWithActivations.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_license_keys_list

> models::ListResourceLicenseKeyRead customer_portal_license_keys_list(benefit_id, page, limit)
List License Keys

**Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**benefit_id** | Option<**String**> | Filter by a specific benefit |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceLicenseKeyRead**](ListResource_LicenseKeyRead_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_license_keys_validate

> models::ValidatedLicenseKey customer_portal_license_keys_validate(license_key_validate)
Validate License Key

Validate a license key.  > This endpoint doesn't require authentication and can be safely used on a public > client, like a desktop application or a mobile app. > If you plan to validate a license key on a server, use the `/v1/license-keys/validate` > endpoint instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key_validate** | [**LicenseKeyValidate**](LicenseKeyValidate.md) |  | [required] |

### Return type

[**models::ValidatedLicenseKey**](ValidatedLicenseKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_orders_confirm_retry_payment

> models::CustomerOrderPaymentConfirmation customer_portal_orders_confirm_retry_payment(id, customer_order_confirm_payment)
Confirm Retry Payment

Confirm a retry payment using a Stripe confirmation token.  **Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |
**customer_order_confirm_payment** | [**CustomerOrderConfirmPayment**](CustomerOrderConfirmPayment.md) |  | [required] |

### Return type

[**models::CustomerOrderPaymentConfirmation**](CustomerOrderPaymentConfirmation.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_orders_generate_invoice

> serde_json::Value customer_portal_orders_generate_invoice(id)
Generate Order Invoice

Trigger generation of an order's invoice.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_orders_get

> models::CustomerOrder customer_portal_orders_get(id)
Get Order

Get an order by ID for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |

### Return type

[**models::CustomerOrder**](CustomerOrder.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_orders_get_payment_status

> models::CustomerOrderPaymentStatus customer_portal_orders_get_payment_status(id)
Get Order Payment Status

Get the current payment status for an order.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |

### Return type

[**models::CustomerOrderPaymentStatus**](CustomerOrderPaymentStatus.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_orders_invoice

> models::CustomerOrderInvoice customer_portal_orders_invoice(id)
Get Order Invoice

Get an order's invoice data.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |

### Return type

[**models::CustomerOrderInvoice**](CustomerOrderInvoice.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_orders_list

> models::ListResourceCustomerOrder customer_portal_orders_list(product_id, product_billing_type, subscription_id, query, page, limit, sorting)
List Orders

List orders of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**product_billing_type** | Option<[**ProductBillingTypeFilter**](.md)> | Filter by product billing type. `recurring` will filter data corresponding to subscriptions creations or renewals. `one_time` will filter data corresponding to one-time purchases. |  |
**subscription_id** | Option<[**SubscriptionIdFilter**](.md)> | Filter by subscription ID. |  |
**query** | Option<**String**> | Search by product or organization name. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerOrderSortProperty>**](models::CustomerOrderSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerOrder**](ListResource_CustomerOrder_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_orders_update

> models::CustomerOrder customer_portal_orders_update(id, customer_order_update)
Update Order

Update an order for the authenticated customer.  **Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |
**customer_order_update** | [**CustomerOrderUpdate**](CustomerOrderUpdate.md) |  | [required] |

### Return type

[**models::CustomerOrder**](CustomerOrder.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_organizations_get

> models::CustomerOrganization customer_portal_organizations_get(slug)
Get Organization

Get a customer portal's organization by slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The organization slug. | [required] |

### Return type

[**models::CustomerOrganization**](CustomerOrganization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## customer_portal_subscriptions_cancel

> models::CustomerSubscription customer_portal_subscriptions_cancel(id)
Cancel Subscription

Cancel a subscription of the authenticated customer.  **Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |

### Return type

[**models::CustomerSubscription**](CustomerSubscription.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_subscriptions_get

> models::CustomerSubscription customer_portal_subscriptions_get(id)
Get Subscription

Get a subscription for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |

### Return type

[**models::CustomerSubscription**](CustomerSubscription.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_subscriptions_list

> models::ListResourceCustomerSubscription customer_portal_subscriptions_list(product_id, active, query, page, limit, sorting)
List Subscriptions

List subscriptions of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**active** | Option<**bool**> | Filter by active or cancelled subscription. |  |
**query** | Option<**String**> | Search by product or organization name. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerSubscriptionSortProperty>**](models::CustomerSubscriptionSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerSubscription**](ListResource_CustomerSubscription_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_subscriptions_update

> models::CustomerSubscription customer_portal_subscriptions_update(id, customer_subscription_update)
Update Subscription

Update a subscription of the authenticated customer.  **Scopes**: `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |
**customer_subscription_update** | [**CustomerSubscriptionUpdate**](CustomerSubscriptionUpdate.md) |  | [required] |

### Return type

[**models::CustomerSubscription**](CustomerSubscription.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_wallets_get

> models::CustomerWallet customer_portal_wallets_get(id)
Get Wallet

Get a wallet by ID for the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The wallet ID. | [required] |

### Return type

[**models::CustomerWallet**](CustomerWallet.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_portal_wallets_list

> models::ListResourceCustomerWallet customer_portal_wallets_list(page, limit, sorting)
List Wallets

List wallets of the authenticated customer.  **Scopes**: `customer_portal:read` `customer_portal:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerWalletSortProperty>**](models::CustomerWalletSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerWallet**](ListResource_CustomerWallet_.md)

### Authorization

[customer_session](../README.md#customer_session)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

