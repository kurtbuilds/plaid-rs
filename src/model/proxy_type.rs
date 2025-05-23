use serde::{Serialize, Deserialize};
/**An enum indicating whether a network proxy is present and if so what type it is.

`none_detected` indicates the user is not on a detectable proxy network.

`tor` indicates the user was using a Tor browser, which sends encrypted traffic on a decentralized network and is somewhat similar to a VPN (Virtual Private Network).

`vpn` indicates the user is on a VPN (Virtual Private Network)

`web_proxy` indicates the user is on a web proxy server, which may allow them to conceal information such as their IP address or other identifying information.

`public_proxy` indicates the user is on a public web proxy server, which is similar to a web proxy but can be shared by multiple users. This may allow multiple users to appear as if they have the same IP address for instance.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProxyType {
    #[serde(rename = "none_detected")]
    NoneDetected,
    #[serde(rename = "tor")]
    Tor,
    #[serde(rename = "vpn")]
    Vpn,
    #[serde(rename = "web_proxy")]
    WebProxy,
    #[serde(rename = "public_proxy")]
    PublicProxy,
}
