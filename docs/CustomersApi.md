# \CustomersApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_customers_add_payment_method**](CustomersApi.md#customer_portal_customers_add_payment_method) | **POST** /v1/customer-portal/customers/me/payment-methods | Add Customer Payment Method
[**customer_portal_customers_confirm_payment_method**](CustomersApi.md#customer_portal_customers_confirm_payment_method) | **POST** /v1/customer-portal/customers/me/payment-methods/confirm | Confirm Customer Payment Method
[**customer_portal_customers_delete_payment_method**](CustomersApi.md#customer_portal_customers_delete_payment_method) | **DELETE** /v1/customer-portal/customers/me/payment-methods/{id} | Delete Customer Payment Method
[**customer_portal_customers_get**](CustomersApi.md#customer_portal_customers_get) | **GET** /v1/customer-portal/customers/me | Get Customer
[**customer_portal_customers_list_payment_methods**](CustomersApi.md#customer_portal_customers_list_payment_methods) | **GET** /v1/customer-portal/customers/me/payment-methods | List Customer Payment Methods
[**customer_portal_customers_update**](CustomersApi.md#customer_portal_customers_update) | **PATCH** /v1/customer-portal/customers/me | Update Customer
[**customers_create**](CustomersApi.md#customers_create) | **POST** /v1/customers/ | Create Customer
[**customers_delete**](CustomersApi.md#customers_delete) | **DELETE** /v1/customers/{id} | Delete Customer
[**customers_delete_external**](CustomersApi.md#customers_delete_external) | **DELETE** /v1/customers/external/{external_id} | Delete Customer by External ID
[**customers_export**](CustomersApi.md#customers_export) | **GET** /v1/customers/export | Export Customers
[**customers_get**](CustomersApi.md#customers_get) | **GET** /v1/customers/{id} | Get Customer
[**customers_get_balance**](CustomersApi.md#customers_get_balance) | **GET** /v1/customers/{id}/balance | Get Customer Balance
[**customers_get_external**](CustomersApi.md#customers_get_external) | **GET** /v1/customers/external/{external_id} | Get Customer by External ID
[**customers_get_state**](CustomersApi.md#customers_get_state) | **GET** /v1/customers/{id}/state | Get Customer State
[**customers_get_state_external**](CustomersApi.md#customers_get_state_external) | **GET** /v1/customers/external/{external_id}/state | Get Customer State by External ID
[**customers_list**](CustomersApi.md#customers_list) | **GET** /v1/customers/ | List Customers
[**customers_update**](CustomersApi.md#customers_update) | **PATCH** /v1/customers/{id} | Update Customer
[**customers_update_external**](CustomersApi.md#customers_update_external) | **PATCH** /v1/customers/external/{external_id} | Update Customer by External ID



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


## customers_create

> models::Customer customers_create(customer_create)
Create Customer

Create a customer.  **Scopes**: `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_create** | [**CustomerCreate**](CustomerCreate.md) |  | [required] |

### Return type

[**models::Customer**](Customer.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_delete

> customers_delete(id)
Delete Customer

Delete a customer.  This action cannot be undone and will immediately: - Cancel any active subscriptions for the customer - Revoke all their benefits - Clear any `external_id`  Use it only in the context of deleting a user within your own service. Otherwise, use more granular API endpoints to cancel a specific subscription or revoke certain benefits.  Note: The customers information will nonetheless be retained for historic orders and subscriptions.  **Scopes**: `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_delete_external

> customers_delete_external(external_id)
Delete Customer by External ID

Delete a customer by external ID.  Immediately cancels any active subscriptions and revokes any active benefits.  **Scopes**: `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | The customer external ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_export

> serde_json::Value customers_export(organization_id)
Export Customers

Export customers as a CSV file.  **Scopes**: `customers:read` `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationId**](.md)> | Filter by organization ID. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_get

> models::Customer customers_get(id)
Get Customer

Get a customer by ID.  **Scopes**: `customers:read` `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer ID. | [required] |

### Return type

[**models::Customer**](Customer.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_get_balance

> models::CustomerBalance customers_get_balance(id)
Get Customer Balance

Get customer balance information.  **Scopes**: `customers:read` `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer ID. | [required] |

### Return type

[**models::CustomerBalance**](CustomerBalance.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_get_external

> models::Customer customers_get_external(external_id)
Get Customer by External ID

Get a customer by external ID.  **Scopes**: `customers:read` `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | The customer external ID. | [required] |

### Return type

[**models::Customer**](Customer.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_get_state

> models::CustomerState customers_get_state(id)
Get Customer State

Get a customer state by ID.  The customer state includes information about the customer's active subscriptions and benefits.  It's the ideal endpoint to use when you need to get a full overview of a customer's status.  **Scopes**: `customers:read` `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer ID. | [required] |

### Return type

[**models::CustomerState**](CustomerState.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_get_state_external

> models::CustomerState customers_get_state_external(external_id)
Get Customer State by External ID

Get a customer state by external ID.  The customer state includes information about the customer's active subscriptions and benefits.  It's the ideal endpoint to use when you need to get a full overview of a customer's status.  **Scopes**: `customers:read` `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | The customer external ID. | [required] |

### Return type

[**models::CustomerState**](CustomerState.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_list

> models::ListResourceCustomer customers_list(organization_id, email, query, page, limit, sorting, metadata)
List Customers

List customers.  **Scopes**: `customers:read` `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**email** | Option<**String**> | Filter by exact email. |  |
**query** | Option<**String**> | Filter by name, email, or external ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerSortProperty>**](models::CustomerSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceCustomer**](ListResource_Customer_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_update

> models::Customer customers_update(id, customer_update)
Update Customer

Update a customer.  **Scopes**: `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer ID. | [required] |
**customer_update** | [**CustomerUpdate**](CustomerUpdate.md) |  | [required] |

### Return type

[**models::Customer**](Customer.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customers_update_external

> models::Customer customers_update_external(external_id, customer_update_external_id)
Update Customer by External ID

Update a customer by external ID.  **Scopes**: `customers:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | The customer external ID. | [required] |
**customer_update_external_id** | [**CustomerUpdateExternalId**](CustomerUpdateExternalId.md) |  | [required] |

### Return type

[**models::Customer**](Customer.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

