# \OrdersApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_orders_confirm_retry_payment**](OrdersApi.md#customer_portal_orders_confirm_retry_payment) | **POST** /v1/customer-portal/orders/{id}/confirm-payment | Confirm Retry Payment
[**customer_portal_orders_generate_invoice**](OrdersApi.md#customer_portal_orders_generate_invoice) | **POST** /v1/customer-portal/orders/{id}/invoice | Generate Order Invoice
[**customer_portal_orders_get**](OrdersApi.md#customer_portal_orders_get) | **GET** /v1/customer-portal/orders/{id} | Get Order
[**customer_portal_orders_get_payment_status**](OrdersApi.md#customer_portal_orders_get_payment_status) | **GET** /v1/customer-portal/orders/{id}/payment-status | Get Order Payment Status
[**customer_portal_orders_invoice**](OrdersApi.md#customer_portal_orders_invoice) | **GET** /v1/customer-portal/orders/{id}/invoice | Get Order Invoice
[**customer_portal_orders_list**](OrdersApi.md#customer_portal_orders_list) | **GET** /v1/customer-portal/orders/ | List Orders
[**customer_portal_orders_update**](OrdersApi.md#customer_portal_orders_update) | **PATCH** /v1/customer-portal/orders/{id} | Update Order
[**orders_export**](OrdersApi.md#orders_export) | **GET** /v1/orders/export | Export Subscriptions
[**orders_generate_invoice**](OrdersApi.md#orders_generate_invoice) | **POST** /v1/orders/{id}/invoice | Generate Order Invoice
[**orders_get**](OrdersApi.md#orders_get) | **GET** /v1/orders/{id} | Get Order
[**orders_invoice**](OrdersApi.md#orders_invoice) | **GET** /v1/orders/{id}/invoice | Get Order Invoice
[**orders_list**](OrdersApi.md#orders_list) | **GET** /v1/orders/ | List Orders
[**orders_update**](OrdersApi.md#orders_update) | **PATCH** /v1/orders/{id} | Update Order



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

