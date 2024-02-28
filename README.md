```sh
git clone https://github.com/debianmaster/aes-encryption-rust-example.git
cd aes-encryption-rust-example.git
cargo run 
```

```sh
curl -X POST http://127.0.0.1:8080/encrypt \
-H "Content-Type: application/json" \
-d '{"data":"Hello, AES-256-CBC!","key":"4050ba0bc19dc38e2587a0837db956164f754c2364519aff1f07198612d3bf3a","iv":"5fb160e67acd8772939ba5aee58fba2a"}'

{"encrypted_data":"118e7ab1869c9d3d94199202ffc78c2760813a2c9f7b1f0024c5073c9a6a9ac3","key":"4050ba0bc19dc38e2587a0837db956164f754c2364519aff1f07198612d3bf3a","iv":"5fb160e67acd8772939ba5aee58fba2a"}
```


```sh
curl -X POST http://127.0.0.1:8080/decrypt \
-H "Content-Type: application/json" \
-d '{"encrypted_data":"118e7ab1869c9d3d94199202ffc78c2760813a2c9f7b1f0024c5073c9a6a9ac3","key":"4050ba0bc19dc38e2587a0837db956164f754c2364519aff1f07198612d3bf3a","iv":"5fb160e67acd8772939ba5aee58fba2a"}'
```




```rust
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{thread_rng, Rng};
use hex::encode; // Removed decode to resolve the warning

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn main() {
    let key = thread_rng().gen::<[u8; 32]>(); // Generate a random 256-bit key
    let iv = thread_rng().gen::<[u8; 16]>(); // Generate a random initialization vector (IV)
    let data = b"Hello, AES-256-CBC!"; // Data to be encrypted

    // Create the cipher for encryption
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    // Encrypt the data
    let encrypted_data = cipher.encrypt_vec(data);
    println!("Encrypted data: {}", encode(&encrypted_data));

    // Create the cipher again for decryption
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    // Decrypt the data
    let decrypted_data = cipher.decrypt_vec(&encrypted_data).unwrap();
    println!("Decrypted data: {}", String::from_utf8_lossy(&decrypted_data));
}

```
