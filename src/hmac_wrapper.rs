use hmac::{digest::generic_array::GenericArray, Hmac, Mac, NewMac};
use sha2::Sha256;

use crate::ShopifyApp;

// Types
type HmacSha256 = Hmac<Sha256>;

impl ShopifyApp {

    /// This method generates a HMAC (keyed-hash message authentication code) of the data passed as an argument as a byte array.
    ///
    /// The method takes a slice of bytes `&[u8]` as an argument and returns a type that implements the `GenericArray` trait from the `generic-array` crate, with the element type `u8` and the size specified by the `OutputSize` associated type of the `HmacSha256` type from the `hmac` crate.
    ///
    /// The method uses the `HmacSha256` type to create a new variable-length HMAC using the `HmacSha256::new_varkey` method, passing in the `secret` field of the `credentials` field of the `ShopifyApp` struct as a byte slice. The `expect` method is called on the result to unwrap the `Result` type that `new_varkey` returns, which will panic if the key is invalid.
    
    pub fn generate_hmac_bytes(
        &self,
        data: &[u8],
    ) -> GenericArray<u8, <HmacSha256 as Mac>::OutputSize> {
        let mut mac = HmacSha256::new_varkey(self.credentials.secret.as_bytes())
            .expect("Invalid store secret");
        mac.update(data);
        let result = mac.finalize();
        result.into_bytes()
    }

    pub fn generate_hmac_hex(&self, data: &[u8]) -> String {
        let hmac_bytes = self.generate_hmac_bytes(data);
        hex::encode(hmac_bytes)
    }

    pub fn generate_hmac_base64(&self, data: &[u8]) -> String {
        let hmac_bytes = self.generate_hmac_bytes(data);
        base64::encode(hmac_bytes)
    }

    pub fn valid_hmac(&self, query_params: &Vec<(String, String)>) -> bool {
        let mut hmac = String::new();

        let mut message = query_params.into_iter().fold(
            Vec::<(&str, &str)>::with_capacity(2),
            |mut acc, query_item| {
                let (key, value) = query_item;

                if "hmac".to_string() == *key {
                    hmac = value.to_string();
                    return acc;
                }

                acc.push((key, value));
                acc
            },
        );

        if message.is_empty() {
            return false;
        }

        message.sort_by(|a, b| a.0.cmp(&b.0));

        let message_str = &message.iter().fold(String::new(), |acc, (key, val)| {
            acc + "&" + &key + "=" + &val
        })[1..];

        let encoded = self.generate_hmac_hex(message_str.as_bytes());

        encoded == hmac
    }
}
