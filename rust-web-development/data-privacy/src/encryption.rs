use aes_gcm::{
    Aes256Gcm, Error as AesGcmError, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Encryption Fail")]
    Encrypt(AesGcmError),
}

pub fn encrypt_data(data: &str, key: &[u8; 32]) -> Result<Vec<u8>, EncryptionError> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes())
        .map_err(EncryptionError::Encrypt)?;
    Ok([nonce.to_vec(), ciphertext].concat())
}

pub fn decrypt_data(encrypted_data: &[u8], key: &[u8; 32]) -> String {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let (nonce, ciphertext) = encrypted_data.split_at(12); // Extract nonce
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .unwrap();
    String::from_utf8(plaintext).unwrap()
}
