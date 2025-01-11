#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::fdx_notifications::FdxNotificationsRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let category = FdxNotificationCategory::Security;
    let notification_id = "your notification id";
    let notification_payload = FdxNotificationPayload {
        custom_fields: Some(
            vec![
                FdxFiAttribute { name : "your name".to_owned(), value : "your value"
                .to_owned() }
            ],
        ),
        id: Some("your id".to_owned()),
        id_type: Some(FdxNotificationPayloadIdType::Account),
    };
    let sent_on = chrono::Utc::now();
    let type_ = FdxNotificationType::ConsentRevoked;
    let response = client
        .fdx_notifications(FdxNotificationsRequired {
            category,
            notification_id,
            notification_payload,
            sent_on,
            type_,
        })
        .priority(FdxNotificationPriority::High)
        .publisher(FdxParty {
            home_uri: Some("your home uri".to_owned()),
            logo_uri: Some("your logo uri".to_owned()),
            name: "your name".to_owned(),
            registered_entity_id: Some("your registered entity id".to_owned()),
            registered_entity_name: Some("your registered entity name".to_owned()),
            registry: Some(FdxPartyRegistry::Fdx),
            type_: FdxPartyType::DataAccessPlatform,
        })
        .severity(FdxNotificationSeverity::Emergency)
        .subscriber(FdxParty {
            home_uri: Some("your home uri".to_owned()),
            logo_uri: Some("your logo uri".to_owned()),
            name: "your name".to_owned(),
            registered_entity_id: Some("your registered entity id".to_owned()),
            registered_entity_name: Some("your registered entity name".to_owned()),
            registry: Some(FdxPartyRegistry::Fdx),
            type_: FdxPartyType::DataAccessPlatform,
        })
        .url(FdxHateoasLink {
            action: Some(FdxHateoasLinkAction::Get),
            href: "your href".to_owned(),
            rel: Some("your rel".to_owned()),
            types: Some(vec![FdxContentTypes::ApplicationPdf]),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
