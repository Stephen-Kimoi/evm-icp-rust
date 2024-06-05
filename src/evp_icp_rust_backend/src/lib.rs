use ic_cdk::update; 
use ic_cdk::api::management_canister::ecdsa::{ecdsa_public_key, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument, EcdsaPublicKeyResponse}; 

#[update]
async fn get_ecdsa_public_key() -> EcdsaPublicKeyResponse {
    let (pub_key,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        key_id: key_id(), 
        ..Default::default()
    })
    .await
    .expect("Failed to get ECDSA public key");
    
    pub_key
}

fn key_id() -> EcdsaKeyId {
    EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1, 
        name: "dfx_test_key".to_string() 
    }
}

ic_cdk::export_candid!(); 