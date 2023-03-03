use serde::Deserialize as _;
use std::collections::{HashMap, HashSet};
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct Credential {
    secret: (String, String),
    url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum CredentialValue {
    Tuple(String, String),
    String(String),
}

impl Credential {
    #[allow(dead_code)]
    fn keys() -> HashSet<String> {
        serde_json::to_value(Credential::default())
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .map(|x| x.to_owned())
            .collect::<HashSet<_>>()
    }

    #[allow(dead_code)]
    pub fn from_keepass_db(db: keepass::Database) -> Self {
        let keys = Self::keys();
        let mut map = HashMap::new();
        for node in &db.root {
            match node {
                keepass::NodeRef::Entry(e) => {
                    let title = e.get_title().unwrap();
                    if keys.contains(title) {
                        let user_name = e.get_username().unwrap();
                        let password = e.get_password();
                        let pat = if let Some(password) = password {
                            serde_json::json!([user_name, password])
                        } else {
                            serde_json::json!(user_name)
                        };
                        map.insert(title, pat);
                    }
                }
                keepass::NodeRef::Group(_) => {}
            }
        }
        let map_deser = serde::de::value::MapDeserializer::new(map.into_iter());
        Credential::deserialize(map_deser).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Credential;
    use aes_gcm::{aead::Aead, KeyInit};

    #[test]
    fn from_keepass_db_ok() {
        let db = keepass::Database::open(
            &mut include_bytes!("../../test_files/keepass/testcases.kdbx").as_slice(),
            Some(include_str!("../../test_files/keepass/master_password")),
            None,
        )
        .unwrap();
        let c = Credential::from_keepass_db(db);
        assert_eq!(c.url, "example.com");
        assert_eq!(c.secret, ("user".to_string(), "password".to_string()));
    }

    #[test]
    fn encrypt_and_decrypt_ok() {
        let db = keepass::Database::open(
            &mut include_bytes!("../../test_files/keepass/testcases.kdbx").as_slice(),
            Some(include_str!("../../test_files/keepass/master_password")),
            None,
        )
        .unwrap();
        let c = Credential::from_keepass_db(db);
        let json = serde_json::to_string(&c).unwrap();
        let crypt_password = include_bytes!("../../test_files/keepass/crypt_password");
        let encrypter = aes_gcm::Aes256Gcm::new_from_slice(crypt_password).unwrap();
        let nonce = aes_gcm::Nonce::from_slice(b"unique nonce");
        let encrypted = encrypter.encrypt(nonce, json.as_bytes()).unwrap();
        let decrypter = aes_gcm::Aes256Gcm::new_from_slice(crypt_password).unwrap();
        let decrypted = decrypter.decrypt(nonce, encrypted.as_ref()).unwrap();
        let c = serde_json::from_str::<Credential>(&String::from_utf8(decrypted).unwrap()).unwrap();
        assert_eq!(c.url, "example.com");
        assert_eq!(c.secret, ("user".to_string(), "password".to_string()));
    }
}
