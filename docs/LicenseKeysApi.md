# \LicenseKeysApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_portal_license_keys_activate**](LicenseKeysApi.md#customer_portal_license_keys_activate) | **POST** /v1/customer-portal/license-keys/activate | Activate License Key
[**customer_portal_license_keys_deactivate**](LicenseKeysApi.md#customer_portal_license_keys_deactivate) | **POST** /v1/customer-portal/license-keys/deactivate | Deactivate License Key
[**customer_portal_license_keys_get**](LicenseKeysApi.md#customer_portal_license_keys_get) | **GET** /v1/customer-portal/license-keys/{id} | Get License Key
[**customer_portal_license_keys_list**](LicenseKeysApi.md#customer_portal_license_keys_list) | **GET** /v1/customer-portal/license-keys/ | List License Keys
[**customer_portal_license_keys_validate**](LicenseKeysApi.md#customer_portal_license_keys_validate) | **POST** /v1/customer-portal/license-keys/validate | Validate License Key
[**license_keys_activate**](LicenseKeysApi.md#license_keys_activate) | **POST** /v1/license-keys/activate | Activate License Key
[**license_keys_deactivate**](LicenseKeysApi.md#license_keys_deactivate) | **POST** /v1/license-keys/deactivate | Deactivate License Key
[**license_keys_get**](LicenseKeysApi.md#license_keys_get) | **GET** /v1/license-keys/{id} | Get License Key
[**license_keys_get_activation**](LicenseKeysApi.md#license_keys_get_activation) | **GET** /v1/license-keys/{id}/activations/{activation_id} | Get Activation
[**license_keys_list**](LicenseKeysApi.md#license_keys_list) | **GET** /v1/license-keys/ | List License Keys
[**license_keys_update**](LicenseKeysApi.md#license_keys_update) | **PATCH** /v1/license-keys/{id} | Update License Key
[**license_keys_validate**](LicenseKeysApi.md#license_keys_validate) | **POST** /v1/license-keys/validate | Validate License Key



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

