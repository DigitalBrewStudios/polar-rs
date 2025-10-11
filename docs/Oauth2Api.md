# \Oauth2Api

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth2_authorize**](Oauth2Api.md#oauth2_authorize) | **GET** /v1/oauth2/authorize | Authorize
[**oauth2_clients_oauth2_create_client**](Oauth2Api.md#oauth2_clients_oauth2_create_client) | **POST** /v1/oauth2/register | Create Client
[**oauth2_clients_oauth2_delete_client**](Oauth2Api.md#oauth2_clients_oauth2_delete_client) | **DELETE** /v1/oauth2/register/{client_id} | Delete Client
[**oauth2_clients_oauth2_get_client**](Oauth2Api.md#oauth2_clients_oauth2_get_client) | **GET** /v1/oauth2/register/{client_id} | Get Client
[**oauth2_clients_oauth2_update_client**](Oauth2Api.md#oauth2_clients_oauth2_update_client) | **PUT** /v1/oauth2/register/{client_id} | Update Client
[**oauth2_introspect_token**](Oauth2Api.md#oauth2_introspect_token) | **POST** /v1/oauth2/introspect | Introspect Token
[**oauth2_request_token**](Oauth2Api.md#oauth2_request_token) | **POST** /v1/oauth2/token | Request Token
[**oauth2_revoke_token**](Oauth2Api.md#oauth2_revoke_token) | **POST** /v1/oauth2/revoke | Revoke Token
[**oauth2_userinfo**](Oauth2Api.md#oauth2_userinfo) | **GET** /v1/oauth2/userinfo | Get User Info



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

