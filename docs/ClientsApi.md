# \ClientsApi

All URIs are relative to *https://api.polar.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth2_clients_oauth2_create_client**](ClientsApi.md#oauth2_clients_oauth2_create_client) | **POST** /v1/oauth2/register | Create Client
[**oauth2_clients_oauth2_delete_client**](ClientsApi.md#oauth2_clients_oauth2_delete_client) | **DELETE** /v1/oauth2/register/{client_id} | Delete Client
[**oauth2_clients_oauth2_get_client**](ClientsApi.md#oauth2_clients_oauth2_get_client) | **GET** /v1/oauth2/register/{client_id} | Get Client
[**oauth2_clients_oauth2_update_client**](ClientsApi.md#oauth2_clients_oauth2_update_client) | **PUT** /v1/oauth2/register/{client_id} | Update Client



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

