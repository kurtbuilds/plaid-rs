use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    FdxNotificationCategory, FdxNotificationPayload, FdxNotificationPriority, FdxParty,
    FdxNotificationSeverity, FdxNotificationType, FdxHateoasLink,
};
/**You should use this struct via [`PlaidClient::fdx_notifications`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdxNotificationsRequest {
    pub category: FdxNotificationCategory,
    pub notification_id: String,
    pub notification_payload: FdxNotificationPayload,
    pub priority: Option<FdxNotificationPriority>,
    pub publisher: Option<FdxParty>,
    pub sent_on: chrono::DateTime<chrono::Utc>,
    pub severity: Option<FdxNotificationSeverity>,
    pub subscriber: Option<FdxParty>,
    pub type_: FdxNotificationType,
    pub url: Option<FdxHateoasLink>,
}
pub struct FdxNotificationsRequired<'a> {
    pub category: FdxNotificationCategory,
    pub notification_id: &'a str,
    pub notification_payload: FdxNotificationPayload,
    pub sent_on: chrono::DateTime<chrono::Utc>,
    pub type_: FdxNotificationType,
}
impl FluentRequest<'_, FdxNotificationsRequest> {
    ///Set the value of the priority field.
    pub fn priority(mut self, priority: FdxNotificationPriority) -> Self {
        self.params.priority = Some(priority);
        self
    }
    ///Set the value of the publisher field.
    pub fn publisher(mut self, publisher: FdxParty) -> Self {
        self.params.publisher = Some(publisher);
        self
    }
    ///Set the value of the severity field.
    pub fn severity(mut self, severity: FdxNotificationSeverity) -> Self {
        self.params.severity = Some(severity);
        self
    }
    ///Set the value of the subscriber field.
    pub fn subscriber(mut self, subscriber: FdxParty) -> Self {
        self.params.subscriber = Some(subscriber);
        self
    }
    ///Set the value of the url field.
    pub fn url(mut self, url: FdxHateoasLink) -> Self {
        self.params.url = Some(url);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, FdxNotificationsRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/fdx/notifications";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "category" : self.params.category }));
            r = r
                .json(
                    serde_json::json!({ "notificationId" : self.params.notification_id }),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "notificationPayload" : self.params.notification_payload }
                    ),
                );
            if let Some(ref unwrapped) = self.params.priority {
                r = r.json(serde_json::json!({ "priority" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.publisher {
                r = r.json(serde_json::json!({ "publisher" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "sentOn" : self.params.sent_on }));
            if let Some(ref unwrapped) = self.params.severity {
                r = r.json(serde_json::json!({ "severity" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.subscriber {
                r = r.json(serde_json::json!({ "subscriber" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "type" : self.params.type_ }));
            if let Some(ref unwrapped) = self.params.url {
                r = r.json(serde_json::json!({ "url" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Webhook receiver for fdx notifications

A generic webhook receiver endpoint for FDX Event Notifications

See endpoint docs at <https://plaid.com/docs/api/fdx/notifications/#post>.*/
    pub fn fdx_notifications(
        &self,
        args: FdxNotificationsRequired,
    ) -> FluentRequest<'_, FdxNotificationsRequest> {
        FluentRequest {
            client: self,
            params: FdxNotificationsRequest {
                category: args.category,
                notification_id: args.notification_id.to_owned(),
                notification_payload: args.notification_payload,
                priority: None,
                publisher: None,
                sent_on: args.sent_on,
                severity: None,
                subscriber: None,
                type_: args.type_,
                url: None,
            },
        }
    }
}
