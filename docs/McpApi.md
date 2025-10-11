# \McpApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_meters_get**](McpApi.md#customer_meters_get) | **GET** /v1/customer-meters/{id} | Get Customer Meter
[**customer_meters_list**](McpApi.md#customer_meters_list) | **GET** /v1/customer-meters/ | List Customer Meters
[**customers_create**](McpApi.md#customers_create) | **POST** /v1/customers/ | Create Customer
[**customers_delete**](McpApi.md#customers_delete) | **DELETE** /v1/customers/{id} | Delete Customer
[**customers_delete_external**](McpApi.md#customers_delete_external) | **DELETE** /v1/customers/external/{external_id} | Delete Customer by External ID
[**customers_export**](McpApi.md#customers_export) | **GET** /v1/customers/export | Export Customers
[**customers_get**](McpApi.md#customers_get) | **GET** /v1/customers/{id} | Get Customer
[**customers_get_balance**](McpApi.md#customers_get_balance) | **GET** /v1/customers/{id}/balance | Get Customer Balance
[**customers_get_external**](McpApi.md#customers_get_external) | **GET** /v1/customers/external/{external_id} | Get Customer by External ID
[**customers_get_state**](McpApi.md#customers_get_state) | **GET** /v1/customers/{id}/state | Get Customer State
[**customers_get_state_external**](McpApi.md#customers_get_state_external) | **GET** /v1/customers/external/{external_id}/state | Get Customer State by External ID
[**customers_list**](McpApi.md#customers_list) | **GET** /v1/customers/ | List Customers
[**customers_update**](McpApi.md#customers_update) | **PATCH** /v1/customers/{id} | Update Customer
[**customers_update_external**](McpApi.md#customers_update_external) | **PATCH** /v1/customers/external/{external_id} | Update Customer by External ID
[**metrics_get**](McpApi.md#metrics_get) | **GET** /v1/metrics/ | Get Metrics
[**metrics_limits**](McpApi.md#metrics_limits) | **GET** /v1/metrics/limits | Get Metrics Limits
[**orders_export**](McpApi.md#orders_export) | **GET** /v1/orders/export | Export Subscriptions
[**orders_generate_invoice**](McpApi.md#orders_generate_invoice) | **POST** /v1/orders/{id}/invoice | Generate Order Invoice
[**orders_get**](McpApi.md#orders_get) | **GET** /v1/orders/{id} | Get Order
[**orders_invoice**](McpApi.md#orders_invoice) | **GET** /v1/orders/{id}/invoice | Get Order Invoice
[**orders_list**](McpApi.md#orders_list) | **GET** /v1/orders/ | List Orders
[**orders_update**](McpApi.md#orders_update) | **PATCH** /v1/orders/{id} | Update Order
[**payments_get**](McpApi.md#payments_get) | **GET** /v1/payments/{id} | Get Payment
[**payments_list**](McpApi.md#payments_list) | **GET** /v1/payments/ | List Payments
[**products_create**](McpApi.md#products_create) | **POST** /v1/products/ | Create Product
[**products_get**](McpApi.md#products_get) | **GET** /v1/products/{id} | Get Product
[**products_list**](McpApi.md#products_list) | **GET** /v1/products/ | List Products
[**products_update**](McpApi.md#products_update) | **PATCH** /v1/products/{id} | Update Product
[**products_update_benefits**](McpApi.md#products_update_benefits) | **POST** /v1/products/{id}/benefits | Update Product Benefits
[**subscriptions_create**](McpApi.md#subscriptions_create) | **POST** /v1/subscriptions/ | Create Subscription
[**subscriptions_export**](McpApi.md#subscriptions_export) | **GET** /v1/subscriptions/export | Export Subscriptions
[**subscriptions_get**](McpApi.md#subscriptions_get) | **GET** /v1/subscriptions/{id} | Get Subscription
[**subscriptions_list**](McpApi.md#subscriptions_list) | **GET** /v1/subscriptions/ | List Subscriptions
[**subscriptions_revoke**](McpApi.md#subscriptions_revoke) | **DELETE** /v1/subscriptions/{id} | Revoke Subscription
[**subscriptions_update**](McpApi.md#subscriptions_update) | **PATCH** /v1/subscriptions/{id} | Update Subscription
[**wallets_get**](McpApi.md#wallets_get) | **GET** /v1/wallets/{id} | Get Wallet
[**wallets_list**](McpApi.md#wallets_list) | **GET** /v1/wallets/ | List Wallets
[**wallets_top_up**](McpApi.md#wallets_top_up) | **POST** /v1/wallets/{id}/top-up | Top-Up Wallet



## customer_meters_get

> models::CustomerMeter customer_meters_get(id)
Get Customer Meter

Get a customer meter by ID.  **Scopes**: `customer_meters:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The customer meter ID. | [required] |

### Return type

[**models::CustomerMeter**](CustomerMeter.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_meters_list

> models::ListResourceCustomerMeter customer_meters_list(organization_id, customer_id, external_customer_id, meter_id, page, limit, sorting)
List Customer Meters

List customer meters.  **Scopes**: `customer_meters:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**external_customer_id** | Option<[**ExternalCustomerIdFilter2**](.md)> | Filter by external customer ID. |  |
**meter_id** | Option<[**MeterIdFilter1**](.md)> | Filter by meter ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomerMeterSortProperty>**](models::CustomerMeterSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomerMeter**](ListResource_CustomerMeter_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
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


## metrics_get

> models::MetricsResponse metrics_get(start_date, end_date, interval, timezone, organization_id, product_id, billing_type, customer_id)
Get Metrics

Get metrics about your orders and subscriptions.  Currency values are output in cents.  **Scopes**: `metrics:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | Start date. | [required] |
**end_date** | **String** | End date. | [required] |
**interval** | [**TimeInterval**](.md) | Interval between two timestamps. | [required] |
**timezone** | Option<**String**> | Timezone to use for the timestamps. Default is UTC. |  |[default to UTC]
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**billing_type** | Option<[**ProductBillingTypeFilter1**](.md)> | Filter by billing type. `recurring` will filter data corresponding to subscriptions creations or renewals. `one_time` will filter data corresponding to one-time purchases. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |

### Return type

[**models::MetricsResponse**](MetricsResponse.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metrics_limits

> models::MetricsLimits metrics_limits()
Get Metrics Limits

Get the interval limits for the metrics endpoint.  **Scopes**: `metrics:read`

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MetricsLimits**](MetricsLimits.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders_export

> serde_json::Value orders_export(organization_id, product_id)
Export Subscriptions

Export orders as a CSV file.  **Scopes**: `orders:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders_generate_invoice

> serde_json::Value orders_generate_invoice(id)
Generate Order Invoice

Trigger generation of an order's invoice.  **Scopes**: `orders:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders_get

> models::Order orders_get(id)
Get Order

Get an order by ID.  **Scopes**: `orders:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders_invoice

> models::OrderInvoice orders_invoice(id)
Get Order Invoice

Get an order's invoice data.  **Scopes**: `orders:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |

### Return type

[**models::OrderInvoice**](OrderInvoice.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders_list

> models::ListResourceOrder orders_list(organization_id, product_id, product_billing_type, discount_id, customer_id, checkout_id, page, limit, sorting, metadata)
List Orders

List orders.  **Scopes**: `orders:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**product_billing_type** | Option<[**ProductBillingTypeFilter**](.md)> | Filter by product billing type. `recurring` will filter data corresponding to subscriptions creations or renewals. `one_time` will filter data corresponding to one-time purchases. |  |
**discount_id** | Option<[**DiscountIdFilter1**](.md)> | Filter by discount ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**checkout_id** | Option<[**CheckoutIdFilter**](.md)> | Filter by checkout ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::OrderSortProperty>**](models::OrderSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceOrder**](ListResource_Order_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders_update

> models::Order orders_update(id, order_update)
Update Order

Update an order.  **Scopes**: `orders:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The order ID. | [required] |
**order_update** | [**OrderUpdate**](OrderUpdate.md) |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_get

> models::Payment payments_get(id)
Get Payment

Get a payment by ID.  **Scopes**: `payments:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The payment ID. | [required] |

### Return type

[**models::Payment**](Payment.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_list

> models::ListResource payments_list(organization_id, checkout_id, order_id, status, method, customer_email, page, limit, sorting)
List Payments

List payments.  **Scopes**: `payments:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**checkout_id** | Option<[**CheckoutIdFilter1**](.md)> | Filter by checkout ID. |  |
**order_id** | Option<[**OrderIdFilter1**](.md)> | Filter by order ID. |  |
**status** | Option<[**StatusFilter1**](.md)> | Filter by payment status. |  |
**method** | Option<[**MethodFilter**](.md)> | Filter by payment method. |  |
**customer_email** | Option<[**CustomerEmailFilter**](.md)> | Filter by customer email. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::PaymentSortProperty>**](models::PaymentSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResource**](ListResource__.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_create

> models::Product products_create(product_create)
Create Product

Create a product.  **Scopes**: `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_create** | [**ProductCreate**](ProductCreate.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_get

> models::Product products_get(id)
Get Product

Get a product by ID.  **Scopes**: `products:read` `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_list

> models::ListResourceProduct products_list(id, organization_id, query, is_archived, is_recurring, benefit_id, page, limit, sorting, metadata)
List Products

List products.  **Scopes**: `products:read` `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**query** | Option<**String**> | Filter by product name. |  |
**is_archived** | Option<**bool**> | Filter on archived products. |  |
**is_recurring** | Option<**bool**> | Filter on recurring products. If `true`, only subscriptions tiers are returned. If `false`, only one-time purchase products are returned.  |  |
**benefit_id** | Option<[**BenefitIdFilter**](.md)> | Filter products granting specific benefit. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::ProductSortProperty>**](models::ProductSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceProduct**](ListResource_Product_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_update

> models::Product products_update(id, product_update)
Update Product

Update a product.  **Scopes**: `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**product_update** | [**ProductUpdate**](ProductUpdate.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_update_benefits

> models::Product products_update_benefits(id, product_benefits_update)
Update Product Benefits

Update benefits granted by a product.  **Scopes**: `products:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**product_benefits_update** | [**ProductBenefitsUpdate**](ProductBenefitsUpdate.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_create

> models::Subscription subscriptions_create(subscription_create)
Create Subscription

Create a subscription programmatically.  This endpoint only allows to create subscription on free products. For paid products, use the checkout flow.  No initial order will be created and no confirmation email will be sent.  **Scopes**: `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_create** | [**SubscriptionCreate**](SubscriptionCreate.md) |  | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_export

> serde_json::Value subscriptions_export(organization_id)
Export Subscriptions

Export subscriptions as a CSV file.  **Scopes**: `subscriptions:read` `subscriptions:write`

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


## subscriptions_get

> models::Subscription subscriptions_get(id)
Get Subscription

Get a subscription by ID.  **Scopes**: `subscriptions:read` `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_list

> models::ListResourceSubscription subscriptions_list(organization_id, product_id, customer_id, external_customer_id, discount_id, active, page, limit, sorting, metadata)
List Subscriptions

List subscriptions.  **Scopes**: `subscriptions:read` `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**external_customer_id** | Option<[**ExternalCustomerIdFilter**](.md)> | Filter by customer external ID. |  |
**discount_id** | Option<[**DiscountIdFilter**](.md)> | Filter by discount ID. |  |
**active** | Option<**bool**> | Filter by active or inactive subscription. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::SubscriptionSortProperty>**](models::SubscriptionSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceSubscription**](ListResource_Subscription_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_revoke

> models::Subscription subscriptions_revoke(id)
Revoke Subscription

Revoke a subscription, i.e cancel immediately.  **Scopes**: `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_update

> models::Subscription subscriptions_update(id, subscription_update)
Update Subscription

Update a subscription.  **Scopes**: `subscriptions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The subscription ID. | [required] |
**subscription_update** | [**SubscriptionUpdate**](SubscriptionUpdate.md) |  | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallets_get

> models::Wallet wallets_get(id)
Get Wallet

Get a wallet by ID.  **Scopes**: `wallets:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The wallet ID. | [required] |

### Return type

[**models::Wallet**](Wallet.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallets_list

> models::ListResourceWallet wallets_list(organization_id, customer_id, page, limit, sorting)
List Wallets

List wallets.  **Scopes**: `wallets:read`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::WalletSortProperty>**](models::WalletSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceWallet**](ListResource_Wallet_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallets_top_up

> models::Wallet wallets_top_up(id, wallet_top_up_create)
Top-Up Wallet

Top-up a wallet by adding funds to its balance.  The customer should have a valid payment method on file.  **Scopes**: `wallets:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The wallet ID. | [required] |
**wallet_top_up_create** | [**WalletTopUpCreate**](WalletTopUpCreate.md) |  | [required] |

### Return type

[**models::Wallet**](Wallet.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

