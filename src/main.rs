use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce
};
use secrecy::{Secret, ExposeSecret};
use rand::RngCore;

fn main() {
    // 1. Simulação de um segredo (em um app real, viria de um input)
    let password = Secret::new("senha-muito-segura".to_string());
    
    // 2. Gerar uma chave aleatória de 32 bytes (AES-256)
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).expect("Erro ao criar cifra");

    // 3. Gerar um Nonce (Número usado uma única vez para evitar ataques de repetição)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 4. Criptografar os dados
    let message = "Dados sensíveis do meu cofre".as_bytes();
    let ciphertext = cipher.encrypt(nonce, message)
        .expect("Erro na criptografia");

    println!("Criptografia concluída com sucesso!");
    println!("Tamanho do payload: {} bytes", ciphertext.len());
    
    // O Rust limpa a memória automaticamente ao sair do escopo
}
