#[cfg(all(test, any(feature = "tls", feature = "native-tls")))]
use std::io::Read;

#[cfg(any(feature = "tls", feature = "native-tls"))]
#[test]
fn tls_connection_close() {
    let agent = ureq::Agent::default().build();
    let resp = agent
        .get("https://example.com/404")
        .set("Connection", "close")
        .call();
    assert_eq!(resp.status(), 404);
    resp.into_reader().read_to_end(&mut vec![]).unwrap();
}

#[cfg(feature = "tls")]
#[cfg(feature = "cookies")]
#[cfg(feature = "json")]
#[test]
fn agent_set_cookie() {
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize)]
    struct HttpBin {
        headers: HashMap<String, String>,
    }

    let agent = ureq::Agent::default().build();
    let cookie = ureq::Cookie::build("name", "value")
        .domain("httpbin.org")
        .secure(true)
        .finish();
    agent.set_cookie(cookie);
    let resp = agent
        .get("https://httpbin.org/get")
        .set("Connection", "close")
        .call();
    assert_eq!(resp.status(), 200);
    assert_eq!(
        "name=value",
        resp.into_json_deserialize::<HttpBin>()
            .unwrap()
            .headers
            .get("Cookie")
            .unwrap()
    );
}

#[cfg(feature = "tls")]
const BADSSL_CLIENT_CERT_PEM: &'static str = r#"Bag Attributes
    localKeyID: 41 C3 6C 33 C7 E3 36 DD EA 4A 1F C0 B7 23 B8 E6 9C DC D8 0F
subject=C = US, ST = California, L = San Francisco, O = BadSSL, CN = BadSSL Client Certificate

issuer=C = US, ST = California, L = San Francisco, O = BadSSL, CN = BadSSL Client Root Certificate Authority

-----BEGIN CERTIFICATE-----
MIIEqDCCApCgAwIBAgIUK5Ns4y2CzosB/ZoFlaxjZqoBTIIwDQYJKoZIhvcNAQEL
BQAwfjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFjAUBgNVBAcM
DVNhbiBGcmFuY2lzY28xDzANBgNVBAoMBkJhZFNTTDExMC8GA1UEAwwoQmFkU1NM
IENsaWVudCBSb290IENlcnRpZmljYXRlIEF1dGhvcml0eTAeFw0xOTExMjcwMDE5
NTdaFw0yMTExMjYwMDE5NTdaMG8xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxp
Zm9ybmlhMRYwFAYDVQQHDA1TYW4gRnJhbmNpc2NvMQ8wDQYDVQQKDAZCYWRTU0wx
IjAgBgNVBAMMGUJhZFNTTCBDbGllbnQgQ2VydGlmaWNhdGUwggEiMA0GCSqGSIb3
DQEBAQUAA4IBDwAwggEKAoIBAQDHN18R6x5Oz+u6SOXLoxIscz5GHR6cDcCLgyPa
x2XfXHdJs+h6fTy61WGM+aXEhR2SIwbj5997s34m0MsbvkJrFmn0LHK1fuTLCihE
EmxGdCGZA9xrwxFYAkEjP7D8v7cAWRMipYF/JP7VU7xNUo+QSkZ0sOi9k6bNkABK
L3+yP6PqAzsBoKIN5lN/YRLrppsDmk6nrRDo4R3CD+8JQl9quEoOmL22Pc/qpOjL
1jgOIFSE5y3gwbzDlfCYoAL5V+by1vu0yJShTTK8oo5wvphcFfEHaQ9w5jFg2htd
q99UER3BKuNDuL+zejqGQZCWb0Xsk8S5WBuX8l3Brrg5giqNAgMBAAGjLTArMAkG
A1UdEwQCMAAwEQYJYIZIAYb4QgEBBAQDAgeAMAsGA1UdDwQEAwIF4DANBgkqhkiG
9w0BAQsFAAOCAgEAZBauLzFSOijkDadcippr9C6laHebb0oRS54xAV70E9k5GxfR
/E2EMuQ8X+miRUMXxKquffcDsSxzo2ac0flw94hDx3B6vJIYvsQx9Lzo95Im0DdT
DkHFXhTlv2kjQwFVnEsWYwyGpHMTjanvNkO7sBP9p1bN1qTE3QAeyMZNKWJk5xPl
U298ERar6tl3Z2Cl8mO6yLhrq4ba6iPGw08SENxzuAJW+n8r0rq7EU+bMg5spgT1
CxExzG8Bb0f98ZXMklpYFogkcuH4OUOFyRodotrotm3iRbuvZNk0Zz7N5n1oLTPl
bGPMwBcqaGXvK62NlaRkwjnbkPM4MYvREM0bbAgZD2GHyANBTso8bdWvhLvmoSjs
FSqJUJp17AZ0x/ELWZd69v2zKW9UdPmw0evyVR19elh/7dmtF6wbewc4N4jxQnTq
IItuhIWKWB9edgJz65uZ9ubQWjXoa+9CuWcV/1KxuKCbLHdZXiboLrKm4S1WmMYW
d0sJm95H9mJzcLyhLF7iX2kK6K9ug1y02YCVXBC9WGZc2x6GMS7lDkXSkJFy3EWh
CmfxkmFGwOgwKt3Jd1pF9ftcSEMhu4WcMgxi9vZr9OdkJLxmk033sVKI/hnkPaHw
g0Y2YBH5v0xmi8sYU7weOcwynkjZARpUltBUQ0pWCF5uJsEB8uE8PPDD3c4=
-----END CERTIFICATE-----
Bag Attributes
    localKeyID: 41 C3 6C 33 C7 E3 36 DD EA 4A 1F C0 B7 23 B8 E6 9C DC D8 0F
Key Attributes: <No Attributes>
-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAxzdfEeseTs/rukjly6MSLHM+Rh0enA3Ai4Mj2sdl31x3SbPo
en08utVhjPmlxIUdkiMG4+ffe7N+JtDLG75CaxZp9CxytX7kywooRBJsRnQhmQPc
a8MRWAJBIz+w/L+3AFkTIqWBfyT+1VO8TVKPkEpGdLDovZOmzZAASi9/sj+j6gM7
AaCiDeZTf2ES66abA5pOp60Q6OEdwg/vCUJfarhKDpi9tj3P6qToy9Y4DiBUhOct
4MG8w5XwmKAC+Vfm8tb7tMiUoU0yvKKOcL6YXBXxB2kPcOYxYNobXavfVBEdwSrj
Q7i/s3o6hkGQlm9F7JPEuVgbl/Jdwa64OYIqjQIDAQABAoIBAFUQf7fW/YoJnk5c
8kKRzyDL1Lt7k6Zu+NiZlqXEnutRQF5oQ8yJzXS5yH25296eOJI+AqMuT28ypZtN
bGzcQOAZIgTxNcnp9Sf9nlPyyekLjY0Y6PXaxX0e+VFj0N8bvbiYUGNq6HCyC15r
8uvRZRvnm04YfEj20zLTWkxTG+OwJ6ZNha1vfq8z7MG5JTsZbP0g7e/LrEb3wI7J
Zu9yHQUzq23HhfhpmLN/0l89YLtOaS8WNq4QvKYgZapw/0G1wWoWW4Y2/UpAxZ9r
cqTBWSpCSCCgyWjiNhPbSJWfe/9J2bcanITLcvCLlPWGAHy1wpo9iBH57y7S+7YS
3yi7lgECgYEA8lwaRIChc38tmtQCNPtai/7uVDdeJe0uv8Jsg04FTF8KMYcD0V1g
+T7rUPA+rTHwv8uAGLdzl4NW5Qryw18rDY+UivnaZkEdEsnlo3fc8MSQF78dDHCX
nwmHfOmBnBoSbLl+W5ByHkJRHOnX+8qKq9ePNFUMf/hZNYuma9BCFBUCgYEA0m2p
VDn12YdhFUUBIH91aD5cQIsBhkHFU4vqW4zBt6TsJpFciWbrBrTeRzeDou59aIsn
zGBrLMykOY+EwwRku9KTVM4U791Z/NFbH89GqyUaicb4or+BXw5rGF8DmzSsDo0f
ixJ9TVD5DmDi3c9ZQ7ljrtdSxPdA8kOoYPFsApkCgYEA08uZSPQAI6aoe/16UEK4
Rk9qhz47kHlNuVZ27ehoyOzlQ5Lxyy0HacmKaxkILOLPuUxljTQEWAv3DAIdVI7+
WMN41Fq0eVe9yIWXoNtGwUGFirsA77YVSm5RcN++3GQMZedUfUAl+juKFvJkRS4j
MTkXdGw+mDa3/wsjTGSa2mECgYABO6NCWxSVsbVf6oeXKSgG9FaWCjp4DuqZErjM
0IZSDSVVFIT2SSQXZffncuvSiJMziZ0yFV6LZKeRrsWYXu44K4Oxe4Oj5Cgi0xc1
mIFRf2YoaIIMchLP+8Wk3ummfyiC7VDB/9m8Gj1bWDX8FrrvKqbq31gcz1YSFVNn
PgLkAQKBgFzG8NdL8os55YcjBcOZMUs5QTKiQSyZM0Abab17k9JaqsU0jQtzeFsY
FTiwh2uh6l4gdO/dGC/P0Vrp7F05NnO7oE4T+ojDzVQMnFpCBeL7x08GfUQkphEG
m0Wqhhi8/24Sy934t5Txgkfoltg8ahkx934WjP6WWRnSAu+cf+vW
-----END RSA PRIVATE KEY-----
"#;

#[cfg(feature = "tls")]
#[test]
fn tls_client_certificate() {
    let agent = ureq::Agent::default().build();

    let mut tls_config = rustls::ClientConfig::new();

    let certs =
        rustls::internal::pemfile::certs(&mut BADSSL_CLIENT_CERT_PEM.clone().as_bytes()).unwrap();
    let key =
        rustls::internal::pemfile::rsa_private_keys(&mut BADSSL_CLIENT_CERT_PEM.clone().as_bytes())
            .unwrap()[0]
            .clone();

    tls_config.set_single_client_cert(certs, key).unwrap();
    tls_config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let resp = agent
        .get("https://client.badssl.com/")
        .set_tls_config(std::sync::Arc::new(tls_config))
        .call();

    assert_eq!(resp.status(), 200);
}
