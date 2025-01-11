use serde::{Serialize, Deserialize};
///An enum indicating the type of a linked service. Note that `adult_sites` refers' to explicit video content, and includes a number of related services.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskCheckLinkedService {
    #[serde(rename = "aboutme")]
    Aboutme,
    #[serde(rename = "adobe")]
    Adobe,
    #[serde(rename = "adult_sites")]
    AdultSites,
    #[serde(rename = "airbnb")]
    Airbnb,
    #[serde(rename = "altbalaji")]
    Altbalaji,
    #[serde(rename = "amazon")]
    Amazon,
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "archiveorg")]
    Archiveorg,
    #[serde(rename = "atlassian")]
    Atlassian,
    #[serde(rename = "bitmoji")]
    Bitmoji,
    #[serde(rename = "bodybuilding")]
    Bodybuilding,
    #[serde(rename = "booking")]
    Booking,
    #[serde(rename = "bukalapak")]
    Bukalapak,
    #[serde(rename = "codecademy")]
    Codecademy,
    #[serde(rename = "deliveroo")]
    Deliveroo,
    #[serde(rename = "diigo")]
    Diigo,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "disneyplus")]
    Disneyplus,
    #[serde(rename = "duolingo")]
    Duolingo,
    #[serde(rename = "ebay")]
    Ebay,
    #[serde(rename = "envato")]
    Envato,
    #[serde(rename = "eventbrite")]
    Eventbrite,
    #[serde(rename = "evernote")]
    Evernote,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "firefox")]
    Firefox,
    #[serde(rename = "flickr")]
    Flickr,
    #[serde(rename = "flipkart")]
    Flipkart,
    #[serde(rename = "foursquare")]
    Foursquare,
    #[serde(rename = "freelancer")]
    Freelancer,
    #[serde(rename = "gaana")]
    Gaana,
    #[serde(rename = "giphy")]
    Giphy,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "gravatar")]
    Gravatar,
    #[serde(rename = "hubspot")]
    Hubspot,
    #[serde(rename = "imgur")]
    Imgur,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "jdid")]
    Jdid,
    #[serde(rename = "kakao")]
    Kakao,
    #[serde(rename = "kommo")]
    Kommo,
    #[serde(rename = "komoot")]
    Komoot,
    #[serde(rename = "lastfm")]
    Lastfm,
    #[serde(rename = "lazada")]
    Lazada,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "linkedin")]
    Linkedin,
    #[serde(rename = "mailru")]
    Mailru,
    #[serde(rename = "microsoft")]
    Microsoft,
    #[serde(rename = "myspace")]
    Myspace,
    #[serde(rename = "netflix")]
    Netflix,
    #[serde(rename = "nike")]
    Nike,
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "patreon")]
    Patreon,
    #[serde(rename = "pinterest")]
    Pinterest,
    #[serde(rename = "plurk")]
    Plurk,
    #[serde(rename = "quora")]
    Quora,
    #[serde(rename = "qzone")]
    Qzone,
    #[serde(rename = "rambler")]
    Rambler,
    #[serde(rename = "rappi")]
    Rappi,
    #[serde(rename = "replit")]
    Replit,
    #[serde(rename = "samsung")]
    Samsung,
    #[serde(rename = "seoclerks")]
    Seoclerks,
    #[serde(rename = "shopclues")]
    Shopclues,
    #[serde(rename = "skype")]
    Skype,
    #[serde(rename = "snapchat")]
    Snapchat,
    #[serde(rename = "snapdeal")]
    Snapdeal,
    #[serde(rename = "soundcloud")]
    Soundcloud,
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "starz")]
    Starz,
    #[serde(rename = "strava")]
    Strava,
    #[serde(rename = "taringa")]
    Taringa,
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "tiki")]
    Tiki,
    #[serde(rename = "tokopedia")]
    Tokopedia,
    #[serde(rename = "treehouse")]
    Treehouse,
    #[serde(rename = "tumblr")]
    Tumblr,
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "venmo")]
    Venmo,
    #[serde(rename = "viber")]
    Viber,
    #[serde(rename = "vimeo")]
    Vimeo,
    #[serde(rename = "vivino")]
    Vivino,
    #[serde(rename = "vkontakte")]
    Vkontakte,
    #[serde(rename = "wattpad")]
    Wattpad,
    #[serde(rename = "weibo")]
    Weibo,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "wordpress")]
    Wordpress,
    #[serde(rename = "xing")]
    Xing,
    #[serde(rename = "yahoo")]
    Yahoo,
    #[serde(rename = "yandex")]
    Yandex,
    #[serde(rename = "zalo")]
    Zalo,
    #[serde(rename = "zoho")]
    Zoho,
}
