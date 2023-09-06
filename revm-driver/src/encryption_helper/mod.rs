pub mod kms_util;

pub use self::kms_util::get_kms_client;
use mbedtls::cipher::raw::{CipherId, CipherMode};
use mbedtls::cipher::{Authenticated, Cipher, Encryption, Fresh};

use aws_sdk_kms::{primitives::Blob, types::DataKeySpec, Client};
pub struct EncryptionHelper {
    client: Client,
    key_id: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct EncryptionPayload {
    encrypted_data: Vec<u8>,
    decryption_key: Vec<u8>,
    ad: Vec<u8>,
    iv: Vec<u8>,
}

impl EncryptionHelper {
    // TODO: even the key should be created programmatically with the key policy permissions for a given enclave
    // creating it outside and passing the id in for now
    pub fn new(client: Client, key_id: String) -> Self {
        Self { client, key_id }
    }
    async fn generate_encryption_key(&self) -> (Blob, Blob) {
        let resp = self
            .client
            .generate_data_key()
            .key_id(&self.key_id)
            .key_spec(DataKeySpec::Aes256)
            .send()
            .await
            .unwrap();
        // resp should have the plaintext data key and the encrypted data key used to decrypt;
        // note that this code is running outside the enclave, so we'd want to drop this plaintext key from memory asap after encryption.
        // the nitro vm that hosts should be safe, but this is a possible vulnerable spot.
        // Ideally this code (data generation in the callee + encryption key gen + encryption) also runs in its own enclave/secured env
        let encryption_key = resp.plaintext.unwrap();
        println!("plaintext key is: {:#?}", encryption_key);
        // decryption key (encrypted data key) needs to be sent into the enclave for it to decrypt data
        let decryption_key = resp.ciphertext_blob.unwrap();
        (encryption_key, decryption_key)
    }

    pub async fn encrypt_data(&self, data: &[u8]) -> EncryptionPayload {
        let (encryption_key, decryption_key) = self.generate_encryption_key().await;
        let plaintext_key = encryption_key.into_inner();

        let mut ciphertext_out: Vec<u8> = vec![0; data.len() + 4];
        // symmetric default is AES256_GCM for KMS:
        // https://docs.aws.amazon.com/kms/latest/developerguide/asymmetric-key-specs.html#key-spec-symmetric-default
        let cipher: Cipher<Encryption, Authenticated, Fresh> = Cipher::new(
            CipherId::Aes,
            CipherMode::GCM,
            (plaintext_key.len() * 8) as _,
        )
        .unwrap();

        // iv length check for GCM: https://github.com/Mbed-TLS/mbedtls/blob/development/library/gcm.c#L271
        // 0 < iv.len() < 2^61 bytes
        // init our nonce to 0, shouldn't need to be rand(?) - but in actual code would need to be incremented.
        let iv = [0x00, 0x00, 0x00, 0x00];
        // random 3 bytes of auth ad
        let ad = [0x03, 0x04, 0x05];
        let cipher = cipher.set_key_iv(&plaintext_key, &iv).unwrap();
        cipher
            .encrypt_auth(&ad, &data, &mut ciphertext_out, 4)
            .unwrap();
        EncryptionPayload {
            encrypted_data: ciphertext_out,
            decryption_key: decryption_key.into_inner(),
            ad: ad.to_vec(),
            iv: iv.to_vec(),
        }
    }
}
