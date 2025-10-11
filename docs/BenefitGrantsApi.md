# \BenefitGrantsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**benefit_grants_list**](BenefitGrantsApi.md#benefit_grants_list) | **GET** /v1/benefit-grants/ | List Benefit Grants
[**customer_portal_benefit_grants_get**](BenefitGrantsApi.md#customer_portal_benefit_grants_get) | **GET** /v1/customer-portal/benefit-grants/{id} | Get Benefit Grant
[**customer_portal_benefit_grants_list**](BenefitGrantsApi.md#customer_portal_benefit_grants_list) | **GET** /v1/customer-portal/benefit-grants/ | List Benefit Grants
[**customer_portal_benefit_grants_update**](BenefitGrantsApi.md#customer_portal_benefit_grants_update) | **PATCH** /v1/customer-portal/benefit-grants/{id} | Update Benefit Grant



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

