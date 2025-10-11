# \PublicApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**benefit_grants_list**](PublicApi.md#benefit_grants_list) | **GET** /v1/benefit-grants/ | List Benefit Grants
[**benefits_create**](PublicApi.md#benefits_create) | **POST** /v1/benefits/ | Create Benefit
[**benefits_delete**](PublicApi.md#benefits_delete) | **DELETE** /v1/benefits/{id} | Delete Benefit
[**benefits_get**](PublicApi.md#benefits_get) | **GET** /v1/benefits/{id} | Get Benefit
[**benefits_grants**](PublicApi.md#benefits_grants) | **GET** /v1/benefits/{id}/grants | List Benefit Grants
[**benefits_list**](PublicApi.md#benefits_list) | **GET** /v1/benefits/ | List Benefits
[**benefits_update**](PublicApi.md#benefits_update) | **PATCH** /v1/benefits/{id} | Update Benefit
[**checkout_links_create**](PublicApi.md#checkout_links_create) | **POST** /v1/checkout-links/ | Create Checkout Link
[**checkout_links_delete**](PublicApi.md#checkout_links_delete) | **DELETE** /v1/checkout-links/{id} | Delete Checkout Link
[**checkout_links_get**](PublicApi.md#checkout_links_get) | **GET** /v1/checkout-links/{id} | Get Checkout Link
[**checkout_links_list**](PublicApi.md#checkout_links_list) | **GET** /v1/checkout-links/ | List Checkout Links
[**checkout_links_update**](PublicApi.md#checkout_links_update) | **PATCH** /v1/checkout-links/{id} | Update Checkout Link
[**checkouts_client_confirm**](PublicApi.md#checkouts_client_confirm) | **POST** /v1/checkouts/client/{client_secret}/confirm | Confirm Checkout Session from Client
[**checkouts_client_get**](PublicApi.md#checkouts_client_get) | **GET** /v1/checkouts/client/{client_secret} | Get Checkout Session from Client
[**checkouts_client_update**](PublicApi.md#checkouts_client_update) | **PATCH** /v1/checkouts/client/{client_secret} | Update Checkout Session from Client
[**checkouts_create**](PublicApi.md#checkouts_create) | **POST** /v1/checkouts/ | Create Checkout Session
[**checkouts_get**](PublicApi.md#checkouts_get) | **GET** /v1/checkouts/{id} | Get Checkout Session
[**checkouts_list**](PublicApi.md#checkouts_list) | **GET** /v1/checkouts/ | List Checkout Sessions
[**checkouts_update**](PublicApi.md#checkouts_update) | **PATCH** /v1/checkouts/{id} | Update Checkout Session
[**custom_fields_create**](PublicApi.md#custom_fields_create) | **POST** /v1/custom-fields/ | Create Custom Field
[**custom_fields_delete**](PublicApi.md#custom_fields_delete) | **DELETE** /v1/custom-fields/{id} | Delete Custom Field
[**custom_fields_get**](PublicApi.md#custom_fields_get) | **GET** /v1/custom-fields/{id} | Get Custom Field
[**custom_fields_list**](PublicApi.md#custom_fields_list) | **GET** /v1/custom-fields/ | List Custom Fields
[**custom_fields_update**](PublicApi.md#custom_fields_update) | **PATCH** /v1/custom-fields/{id} | Update Custom Field
[**customer_meters_get**](PublicApi.md#customer_meters_get) | **GET** /v1/customer-meters/{id} | Get Customer Meter
[**customer_meters_list**](PublicApi.md#customer_meters_list) | **GET** /v1/customer-meters/ | List Customer Meters
[**customer_portal_benefit_grants_get**](PublicApi.md#customer_portal_benefit_grants_get) | **GET** /v1/customer-portal/benefit-grants/{id} | Get Benefit Grant
[**customer_portal_benefit_grants_list**](PublicApi.md#customer_portal_benefit_grants_list) | **GET** /v1/customer-portal/benefit-grants/ | List Benefit Grants
[**customer_portal_benefit_grants_update**](PublicApi.md#customer_portal_benefit_grants_update) | **PATCH** /v1/customer-portal/benefit-grants/{id} | Update Benefit Grant
[**customer_portal_customer_meters_get**](PublicApi.md#customer_portal_customer_meters_get) | **GET** /v1/customer-portal/meters/{id} | Get Customer Meter
[**customer_portal_customer_meters_list**](PublicApi.md#customer_portal_customer_meters_list) | **GET** /v1/customer-portal/meters/ | List Meters
[**customer_portal_customer_session_introspect**](PublicApi.md#customer_portal_customer_session_introspect) | **GET** /v1/customer-portal/customer-session/introspect | Introspect Customer Session
[**customer_portal_customers_add_payment_method**](PublicApi.md#customer_portal_customers_add_payment_method) | **POST** /v1/customer-portal/customers/me/payment-methods | Add Customer Payment Method
[**customer_portal_customers_confirm_payment_method**](PublicApi.md#customer_portal_customers_confirm_payment_method) | **POST** /v1/customer-portal/customers/me/payment-methods/confirm | Confirm Customer Payment Method
[**customer_portal_customers_delete_payment_method**](PublicApi.md#customer_portal_customers_delete_payment_method) | **DELETE** /v1/customer-portal/customers/me/payment-methods/{id} | Delete Customer Payment Method
[**customer_portal_customers_get**](PublicApi.md#customer_portal_customers_get) | **GET** /v1/customer-portal/customers/me | Get Customer
[**customer_portal_customers_list_payment_methods**](PublicApi.md#customer_portal_customers_list_payment_methods) | **GET** /v1/customer-portal/customers/me/payment-methods | List Customer Payment Methods
[**customer_portal_customers_update**](PublicApi.md#customer_portal_customers_update) | **PATCH** /v1/customer-portal/customers/me | Update Customer
[**customer_portal_downloadables_list**](PublicApi.md#customer_portal_downloadables_list) | **GET** /v1/customer-portal/downloadables/ | List Downloadables
[**customer_portal_license_keys_activate**](PublicApi.md#customer_portal_license_keys_activate) | **POST** /v1/customer-portal/license-keys/activate | Activate License Key
[**customer_portal_license_keys_deactivate**](PublicApi.md#customer_portal_license_keys_deactivate) | **POST** /v1/customer-portal/license-keys/deactivate | Deactivate License Key
[**customer_portal_license_keys_get**](PublicApi.md#customer_portal_license_keys_get) | **GET** /v1/customer-portal/license-keys/{id} | Get License Key
[**customer_portal_license_keys_list**](PublicApi.md#customer_portal_license_keys_list) | **GET** /v1/customer-portal/license-keys/ | List License Keys
[**customer_portal_license_keys_validate**](PublicApi.md#customer_portal_license_keys_validate) | **POST** /v1/customer-portal/license-keys/validate | Validate License Key
[**customer_portal_orders_confirm_retry_payment**](PublicApi.md#customer_portal_orders_confirm_retry_payment) | **POST** /v1/customer-portal/orders/{id}/confirm-payment | Confirm Retry Payment
[**customer_portal_orders_generate_invoice**](PublicApi.md#customer_portal_orders_generate_invoice) | **POST** /v1/customer-portal/orders/{id}/invoice | Generate Order Invoice
[**customer_portal_orders_get**](PublicApi.md#customer_portal_orders_get) | **GET** /v1/customer-portal/orders/{id} | Get Order
[**customer_portal_orders_get_payment_status**](PublicApi.md#customer_portal_orders_get_payment_status) | **GET** /v1/customer-portal/orders/{id}/payment-status | Get Order Payment Status
[**customer_portal_orders_invoice**](PublicApi.md#customer_portal_orders_invoice) | **GET** /v1/customer-portal/orders/{id}/invoice | Get Order Invoice
[**customer_portal_orders_list**](PublicApi.md#customer_portal_orders_list) | **GET** /v1/customer-portal/orders/ | List Orders
[**customer_portal_orders_update**](PublicApi.md#customer_portal_orders_update) | **PATCH** /v1/customer-portal/orders/{id} | Update Order
[**customer_portal_organizations_get**](PublicApi.md#customer_portal_organizations_get) | **GET** /v1/customer-portal/organizations/{slug} | Get Organization
[**customer_portal_seats_assign_seat**](PublicApi.md#customer_portal_seats_assign_seat) | **POST** /v1/customer-portal/seats | Assign Seat
[**customer_portal_seats_list_claimed_subscriptions**](PublicApi.md#customer_portal_seats_list_claimed_subscriptions) | **GET** /v1/customer-portal/seats/subscriptions | List Claimed Subscriptions
[**customer_portal_seats_list_seats**](PublicApi.md#customer_portal_seats_list_seats) | **GET** /v1/customer-portal/seats | List Seats
[**customer_portal_seats_resend_invitation**](PublicApi.md#customer_portal_seats_resend_invitation) | **POST** /v1/customer-portal/seats/{seat_id}/resend | Resend Invitation
[**customer_portal_seats_revoke_seat**](PublicApi.md#customer_portal_seats_revoke_seat) | **DELETE** /v1/customer-portal/seats/{seat_id} | Revoke Seat
[**customer_portal_subscriptions_cancel**](PublicApi.md#customer_portal_subscriptions_cancel) | **DELETE** /v1/customer-portal/subscriptions/{id} | Cancel Subscription
[**customer_portal_subscriptions_get**](PublicApi.md#customer_portal_subscriptions_get) | **GET** /v1/customer-portal/subscriptions/{id} | Get Subscription
[**customer_portal_subscriptions_list**](PublicApi.md#customer_portal_subscriptions_list) | **GET** /v1/customer-portal/subscriptions/ | List Subscriptions
[**customer_portal_subscriptions_update**](PublicApi.md#customer_portal_subscriptions_update) | **PATCH** /v1/customer-portal/subscriptions/{id} | Update Subscription
[**customer_portal_wallets_get**](PublicApi.md#customer_portal_wallets_get) | **GET** /v1/customer-portal/wallets/{id} | Get Wallet
[**customer_portal_wallets_list**](PublicApi.md#customer_portal_wallets_list) | **GET** /v1/customer-portal/wallets/ | List Wallets
[**customer_seats_assign_seat**](PublicApi.md#customer_seats_assign_seat) | **POST** /v1/customer-seats | Assign Seat
[**customer_seats_claim_seat**](PublicApi.md#customer_seats_claim_seat) | **POST** /v1/customer-seats/claim | Claim Seat
[**customer_seats_get_claim_info**](PublicApi.md#customer_seats_get_claim_info) | **GET** /v1/customer-seats/claim/{invitation_token} | Get Claim Info
[**customer_seats_list_seats**](PublicApi.md#customer_seats_list_seats) | **GET** /v1/customer-seats | List Seats
[**customer_seats_resend_invitation**](PublicApi.md#customer_seats_resend_invitation) | **POST** /v1/customer-seats/{seat_id}/resend | Resend Invitation
[**customer_seats_revoke_seat**](PublicApi.md#customer_seats_revoke_seat) | **DELETE** /v1/customer-seats/{seat_id} | Revoke Seat
[**customer_sessions_create**](PublicApi.md#customer_sessions_create) | **POST** /v1/customer-sessions/ | Create Customer Session
[**customers_create**](PublicApi.md#customers_create) | **POST** /v1/customers/ | Create Customer
[**customers_delete**](PublicApi.md#customers_delete) | **DELETE** /v1/customers/{id} | Delete Customer
[**customers_delete_external**](PublicApi.md#customers_delete_external) | **DELETE** /v1/customers/external/{external_id} | Delete Customer by External ID
[**customers_export**](PublicApi.md#customers_export) | **GET** /v1/customers/export | Export Customers
[**customers_get**](PublicApi.md#customers_get) | **GET** /v1/customers/{id} | Get Customer
[**customers_get_balance**](PublicApi.md#customers_get_balance) | **GET** /v1/customers/{id}/balance | Get Customer Balance
[**customers_get_external**](PublicApi.md#customers_get_external) | **GET** /v1/customers/external/{external_id} | Get Customer by External ID
[**customers_get_state**](PublicApi.md#customers_get_state) | **GET** /v1/customers/{id}/state | Get Customer State
[**customers_get_state_external**](PublicApi.md#customers_get_state_external) | **GET** /v1/customers/external/{external_id}/state | Get Customer State by External ID
[**customers_list**](PublicApi.md#customers_list) | **GET** /v1/customers/ | List Customers
[**customers_update**](PublicApi.md#customers_update) | **PATCH** /v1/customers/{id} | Update Customer
[**customers_update_external**](PublicApi.md#customers_update_external) | **PATCH** /v1/customers/external/{external_id} | Update Customer by External ID
[**discounts_create**](PublicApi.md#discounts_create) | **POST** /v1/discounts/ | Create Discount
[**discounts_delete**](PublicApi.md#discounts_delete) | **DELETE** /v1/discounts/{id} | Delete Discount
[**discounts_get**](PublicApi.md#discounts_get) | **GET** /v1/discounts/{id} | Get Discount
[**discounts_list**](PublicApi.md#discounts_list) | **GET** /v1/discounts/ | List Discounts
[**discounts_update**](PublicApi.md#discounts_update) | **PATCH** /v1/discounts/{id} | Update Discount
[**events_get**](PublicApi.md#events_get) | **GET** /v1/events/{id} | Get Event
[**events_ingest**](PublicApi.md#events_ingest) | **POST** /v1/events/ingest | Ingest Events
[**events_list**](PublicApi.md#events_list) | **GET** /v1/events/ | List Events
[**events_list_names**](PublicApi.md#events_list_names) | **GET** /v1/events/names | List Event Names
[**files_create**](PublicApi.md#files_create) | **POST** /v1/files/ | Create File
[**files_delete**](PublicApi.md#files_delete) | **DELETE** /v1/files/{id} | Delete File
[**files_list**](PublicApi.md#files_list) | **GET** /v1/files/ | List Files
[**files_update**](PublicApi.md#files_update) | **PATCH** /v1/files/{id} | Update File
[**files_uploaded**](PublicApi.md#files_uploaded) | **POST** /v1/files/{id}/uploaded | Complete File Upload
[**license_keys_activate**](PublicApi.md#license_keys_activate) | **POST** /v1/license-keys/activate | Activate License Key
[**license_keys_deactivate**](PublicApi.md#license_keys_deactivate) | **POST** /v1/license-keys/deactivate | Deactivate License Key
[**license_keys_get**](PublicApi.md#license_keys_get) | **GET** /v1/license-keys/{id} | Get License Key
[**license_keys_get_activation**](PublicApi.md#license_keys_get_activation) | **GET** /v1/license-keys/{id}/activations/{activation_id} | Get Activation
[**license_keys_list**](PublicApi.md#license_keys_list) | **GET** /v1/license-keys/ | List License Keys
[**license_keys_update**](PublicApi.md#license_keys_update) | **PATCH** /v1/license-keys/{id} | Update License Key
[**license_keys_validate**](PublicApi.md#license_keys_validate) | **POST** /v1/license-keys/validate | Validate License Key
[**meters_create**](PublicApi.md#meters_create) | **POST** /v1/meters/ | Create Meter
[**meters_get**](PublicApi.md#meters_get) | **GET** /v1/meters/{id} | Get Meter
[**meters_list**](PublicApi.md#meters_list) | **GET** /v1/meters/ | List Meters
[**meters_quantities**](PublicApi.md#meters_quantities) | **GET** /v1/meters/{id}/quantities | Get Meter Quantities
[**meters_update**](PublicApi.md#meters_update) | **PATCH** /v1/meters/{id} | Update Meter
[**metrics_get**](PublicApi.md#metrics_get) | **GET** /v1/metrics/ | Get Metrics
[**metrics_limits**](PublicApi.md#metrics_limits) | **GET** /v1/metrics/limits | Get Metrics Limits
[**oauth2_authorize**](PublicApi.md#oauth2_authorize) | **GET** /v1/oauth2/authorize | Authorize
[**oauth2_clients_oauth2_create_client**](PublicApi.md#oauth2_clients_oauth2_create_client) | **POST** /v1/oauth2/register | Create Client
[**oauth2_clients_oauth2_delete_client**](PublicApi.md#oauth2_clients_oauth2_delete_client) | **DELETE** /v1/oauth2/register/{client_id} | Delete Client
[**oauth2_clients_oauth2_get_client**](PublicApi.md#oauth2_clients_oauth2_get_client) | **GET** /v1/oauth2/register/{client_id} | Get Client
[**oauth2_clients_oauth2_update_client**](PublicApi.md#oauth2_clients_oauth2_update_client) | **PUT** /v1/oauth2/register/{client_id} | Update Client
[**oauth2_introspect_token**](PublicApi.md#oauth2_introspect_token) | **POST** /v1/oauth2/introspect | Introspect Token
[**oauth2_request_token**](PublicApi.md#oauth2_request_token) | **POST** /v1/oauth2/token | Request Token
[**oauth2_revoke_token**](PublicApi.md#oauth2_revoke_token) | **POST** /v1/oauth2/revoke | Revoke Token
[**oauth2_userinfo**](PublicApi.md#oauth2_userinfo) | **GET** /v1/oauth2/userinfo | Get User Info
[**orders_export**](PublicApi.md#orders_export) | **GET** /v1/orders/export | Export Subscriptions
[**orders_generate_invoice**](PublicApi.md#orders_generate_invoice) | **POST** /v1/orders/{id}/invoice | Generate Order Invoice
[**orders_get**](PublicApi.md#orders_get) | **GET** /v1/orders/{id} | Get Order
[**orders_invoice**](PublicApi.md#orders_invoice) | **GET** /v1/orders/{id}/invoice | Get Order Invoice
[**orders_list**](PublicApi.md#orders_list) | **GET** /v1/orders/ | List Orders
[**orders_update**](PublicApi.md#orders_update) | **PATCH** /v1/orders/{id} | Update Order
[**organizations_create**](PublicApi.md#organizations_create) | **POST** /v1/organizations/ | Create Organization
[**organizations_get**](PublicApi.md#organizations_get) | **GET** /v1/organizations/{id} | Get Organization
[**organizations_list**](PublicApi.md#organizations_list) | **GET** /v1/organizations/ | List Organizations
[**organizations_update**](PublicApi.md#organizations_update) | **PATCH** /v1/organizations/{id} | Update Organization
[**payments_get**](PublicApi.md#payments_get) | **GET** /v1/payments/{id} | Get Payment
[**payments_list**](PublicApi.md#payments_list) | **GET** /v1/payments/ | List Payments
[**products_create**](PublicApi.md#products_create) | **POST** /v1/products/ | Create Product
[**products_get**](PublicApi.md#products_get) | **GET** /v1/products/{id} | Get Product
[**products_list**](PublicApi.md#products_list) | **GET** /v1/products/ | List Products
[**products_update**](PublicApi.md#products_update) | **PATCH** /v1/products/{id} | Update Product
[**products_update_benefits**](PublicApi.md#products_update_benefits) | **POST** /v1/products/{id}/benefits | Update Product Benefits
[**refunds_create**](PublicApi.md#refunds_create) | **POST** /v1/refunds/ | Create Refund
[**refunds_list**](PublicApi.md#refunds_list) | **GET** /v1/refunds/ | List Refunds
[**subscriptions_create**](PublicApi.md#subscriptions_create) | **POST** /v1/subscriptions/ | Create Subscription
[**subscriptions_export**](PublicApi.md#subscriptions_export) | **GET** /v1/subscriptions/export | Export Subscriptions
[**subscriptions_get**](PublicApi.md#subscriptions_get) | **GET** /v1/subscriptions/{id} | Get Subscription
[**subscriptions_list**](PublicApi.md#subscriptions_list) | **GET** /v1/subscriptions/ | List Subscriptions
[**subscriptions_revoke**](PublicApi.md#subscriptions_revoke) | **DELETE** /v1/subscriptions/{id} | Revoke Subscription
[**subscriptions_update**](PublicApi.md#subscriptions_update) | **PATCH** /v1/subscriptions/{id} | Update Subscription
[**wallets_get**](PublicApi.md#wallets_get) | **GET** /v1/wallets/{id} | Get Wallet
[**wallets_list**](PublicApi.md#wallets_list) | **GET** /v1/wallets/ | List Wallets
[**wallets_top_up**](PublicApi.md#wallets_top_up) | **POST** /v1/wallets/{id}/top-up | Top-Up Wallet
[**webhooks_create_webhook_endpoint**](PublicApi.md#webhooks_create_webhook_endpoint) | **POST** /v1/webhooks/endpoints | Create Webhook Endpoint
[**webhooks_delete_webhook_endpoint**](PublicApi.md#webhooks_delete_webhook_endpoint) | **DELETE** /v1/webhooks/endpoints/{id} | Delete Webhook Endpoint
[**webhooks_get_webhook_endpoint**](PublicApi.md#webhooks_get_webhook_endpoint) | **GET** /v1/webhooks/endpoints/{id} | Get Webhook Endpoint
[**webhooks_list_webhook_deliveries**](PublicApi.md#webhooks_list_webhook_deliveries) | **GET** /v1/webhooks/deliveries | List Webhook Deliveries
[**webhooks_list_webhook_endpoints**](PublicApi.md#webhooks_list_webhook_endpoints) | **GET** /v1/webhooks/endpoints | List Webhook Endpoints
[**webhooks_redeliver_webhook_event**](PublicApi.md#webhooks_redeliver_webhook_event) | **POST** /v1/webhooks/events/{id}/redeliver | Redeliver Webhook Event
[**webhooks_reset_webhook_endpoint_secret**](PublicApi.md#webhooks_reset_webhook_endpoint_secret) | **PATCH** /v1/webhooks/endpoints/{id}/secret | Reset Webhook Endpoint Secret
[**webhooks_update_webhook_endpoint**](PublicApi.md#webhooks_update_webhook_endpoint) | **PATCH** /v1/webhooks/endpoints/{id} | Update Webhook Endpoint



## benefit_grants_list

> models::ListResourceBenefitGrant benefit_grants_list(organization_id, customer_id, is_granted, page, limit, sorting)
List Benefit Grants

List benefit grants across all benefits for the authenticated organization.  **Scopes**: `benefits:read` `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**is_granted** | Option<**bool**> | Filter by granted status. If `true`, only granted benefits will be returned. If `false`, only revoked benefits will be returned.  |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::BenefitGrantSortProperty>**](models::BenefitGrantSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceBenefitGrant**](ListResource_BenefitGrant_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_create

> models::Benefit benefits_create(benefit_create)
Create Benefit

Create a benefit.  **Scopes**: `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**benefit_create** | [**BenefitCreate**](BenefitCreate.md) |  | [required] |

### Return type

[**models::Benefit**](Benefit.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_delete

> benefits_delete(id)
Delete Benefit

Delete a benefit.  > [!WARNING] > Every grants associated with the benefit will be revoked. > Users will lose access to the benefit.  **Scopes**: `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_get

> models::Benefit benefits_get(id)
Get Benefit

Get a benefit by ID.  **Scopes**: `benefits:read` `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Benefit**](Benefit.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_grants

> models::ListResourceBenefitGrant benefits_grants(id, is_granted, customer_id, page, limit)
List Benefit Grants

List the individual grants for a benefit.  It's especially useful to check if a user has been granted a benefit.  **Scopes**: `benefits:read` `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**is_granted** | Option<**bool**> | Filter by granted status. If `true`, only granted benefits will be returned. If `false`, only revoked benefits will be returned.  |  |
**customer_id** | Option<**String**> | Filter by customer. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceBenefitGrant**](ListResource_BenefitGrant_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_list

> models::ListResourceBenefit benefits_list(organization_id, r#type, query, page, limit, sorting, metadata)
List Benefits

List benefits.  **Scopes**: `benefits:read` `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**r#type** | Option<[**BenefitTypeFilter**](.md)> | Filter by benefit type. |  |
**query** | Option<**String**> | Filter by description. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::BenefitSortProperty>**](models::BenefitSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceBenefit**](ListResource_Benefit_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_update

> models::Benefit benefits_update(id, benefit_update)
Update Benefit

Update a benefit.  **Scopes**: `benefits:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**benefit_update** | [**BenefitUpdate**](BenefitUpdate.md) |  | [required] |

### Return type

[**models::Benefit**](Benefit.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_create

> models::CheckoutLink checkout_links_create(checkout_link_create)
Create Checkout Link

Create a checkout link.  **Scopes**: `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checkout_link_create** | [**CheckoutLinkCreate**](CheckoutLinkCreate.md) |  | [required] |

### Return type

[**models::CheckoutLink**](CheckoutLink.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_delete

> checkout_links_delete(id)
Delete Checkout Link

Delete a checkout link.  **Scopes**: `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout link ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_get

> models::CheckoutLink checkout_links_get(id)
Get Checkout Link

Get a checkout link by ID.  **Scopes**: `checkout_links:read` `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout link ID. | [required] |

### Return type

[**models::CheckoutLink**](CheckoutLink.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_list

> models::ListResourceCheckoutLink checkout_links_list(organization_id, product_id, page, limit, sorting)
List Checkout Links

List checkout links.  **Scopes**: `checkout_links:read` `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CheckoutLinkSortProperty>**](models::CheckoutLinkSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCheckoutLink**](ListResource_CheckoutLink_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkout_links_update

> models::CheckoutLink checkout_links_update(id, checkout_link_update)
Update Checkout Link

Update a checkout link.  **Scopes**: `checkout_links:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout link ID. | [required] |
**checkout_link_update** | [**CheckoutLinkUpdate**](CheckoutLinkUpdate.md) |  | [required] |

### Return type

[**models::CheckoutLink**](CheckoutLink.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_client_confirm

> models::CheckoutPublicConfirmed checkouts_client_confirm(client_secret, checkout_confirm_stripe)
Confirm Checkout Session from Client

Confirm a checkout session by client secret.  Orders and subscriptions will be processed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_secret** | **String** | The checkout session client secret. | [required] |
**checkout_confirm_stripe** | [**CheckoutConfirmStripe**](CheckoutConfirmStripe.md) |  | [required] |

### Return type

[**models::CheckoutPublicConfirmed**](CheckoutPublicConfirmed.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_client_get

> models::CheckoutPublic checkouts_client_get(client_secret)
Get Checkout Session from Client

Get a checkout session by client secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_secret** | **String** | The checkout session client secret. | [required] |

### Return type

[**models::CheckoutPublic**](CheckoutPublic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_client_update

> models::CheckoutPublic checkouts_client_update(client_secret, checkout_update_public)
Update Checkout Session from Client

Update a checkout session by client secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_secret** | **String** | The checkout session client secret. | [required] |
**checkout_update_public** | [**CheckoutUpdatePublic**](CheckoutUpdatePublic.md) |  | [required] |

### Return type

[**models::CheckoutPublic**](CheckoutPublic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_create

> models::Checkout checkouts_create(body)
Create Checkout Session

Create a checkout session.  **Scopes**: `checkouts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **models::CheckoutProductsCreate** |  | [required] |

### Return type

[**models::Checkout**](Checkout.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_get

> models::Checkout checkouts_get(id)
Get Checkout Session

Get a checkout session by ID.  **Scopes**: `checkouts:read` `checkouts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout session ID. | [required] |

### Return type

[**models::Checkout**](Checkout.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_list

> models::ListResourceCheckout checkouts_list(organization_id, product_id, customer_id, status, query, page, limit, sorting)
List Checkout Sessions

List checkout sessions.  **Scopes**: `checkouts:read` `checkouts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**product_id** | Option<[**ProductIdFilter**](.md)> | Filter by product ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**status** | Option<[**StatusFilter**](.md)> | Filter by checkout session status. |  |
**query** | Option<**String**> | Filter by customer email. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CheckoutSortProperty>**](models::CheckoutSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCheckout**](ListResource_Checkout_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkouts_update

> models::Checkout checkouts_update(id, checkout_update)
Update Checkout Session

Update a checkout session.  **Scopes**: `checkouts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The checkout session ID. | [required] |
**checkout_update** | [**CheckoutUpdate**](CheckoutUpdate.md) |  | [required] |

### Return type

[**models::Checkout**](Checkout.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_create

> models::CustomField custom_fields_create(custom_field_create)
Create Custom Field

Create a custom field.  **Scopes**: `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_field_create** | [**CustomFieldCreate**](CustomFieldCreate.md) |  | [required] |

### Return type

[**models::CustomField**](CustomField.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_delete

> custom_fields_delete(id)
Delete Custom Field

Delete a custom field.  **Scopes**: `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The custom field ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_get

> models::CustomField custom_fields_get(id)
Get Custom Field

Get a custom field by ID.  **Scopes**: `custom_fields:read` `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The custom field ID. | [required] |

### Return type

[**models::CustomField**](CustomField.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_list

> models::ListResourceCustomField custom_fields_list(organization_id, query, r#type, page, limit, sorting)
List Custom Fields

List custom fields.  **Scopes**: `custom_fields:read` `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**query** | Option<**String**> | Filter by custom field name or slug. |  |
**r#type** | Option<[**CustomFieldTypeFilter**](.md)> | Filter by custom field type. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::CustomFieldSortProperty>**](models::CustomFieldSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceCustomField**](ListResource_CustomField_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_fields_update

> models::CustomField custom_fields_update(id, custom_field_update)
Update Custom Field

Update a custom field.  **Scopes**: `custom_fields:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The custom field ID. | [required] |
**custom_field_update** | [**CustomFieldUpdate**](CustomFieldUpdate.md) |  | [required] |

### Return type

[**models::CustomField**](CustomField.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## customer_sessions_create

> models::CustomerSession customer_sessions_create(customer_session_create)
Create Customer Session

Create a customer session.  **Scopes**: `customer_sessions:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_session_create** | [**CustomerSessionCreate**](CustomerSessionCreate.md) |  | [required] |

### Return type

[**models::CustomerSession**](CustomerSession.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

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


## discounts_create

> models::Discount discounts_create(discount_create)
Create Discount

Create a discount.  **Scopes**: `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discount_create** | [**DiscountCreate**](DiscountCreate.md) |  | [required] |

### Return type

[**models::Discount**](Discount.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discounts_delete

> discounts_delete(id)
Delete Discount

Delete a discount.  **Scopes**: `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The discount ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discounts_get

> models::Discount discounts_get(id)
Get Discount

Get a discount by ID.  **Scopes**: `discounts:read` `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The discount ID. | [required] |

### Return type

[**models::Discount**](Discount.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discounts_list

> models::ListResourceDiscount discounts_list(organization_id, query, page, limit, sorting)
List Discounts

List discounts.  **Scopes**: `discounts:read` `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**query** | Option<**String**> | Filter by name. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::DiscountSortProperty>**](models::DiscountSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceDiscount**](ListResource_Discount_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discounts_update

> models::Discount discounts_update(id, discount_update)
Update Discount

Update a discount.  **Scopes**: `discounts:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The discount ID. | [required] |
**discount_update** | [**DiscountUpdate**](DiscountUpdate.md) |  | [required] |

### Return type

[**models::Discount**](Discount.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_get

> models::Event events_get(id)
Get Event

Get an event by ID.  **Scopes**: `events:read` `events:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The event ID. | [required] |

### Return type

[**models::Event**](Event.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_ingest

> models::EventsIngestResponse events_ingest(events_ingest)
Ingest Events

Ingest batch of events.  **Scopes**: `events:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**events_ingest** | [**EventsIngest**](EventsIngest.md) |  | [required] |

### Return type

[**models::EventsIngestResponse**](EventsIngestResponse.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_list

> models::ListResourceEvent events_list(filter, start_timestamp, end_timestamp, organization_id, customer_id, external_customer_id, meter_id, name, source, query, page, limit, sorting, metadata)
List Events

List events.  **Scopes**: `events:read` `events:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter events following filter clauses. JSON string following the same schema a meter filter clause.  |  |
**start_timestamp** | Option<**String**> | Filter events after this timestamp. |  |
**end_timestamp** | Option<**String**> | Filter events before this timestamp. |  |
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**external_customer_id** | Option<[**ExternalCustomerIdFilter1**](.md)> | Filter by external customer ID. |  |
**meter_id** | Option<**String**> | Filter by a meter filter clause. |  |
**name** | Option<[**NameFilter**](.md)> | Filter by event name. |  |
**source** | Option<[**SourceFilter**](.md)> | Filter by event source. |  |
**query** | Option<**String**> | Query to filter events. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::EventSortProperty>**](models::EventSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceEvent**](ListResource_Event_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_list_names

> models::ListResourceEventName events_list_names(organization_id, customer_id, external_customer_id, source, query, page, limit, sorting)
List Event Names

List event names.  **Scopes**: `events:read` `events:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**external_customer_id** | Option<[**ExternalCustomerIdFilter2**](.md)> | Filter by external customer ID. |  |
**source** | Option<[**SourceFilter**](.md)> | Filter by event source. |  |
**query** | Option<**String**> | Query to filter event names. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::EventNamesSortProperty>**](models::EventNamesSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceEventName**](ListResource_EventName_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_create

> models::FileUpload files_create(file_create)
Create File

Create a file.  **Scopes**: `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_create** | [**FileCreate**](FileCreate.md) |  | [required] |

### Return type

[**models::FileUpload**](FileUpload.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_delete

> files_delete(id)
Delete File

Delete a file.  **Scopes**: `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_list

> models::ListResourceFileRead files_list(organization_id, ids, page, limit)
List Files

List files.  **Scopes**: `files:read` `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**ids** | Option<[**FileIdFilter**](.md)> | Filter by file ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceFileRead**](ListResource_FileRead_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_update

> models::ResponseFilesUpdate files_update(id, file_patch)
Update File

Update a file.  **Scopes**: `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The file ID. | [required] |
**file_patch** | [**FilePatch**](FilePatch.md) |  | [required] |

### Return type

[**models::ResponseFilesUpdate**](Response_Files_Update.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_uploaded

> models::ResponseFilesUploaded files_uploaded(id, file_upload_completed)
Complete File Upload

Complete a file upload.  **Scopes**: `files:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The file ID. | [required] |
**file_upload_completed** | [**FileUploadCompleted**](FileUploadCompleted.md) |  | [required] |

### Return type

[**models::ResponseFilesUploaded**](Response_Files_Uploaded.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_keys_activate

> models::LicenseKeyActivationRead license_keys_activate(license_key_activate)
Activate License Key

Activate a license key instance.  **Scopes**: `license_keys:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key_activate** | [**LicenseKeyActivate**](LicenseKeyActivate.md) |  | [required] |

### Return type

[**models::LicenseKeyActivationRead**](LicenseKeyActivationRead.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_keys_deactivate

> license_keys_deactivate(license_key_deactivate)
Deactivate License Key

Deactivate a license key instance.  **Scopes**: `license_keys:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key_deactivate** | [**LicenseKeyDeactivate**](LicenseKeyDeactivate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_keys_get

> models::LicenseKeyWithActivations license_keys_get(id)
Get License Key

Get a license key.  **Scopes**: `license_keys:read` `license_keys:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::LicenseKeyWithActivations**](LicenseKeyWithActivations.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_keys_get_activation

> models::LicenseKeyActivationRead license_keys_get_activation(id, activation_id)
Get Activation

Get a license key activation.  **Scopes**: `license_keys:read` `license_keys:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**activation_id** | **String** |  | [required] |

### Return type

[**models::LicenseKeyActivationRead**](LicenseKeyActivationRead.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_keys_list

> models::ListResourceLicenseKeyRead license_keys_list(organization_id, benefit_id, page, limit)
List License Keys

Get license keys connected to the given organization & filters.  **Scopes**: `license_keys:read` `license_keys:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**benefit_id** | Option<[**BenefitIdFilter1**](.md)> | Filter by benefit ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceLicenseKeyRead**](ListResource_LicenseKeyRead_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_keys_update

> models::LicenseKeyRead license_keys_update(id, license_key_update)
Update License Key

Update a license key.  **Scopes**: `license_keys:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**license_key_update** | [**LicenseKeyUpdate**](LicenseKeyUpdate.md) |  | [required] |

### Return type

[**models::LicenseKeyRead**](LicenseKeyRead.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_keys_validate

> models::ValidatedLicenseKey license_keys_validate(license_key_validate)
Validate License Key

Validate a license key.  **Scopes**: `license_keys:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key_validate** | [**LicenseKeyValidate**](LicenseKeyValidate.md) |  | [required] |

### Return type

[**models::ValidatedLicenseKey**](ValidatedLicenseKey.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_create

> models::Meter meters_create(meter_create)
Create Meter

Create a meter.  **Scopes**: `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meter_create** | [**MeterCreate**](MeterCreate.md) |  | [required] |

### Return type

[**models::Meter**](Meter.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_get

> models::Meter meters_get(id)
Get Meter

Get a meter by ID.  **Scopes**: `meters:read` `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The meter ID. | [required] |

### Return type

[**models::Meter**](Meter.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_list

> models::ListResourceMeter meters_list(organization_id, query, is_archived, page, limit, sorting, metadata)
List Meters

List meters.  **Scopes**: `meters:read` `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**query** | Option<**String**> | Filter by name. |  |
**is_archived** | Option<**bool**> | Filter on archived meters. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::MeterSortProperty>**](models::MeterSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::ListResourceMeter**](ListResource_Meter_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_quantities

> models::MeterQuantities meters_quantities(id, start_timestamp, end_timestamp, interval, customer_id, external_customer_id, customer_aggregation_function, metadata)
Get Meter Quantities

Get quantities of a meter over a time period.  **Scopes**: `meters:read` `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The meter ID. | [required] |
**start_timestamp** | **String** | Start timestamp. | [required] |
**end_timestamp** | **String** | End timestamp. | [required] |
**interval** | [**TimeInterval**](.md) | Interval between two timestamps. | [required] |
**customer_id** | Option<**String**> | Filter by customer ID. |  |
**external_customer_id** | Option<**String**> | Filter by external customer ID. |  |
**customer_aggregation_function** | Option<[**AggregationFunction**](.md)> | If set, will first compute the quantities per customer before aggregating them using the given function. If not set, the quantities will be aggregated across all events. |  |
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataQueryValue>**](models::MetadataQueryValue.md)> | Filter by metadata key-value pairs. It uses the `deepObject` style, e.g. `?metadata[key]=value`. |  |

### Return type

[**models::MeterQuantities**](MeterQuantities.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meters_update

> models::Meter meters_update(id, meter_update)
Update Meter

Update a meter.  **Scopes**: `meters:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The meter ID. | [required] |
**meter_update** | [**MeterUpdate**](MeterUpdate.md) |  | [required] |

### Return type

[**models::Meter**](Meter.md)

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


## oauth2_authorize

> models::ResponseOauth2Authorize oauth2_authorize()
Authorize

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ResponseOauth2Authorize**](Response_Oauth2_Authorize.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_clients_oauth2_create_client

> serde_json::Value oauth2_clients_oauth2_create_client(o_auth2_client_configuration)
Create Client

Create an OAuth2 client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth2_client_configuration** | [**OAuth2ClientConfiguration**](OAuth2ClientConfiguration.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_clients_oauth2_delete_client

> serde_json::Value oauth2_clients_oauth2_delete_client(client_id)
Delete Client

Delete an OAuth2 client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_clients_oauth2_get_client

> serde_json::Value oauth2_clients_oauth2_get_client(client_id)
Get Client

Get an OAuth2 client by Client ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_clients_oauth2_update_client

> serde_json::Value oauth2_clients_oauth2_update_client(client_id, o_auth2_client_configuration_update)
Update Client

Update an OAuth2 client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** |  | [required] |
**o_auth2_client_configuration_update** | [**OAuth2ClientConfigurationUpdate**](OAuth2ClientConfigurationUpdate.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_introspect_token

> models::IntrospectTokenResponse oauth2_introspect_token(token, client_id, client_secret, token_type_hint)
Introspect Token

Get information about an access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**client_id** | **String** |  | [required] |
**client_secret** | **String** |  | [required] |
**token_type_hint** | Option<**String**> |  |  |

### Return type

[**models::IntrospectTokenResponse**](IntrospectTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_request_token

> models::TokenResponse oauth2_request_token(grant_type, client_id, client_secret, code, redirect_uri, refresh_token, session_token, sub_type, sub, scope)
Request Token

Request an access token using a valid grant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**code** | Option<**String**> |  |  |
**redirect_uri** | Option<**String**> |  |  |
**refresh_token** | Option<**String**> |  |  |
**session_token** | Option<**String**> |  |  |
**sub_type** | Option<**String**> |  |  |[default to user]
**sub** | Option<**String**> |  |  |
**scope** | Option<**String**> |  |  |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_revoke_token

> serde_json::Value oauth2_revoke_token(token, client_id, client_secret, token_type_hint)
Revoke Token

Revoke an access token or a refresh token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**client_id** | **String** |  | [required] |
**client_secret** | **String** |  | [required] |
**token_type_hint** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_userinfo

> models::ResponseOauth2Userinfo oauth2_userinfo()
Get User Info

Get information about the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ResponseOauth2Userinfo**](Response_Oauth2_Userinfo.md)

### Authorization

[oidc](../README.md#oidc)

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


## organizations_create

> models::Organization organizations_create(organization_create)
Create Organization

Create an organization.  **Scopes**: `organizations:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_create** | [**OrganizationCreate**](OrganizationCreate.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[pat](../README.md#pat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_get

> models::Organization organizations_get(id)
Get Organization

Get an organization by ID.  **Scopes**: `organizations:read` `organizations:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_list

> models::ListResourceOrganization organizations_list(slug, page, limit, sorting)
List Organizations

List organizations.  **Scopes**: `organizations:read` `organizations:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | Option<**String**> | Filter by slug. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::OrganizationSortProperty>**](models::OrganizationSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceOrganization**](ListResource_Organization_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_update

> models::Organization organizations_update(id, organization_update)
Update Organization

Update an organization.  **Scopes**: `organizations:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**organization_update** | [**OrganizationUpdate**](OrganizationUpdate.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

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


## refunds_create

> models::Refund refunds_create(refund_create)
Create Refund

Create a refund.  **Scopes**: `refunds:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refund_create** | [**RefundCreate**](RefundCreate.md) |  | [required] |

### Return type

[**models::Refund**](Refund.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refunds_list

> models::ListResourceRefund refunds_list(id, organization_id, order_id, subscription_id, customer_id, succeeded, page, limit, sorting)
List Refunds

List products.  **Scopes**: `refunds:read` `refunds:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**RefundIdFilter**](.md)> | Filter by refund ID. |  |
**organization_id** | Option<[**OrganizationIdFilter**](.md)> | Filter by organization ID. |  |
**order_id** | Option<[**OrderIdFilter**](.md)> | Filter by order ID. |  |
**subscription_id** | Option<[**SubscriptionIdFilter**](.md)> | Filter by subscription ID. |  |
**customer_id** | Option<[**CustomerIdFilter**](.md)> | Filter by customer ID. |  |
**succeeded** | Option<**bool**> | Filter by `succeeded`. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]
**sorting** | Option<[**Vec<models::RefundSortProperty>**](models::RefundSortProperty.md)> | Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign `-` before the criteria name to sort by descending order. |  |

### Return type

[**models::ListResourceRefund**](ListResource_Refund_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
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


## webhooks_create_webhook_endpoint

> models::WebhookEndpoint webhooks_create_webhook_endpoint(webhook_endpoint_create)
Create Webhook Endpoint

Create a webhook endpoint.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_endpoint_create** | [**WebhookEndpointCreate**](WebhookEndpointCreate.md) |  | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_delete_webhook_endpoint

> webhooks_delete_webhook_endpoint(id)
Delete Webhook Endpoint

Delete a webhook endpoint.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook endpoint ID. | [required] |

### Return type

 (empty response body)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_get_webhook_endpoint

> models::WebhookEndpoint webhooks_get_webhook_endpoint(id)
Get Webhook Endpoint

Get a webhook endpoint by ID.  **Scopes**: `webhooks:read` `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook endpoint ID. | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_list_webhook_deliveries

> models::ListResourceWebhookDelivery webhooks_list_webhook_deliveries(endpoint_id, start_timestamp, end_timestamp, page, limit)
List Webhook Deliveries

List webhook deliveries.  Deliveries are all the attempts to deliver a webhook event to an endpoint.  **Scopes**: `webhooks:read` `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | Option<[**EndpointId**](.md)> | Filter by webhook endpoint ID. |  |
**start_timestamp** | Option<**String**> | Filter deliveries after this timestamp. |  |
**end_timestamp** | Option<**String**> | Filter deliveries before this timestamp. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceWebhookDelivery**](ListResource_WebhookDelivery_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_list_webhook_endpoints

> models::ListResourceWebhookEndpoint webhooks_list_webhook_endpoints(organization_id, page, limit)
List Webhook Endpoints

List webhook endpoints.  **Scopes**: `webhooks:read` `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<[**OrganizationId**](.md)> | Filter by organization ID. |  |
**page** | Option<**i32**> | Page number, defaults to 1. |  |[default to 1]
**limit** | Option<**i32**> | Size of a page, defaults to 10. Maximum is 100. |  |[default to 10]

### Return type

[**models::ListResourceWebhookEndpoint**](ListResource_WebhookEndpoint_.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_redeliver_webhook_event

> serde_json::Value webhooks_redeliver_webhook_event(id)
Redeliver Webhook Event

Schedule the re-delivery of a webhook event.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook event ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_reset_webhook_endpoint_secret

> models::WebhookEndpoint webhooks_reset_webhook_endpoint_secret(id)
Reset Webhook Endpoint Secret

Regenerate a webhook endpoint secret.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook endpoint ID. | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_update_webhook_endpoint

> models::WebhookEndpoint webhooks_update_webhook_endpoint(id, webhook_endpoint_update)
Update Webhook Endpoint

Update a webhook endpoint.  **Scopes**: `webhooks:write`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook endpoint ID. | [required] |
**webhook_endpoint_update** | [**WebhookEndpointUpdate**](WebhookEndpointUpdate.md) |  | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[pat](../README.md#pat), [oat](../README.md#oat), [oidc](../README.md#oidc)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

