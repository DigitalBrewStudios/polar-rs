# OAuth2ClientConfigurationUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**redirect_uris** | **Vec<String>** |  | 
**token_endpoint_auth_method** | Option<**String**> |  | [optional][default to ClientSecretPost]
**grant_types** | Option<**Vec<String>**> |  | [optional][default to [authorization_code, refresh_token]]
**response_types** | Option<**Vec<String>**> |  | [optional][default to [code]]
**scope** | Option<**String**> |  | [optional][default to openid profile email user:read organizations:read organizations:write custom_fields:read custom_fields:write discounts:read discounts:write checkout_links:read checkout_links:write checkouts:read checkouts:write transactions:read transactions:write payouts:read payouts:write products:read products:write benefits:read benefits:write events:read events:write meters:read meters:write files:read files:write subscriptions:read subscriptions:write customers:read customers:write wallets:read wallets:write customer_meters:read customer_sessions:write customer_seats:read customer_seats:write orders:read orders:write refunds:read refunds:write payments:read metrics:read webhooks:read webhooks:write external_organizations:read license_keys:read license_keys:write repositories:read repositories:write issues:read issues:write customer_portal:read customer_portal:write notifications:read notifications:write notification_recipients:read notification_recipients:write]
**client_name** | **String** |  | 
**client_uri** | Option<**String**> |  | [optional]
**logo_uri** | Option<**String**> |  | [optional]
**tos_uri** | Option<**String**> |  | [optional]
**policy_uri** | Option<**String**> |  | [optional]
**default_sub_type** | Option<[**models::SubType**](SubType.md)> |  | [optional][default to Organization]
**client_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


