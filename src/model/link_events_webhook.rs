use serde::{Serialize, Deserialize};
use super::LinkEvent;
/**This webhook contains a summary of the events from a Link session and will be fired after the user finishes going through Link. If the user abandons the Link flow (i.e., closes the hosted link webpage or leaves Link open for too long without taking any action), the webhook will be fired 5-15 minutes after the last user interaction. A single Link session may occasionally generate multiple `EVENTS` webhooks. If this occurs, the new webhook will contain all previous events for the session, as well as new events that occurred since the previous `EVENTS` webhook was sent. If this occurs, events can be grouped using the `link_session_id` field and, if necessary, de-duplicated using the `event_id` field.

By default, the `EVENTS` webhook is sent only for sessions where the end user goes through a Hosted Link flow (including Link Recovery flows). If you would like to receive this webhook for sessions not using Hosted Link, contact your Account Manager or Support.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkEventsWebhook {
    ///The Link events emitted during the Link session
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<LinkEvent>,
    ///An identifier for the Link session these events occurred in
    pub link_session_id: String,
    ///The Link token used to create the Link session these events are from
    pub link_token: String,
    ///`EVENTS`
    pub webhook_code: String,
    ///`LINK`
    pub webhook_type: String,
}
impl std::fmt::Display for LinkEventsWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
