use identity_core::{
    did::DID,
    document::DIDDocument,
    utils::{Authentication, Context, PublicKey, Service, Subject},
};

use std::str::FromStr;

use serde_diff::{Apply, Diff};

/// Test a full Did document from String.
#[test]
fn test_parse_document() {
    let json_str = r#"
    {
        "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [{
            "id": "did:iota:123456789abcdefghi#keys-1",
            "type": "RsaVerificationKey2018",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
        }, {
            "id": "did:iota:123456789abcdefghi#keys-2",
            "type": "Ed25519VerificationKey2018",
            "controller": "did:iota:pqrstuvwxyz0987654321",
            "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }, {
            "id": "did:iota:123456789abcdefghi#keys-3",
            "type": "EcdsaSecp256k1VerificationKey2019",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
        }]
    }
    "#;

    let doc = DIDDocument::from_str(json_str);

    assert!(doc.is_ok());

    let doc = doc.unwrap();
    let ctx = Context::new(vec![
        "https://w3id.org/did/v1".into(),
        "https://w3id.org/security/v1".into(),
    ]);

    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456789abcdefghi".into()],
        ..Default::default()
    }
    .init()
    .unwrap();

    assert_eq!(doc.context, ctx);
    assert_eq!(doc.id, did.into());
}

/// test doc creation via the `DIDDocument::new` method.
#[test]
fn test_doc_creation() {
    let mut did_doc = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    let service = Service::new(
        "did:into:123#edv".into(),
        "EncryptedDataVault".into(),
        "https://edv.example.com/".into(),
        None,
        None,
    )
    .unwrap();
    did_doc.update_service(service.clone());
    let public_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();
    did_doc.update_public_key(public_key.clone());

    let mut did_doc_2 = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    did_doc_2.update_service(service);
    did_doc_2.update_public_key(public_key);

    let did_doc = did_doc.init_timestamps().unwrap();

    // did_doc has timestamps while did_doc_2 does not.
    assert_ne!(did_doc, did_doc_2);
}

/// test serde_diff on the did document structure.
#[test]
fn test_doc_diff() {
    // old doc
    let mut old = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    // new doc.
    let mut new = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    let service = Service::new(
        "did:into:123#edv".into(),
        "EncryptedDataVault".into(),
        "https://edv.example.com/".into(),
        None,
        None,
    )
    .unwrap();
    new.update_service(service);
    let public_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();
    new.update_public_key(public_key);

    // diff the two docs and create a json string of the diff.
    let json_diff = serde_json::to_string(&Diff::serializable(&old, &new)).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_diff);

    // apply the json string to the old document.
    Apply::apply(&mut deserializer, &mut old).unwrap();

    // check to see that the old and new docs cotain all of the same fields.
    assert_eq!(new.to_string(), old.to_string());
}

#[test]
fn test_doc_diff_timestamps() {
    let json_str = r#"
    {
        "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
        "id": "did:iota:123456789abcdefghi",
        "updated": "2020-08-25 21:47:28.344412100 UTC",
        "publicKey": [{
            "id": "did:iota:123456789abcdefghi#keys-1",
            "type": "RsaVerificationKey2018",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
        }, {
            "id": "did:iota:123456789abcdefghi#keys-2",
            "type": "Ed25519VerificationKey2018",
            "controller": "did:iota:pqrstuvwxyz0987654321",
            "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }, {
            "id": "did:iota:123456789abcdefghi#keys-3",
            "type": "EcdsaSecp256k1VerificationKey2019",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
        }]
    }
    "#;

    let mut doc1 = DIDDocument::default();

    let doc2 = DIDDocument::from_str(json_str);

    let mut doc2 = doc2.unwrap();
    doc2.update_time();
    let diff = Diff::serializable(&doc1, &doc2);

    let json_diff = serde_json::to_string(&diff).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_diff);

    Apply::apply(&mut deserializer, &mut doc1).unwrap();

    assert_eq!(doc1.to_string(), doc2.to_string());
}

#[test]
fn test_diff_strings() {
    let diff_str = r#"
    [
    {
        "Enter": {
            "Field": "context"
        }
    },
    {
        "Enter": {
            "FieldIndex": 0
        }
    },
    {
        "Enter": {
            "CollectionIndex": 0
        }
    },
    {
        "Value": "https://w3id.org/did/v1"
    },
    "Exit",
    "Exit",
    {
        "Enter": {
            "Field": "id"
        }
    },
    {
        "Enter": {
            "FieldIndex": 0
        }
    },
    {
        "Enter": {
            "Field": "method_name"
        }
    },
    {
        "Value": "iota"
    },
    {
        "Enter": {
            "Field": "id_segments"
        }
    },
    {
        "Enter": "AddToCollection"
    },
    {
        "Value": "123456789abcdefghi"
    },
    "Exit",
    "Exit",
    "Exit",
    {
        "Enter": {
            "Field": "public_key"
        }
    },
    {
        "Enter": "AddToCollection"
    },
    {
        "Value": {
            "id": "did:iota:123456789abcdefghi#keys-1",
            "type": "RsaVerificationKey2018",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }
    },
    "Exit",
    {
        "Enter": {
            "Field": "services"
        }
    },
    {
        "Enter": "AddToCollection"
    },
    {
        "Value": {
            "id": "did:into:123#edv",
            "type": "EncryptedDataVault",
            "serviceEndpoint": "https://edv.example.com/"
        }
    },
    "Exit"
    ]  
    "#;

    let def_doc = DIDDocument::default();

    let mut doc = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    let service = Service::new(
        "did:into:123#edv".into(),
        "EncryptedDataVault".into(),
        "https://edv.example.com/".into(),
        None,
        None,
    )
    .unwrap();
    doc.update_service(service);
    let public_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();
    doc.update_public_key(public_key);

    let json_diff = serde_json::to_string(&Diff::serializable(&def_doc, &doc)).unwrap();

    // remove newlines and spaces from the diff_str.
    assert_eq!(json_diff, diff_str.replace("\n", "").replace(" ", ""))
}

#[test]
fn test_doc_metadata() {
    use std::collections::HashMap;

    let json_str = r#"
    {
        "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
        "id": "did:iota:123456789abcdefghi",
        "updated": "2020-08-25 21:47:28.344412100 UTC",
        "publicKey": [{
            "id": "did:iota:123456789abcdefghi#keys-1",
            "type": "RsaVerificationKey2018",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
        }, {
            "id": "did:iota:123456789abcdefghi#keys-2",
            "type": "Ed25519VerificationKey2018",
            "controller": "did:iota:pqrstuvwxyz0987654321",
            "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }, {
            "id": "did:iota:123456789abcdefghi#keys-3",
            "type": "EcdsaSecp256k1VerificationKey2019",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
        }]
    }
    "#;

    let result_str = r#"
    {
        "@context": [
            "https://w3id.org/did/v1",
            "https://w3id.org/security/v1"
        ],
        "id": "did:iota:123456789abcdefghi",
        "updated": "2020-08-25 21:47:28.344412100 UTC",
        "publicKey": [
            {
                "id": "did:iota:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-3",
                "type": "EcdsaSecp256k1VerificationKey2019",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
            }
        ],
        "some_more": "metadata_stuff",
        "some": "metadata"
    }
    "#;

    let doc = DIDDocument::from_str(json_str);

    assert!(doc.is_ok());

    let doc = doc.unwrap();
    let mut metadata = HashMap::new();
    metadata.insert("some".into(), "metadata".into());
    metadata.insert("some_more".into(), "metadata_stuff".into());

    let doc = doc.supply_metadata(metadata).unwrap();

    let res_doc = DIDDocument::from_str(result_str).unwrap();

    assert_eq!(doc, res_doc);
}

#[test]
fn test_auth() {
    let json_str = r#"
    {
        "@context": [
            "https://w3id.org/did/v1",
            "https://w3id.org/security/v1"
        ],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [
            {
                "id": "did:iota:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ],
        "authentication": [
            "did:iota:123456789abcdefghi#keys-1",
            "did:iota:123456789abcdefghi#biometric-1",
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ]
    }
    "#;

    let doc_1 = DIDDocument::from_str(json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key1 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyPem".into(),
        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into(),
    )
    .unwrap();

    let key2 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:pqrstuvwxyz0987654321".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    let auth1 = Authentication::Method("did:iota:123456789abcdefghi#keys-1".into());
    let auth2 = Authentication::Method("did:iota:123456789abcdefghi#biometric-1".into());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_auth(auth1);
    doc_2.update_auth(auth2);
    doc_2.update_auth(auth3);

    let doc_2 = doc_2.init();

    assert_eq!(doc_1, doc_2);
}
