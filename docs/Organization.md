# Organization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Creation timestamp of the object. | 
**modified_at** | Option<**String**> |  | 
**id** | **String** | The organization ID. | 
**name** | **String** | Organization name shown in checkout, customer portal, emails etc. | 
**slug** | **String** | Unique organization slug in checkout, customer portal and credit card statements. | 
**avatar_url** | Option<**String**> |  | 
**email** | Option<**String**> |  | 
**website** | Option<**String**> |  | 
**socials** | [**Vec<models::OrganizationSocialLink>**](OrganizationSocialLink.md) | Links to social profiles. | 
**status** | [**models::OrganizationStatus**](OrganizationStatus.md) | Current organization status | 
**details_submitted_at** | Option<**String**> |  | 
**feature_settings** | Option<[**models::OrganizationFeatureSettings**](OrganizationFeatureSettings.md)> |  | 
**subscription_settings** | [**models::OrganizationSubscriptionSettings**](OrganizationSubscriptionSettings.md) | Settings related to subscriptions management | 
**notification_settings** | [**models::OrganizationNotificationSettings**](OrganizationNotificationSettings.md) | Settings related to notifications | 
**customer_email_settings** | [**models::OrganizationCustomerEmailSettings**](OrganizationCustomerEmailSettings.md) | Settings related to customer emails | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


