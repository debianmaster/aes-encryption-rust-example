use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{thread_rng, Rng};
use hex::{encode, decode};
use serde::{Deserialize, Serialize};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Serialize, Deserialize)]
struct EncryptRequest {
    data: String,
}

#[derive(Serialize, Deserialize)]
struct DecryptRequest {
    encrypted_data: String,
    key: String, // Add this line
    iv: String,  // Add this line
}

#[derive(Serialize, Deserialize)]
struct EncryptResponse {
    encrypted_data: String,
    key: String,
    iv: String,
}

#[derive(Serialize, Deserialize)]
struct DecryptResponse {
    decrypted_data: String,
}

async fn encrypt(data: web::Json<EncryptRequest>) -> impl Responder {
    let key = thread_rng().gen::<[u8; 32]>();
    let iv = thread_rng().gen::<[u8; 16]>();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    let encrypted_data = cipher.encrypt_vec(data.data.as_bytes());
    HttpResponse::Ok().json(EncryptResponse {
        encrypted_data: encode(&encrypted_data),
        key: encode(&key),
        iv: encode(&iv),
    })
}

async fn decrypt(data: web::Json<DecryptRequest>) -> impl Responder {
    let DecryptRequest { encrypted_data, key, iv } = data.into_inner();
    let key = decode(&key).unwrap();
    let iv = decode(&iv).unwrap();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    let decrypted_data = cipher.decrypt_vec(&decode(&encrypted_data).unwrap()).unwrap();
    HttpResponse::Ok().json(DecryptResponse {
        decrypted_data: String::from_utf8_lossy(&decrypted_data).to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/encrypt", web::post().to(encrypt))
            .route("/decrypt", web::post().to(decrypt))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
