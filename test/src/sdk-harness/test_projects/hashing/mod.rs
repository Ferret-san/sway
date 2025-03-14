use fuel_tx::ContractId;
use fuels::prelude::*;
use fuels::test_helpers;
use fuels_abigen_macro::abigen;
use sha2::{Digest, Sha256};
use sha3::Keccak256;

abigen!(
    HashingTestContract,
    "test_projects/hashing/out/debug/hashing-abi.json"
);

enum Hash {
    Sha256,
    Keccak256,
}

fn hash_u64(number: u64, algorithm: Hash) -> [u8; 32] {
    // Note!
    // Numbers will be padded into u64 in sway regardless of whether you declare a smaller type
    // Therefore tests pass because we use a rust u64 type rather than any smaller type
    match algorithm {
        Hash::Sha256 => Sha256::digest(number.to_be_bytes()).into(),
        Hash::Keccak256 => Keccak256::digest(number.to_be_bytes()).into(),
    }
}

fn hash_bool(value: bool, algorithm: Hash) -> [u8; 32] {
    let hash = match algorithm {
        Hash::Sha256 => {
            if value {
                Sha256::digest([0, 0, 0, 0, 0, 0, 0, 1])
            } else {
                Sha256::digest([0, 0, 0, 0, 0, 0, 0, 0])
            }
        }
        Hash::Keccak256 => {
            if value {
                Keccak256::digest([0, 0, 0, 0, 0, 0, 0, 1])
            } else {
                Keccak256::digest([0, 0, 0, 0, 0, 0, 0, 0])
            }
        }
    };

    hash.into()
}

fn hash_str(text: &str, algorithm: Hash) -> [u8; 32] {
    let mut buffer: Vec<u8> = Vec::new();
    for character in text.chars() {
        buffer.push(character as u8);
    }

    while buffer.len() % 8 != 0 {
        buffer.push(0);
    }

    match algorithm {
        Hash::Sha256 => Sha256::digest(buffer).into(),
        Hash::Keccak256 => Keccak256::digest(buffer).into(),
    }
}

fn hash_b256(arr: [u8; 32], algorithm: Hash) -> [u8; 32] {
    match algorithm {
        Hash::Sha256 => Sha256::digest(arr).into(),
        Hash::Keccak256 => Keccak256::digest(arr).into(),
    }
}

fn hash_tuple(arr: [u8; 16], algorithm: Hash) -> [u8; 32] {
    // A tuple is hashed by converting each element into bytes and then combining them together
    // in the sequential order that they are in the tuple
    // E.g. (true, 5) -> [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5]
    match algorithm {
        Hash::Sha256 => Sha256::digest(arr).into(),
        Hash::Keccak256 => Keccak256::digest(arr).into(),
    }
}

fn hash_array(arr: [u8; 16], algorithm: Hash) -> [u8; 32] {
    // An array is hashed by converting each element into bytes and then combining them together
    // in the sequential order that they are in the array
    // E.g. (18, 555) -> [0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 2, 43]
    match algorithm {
        Hash::Sha256 => Sha256::digest(arr).into(),
        Hash::Keccak256 => Keccak256::digest(arr).into(),
    }
}

fn hash_enum(arr: [u8; 16], algorithm: Hash) -> [u8; 32] {
    /*
        An enum consists of 2 parts in the 16 byte array
        The first 8 bytes are for the values that the enum can have
        Idk what the second 8 bytes are for

        enum Test {
            A,
            B,
            C
        }

        arr of Test::A will be
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        arr of Test::B will be
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]

        arr of Test::C will be
        [0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0]
    */
    match algorithm {
        Hash::Sha256 => Sha256::digest(arr).into(),
        Hash::Keccak256 => Keccak256::digest(arr).into(),
    }
}

fn hash_struct(arr: [u8; 88], algorithm: Hash) -> [u8; 32] {
    match algorithm {
        Hash::Sha256 => Sha256::digest(arr).into(),
        Hash::Keccak256 => Keccak256::digest(arr).into(),
    }
}

async fn get_hashing_instance() -> (HashingTestContract, ContractId) {
    let compiled =
        Contract::load_sway_contract("test_projects/hashing/out/debug/hashing.bin").unwrap();

    let (provider, wallet) = test_helpers::setup_test_provider_and_wallet().await;

    let id = Contract::deploy(&compiled, &provider, &wallet, TxParameters::default())
        .await
        .unwrap();
    let instance = HashingTestContract::new(id.to_string(), provider, wallet);

    (instance, id)
}

mod sha256 {

    use super::*;

    #[tokio::test]
    async fn test_u8() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_u64(254, Hash::Sha256);
        let expected_2 = hash_u64(253, Hash::Sha256);

        let call_1 = instance.sha256_u8(254u8).call().await.unwrap();
        let call_2 = instance.sha256_u8(254u8).call().await.unwrap();
        let call_3 = instance.sha256_u8(253u8).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_u16() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_u64(65534, Hash::Sha256);
        let expected_2 = hash_u64(65533, Hash::Sha256);

        let call_1 = instance.sha256_u16(65534u16).call().await.unwrap();
        let call_2 = instance.sha256_u16(65534u16).call().await.unwrap();
        let call_3 = instance.sha256_u16(65533u16).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_u32() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_u64(4294967294, Hash::Sha256);
        let expected_2 = hash_u64(4294967293, Hash::Sha256);

        let call_1 = instance.sha256_u32(4294967294u32).call().await.unwrap();
        let call_2 = instance.sha256_u32(4294967294u32).call().await.unwrap();
        let call_3 = instance.sha256_u32(4294967293u32).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_u64() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_u64(18446744073709551613, Hash::Sha256);
        let expected_2 = hash_u64(18446744073709551612, Hash::Sha256);

        let call_1 = instance
            .sha256_u64(18446744073709551613)
            .call()
            .await
            .unwrap();
        let call_2 = instance
            .sha256_u64(18446744073709551613)
            .call()
            .await
            .unwrap();
        let call_3 = instance
            .sha256_u64(18446744073709551612)
            .call()
            .await
            .unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_bool() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_bool(true, Hash::Sha256);
        let expected_2 = hash_bool(false, Hash::Sha256);

        let call_1 = instance.sha256_bool(true).call().await.unwrap();
        let call_2 = instance.sha256_bool(true).call().await.unwrap();
        let call_3 = instance.sha256_bool(false).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_str() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_str("John", Hash::Sha256);
        let expected_2 = hash_str("Nick", Hash::Sha256);

        let call_1 = instance
            .sha256_str(String::from("John"))
            .call()
            .await
            .unwrap();
        let call_2 = instance
            .sha256_str(String::from("John"))
            .call()
            .await
            .unwrap();
        let call_3 = instance
            .sha256_str(String::from("Nick"))
            .call()
            .await
            .unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_b256() {
        let (instance, _id) = get_hashing_instance().await;

        let address1 = [
            118, 64, 238, 245, 229, 5, 191, 187, 201, 174, 141, 75, 72, 119, 88, 252, 38, 62, 110,
            176, 51, 16, 126, 190, 233, 136, 54, 127, 90, 101, 230, 168,
        ];
        let address2 = [
            8, 4, 28, 217, 200, 5, 161, 17, 20, 214, 54, 77, 72, 118, 90, 31, 225, 63, 110, 77,
            190, 190, 12, 1, 233, 48, 54, 72, 90, 253, 100, 103,
        ];

        let expected_1 = hash_b256(address1, Hash::Sha256);
        let expected_2 = hash_b256(address2, Hash::Sha256);

        let call_1 = instance.sha256_b256(address1).call().await.unwrap();
        let call_2 = instance.sha256_b256(address1).call().await.unwrap();
        let call_3 = instance.sha256_b256(address2).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_tuple() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_tuple(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5],
            Hash::Sha256,
        );
        let expected_2 = hash_tuple(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 6],
            Hash::Sha256,
        );

        let call_1 = instance.sha256_tuple((true, 5)).call().await.unwrap();
        let call_2 = instance.sha256_tuple((true, 5)).call().await.unwrap();
        let call_3 = instance.sha256_tuple((true, 6)).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_array() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_array(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5],
            Hash::Sha256,
        );
        let expected_2 = hash_array(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 6],
            Hash::Sha256,
        );

        let call_1 = instance.sha256_array(1, 5).call().await.unwrap();
        let call_2 = instance.sha256_array(1, 5).call().await.unwrap();
        let call_3 = instance.sha256_array(1, 6).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_enum() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_enum(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Hash::Sha256,
        );
        let expected_2 = hash_enum(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            Hash::Sha256,
        );

        let call_1 = instance.sha256_enum(true).call().await.unwrap();
        let call_2 = instance.sha256_enum(true).call().await.unwrap();
        let call_3 = instance.sha256_enum(false).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_struct() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_struct(
            [
                74, 111, 104, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
            Hash::Sha256,
        );
        let expected_2 = hash_struct(
            [
                74, 111, 104, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
            Hash::Sha256,
        );

        let call_1 = instance.sha256_struct(true).call().await.unwrap();
        let call_2 = instance.sha256_struct(true).call().await.unwrap();
        let call_3 = instance.sha256_struct(false).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }
}

mod keccak256 {

    use super::*;

    #[tokio::test]
    async fn test_u8() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_u64(254, Hash::Keccak256);
        let expected_2 = hash_u64(253, Hash::Keccak256);

        let call_1 = instance.keccak256_u8(254u8).call().await.unwrap();
        let call_2 = instance.keccak256_u8(254u8).call().await.unwrap();
        let call_3 = instance.keccak256_u8(253u8).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_u16() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_u64(65534, Hash::Keccak256);
        let expected_2 = hash_u64(65533, Hash::Keccak256);

        let call_1 = instance.keccak256_u16(65534u16).call().await.unwrap();
        let call_2 = instance.keccak256_u16(65534u16).call().await.unwrap();
        let call_3 = instance.keccak256_u16(65533u16).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_u32() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_u64(4294967294, Hash::Keccak256);
        let expected_2 = hash_u64(4294967293, Hash::Keccak256);

        let call_1 = instance.keccak256_u32(4294967294u32).call().await.unwrap();
        let call_2 = instance.keccak256_u32(4294967294u32).call().await.unwrap();
        let call_3 = instance.keccak256_u32(4294967293u32).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_u64() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_u64(18446744073709551613, Hash::Keccak256);
        let expected_2 = hash_u64(18446744073709551612, Hash::Keccak256);

        let call_1 = instance
            .keccak256_u64(18446744073709551613)
            .call()
            .await
            .unwrap();
        let call_2 = instance
            .keccak256_u64(18446744073709551613)
            .call()
            .await
            .unwrap();
        let call_3 = instance
            .keccak256_u64(18446744073709551612)
            .call()
            .await
            .unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_bool() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_bool(true, Hash::Keccak256);
        let expected_2 = hash_bool(false, Hash::Keccak256);

        let call_1 = instance.keccak256_bool(true).call().await.unwrap();
        let call_2 = instance.keccak256_bool(true).call().await.unwrap();
        let call_3 = instance.keccak256_bool(false).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_str() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_str("John", Hash::Keccak256);
        let expected_2 = hash_str("Nick", Hash::Keccak256);

        let call_1 = instance
            .keccak256_str(String::from("John"))
            .call()
            .await
            .unwrap();
        let call_2 = instance
            .keccak256_str(String::from("John"))
            .call()
            .await
            .unwrap();
        let call_3 = instance
            .keccak256_str(String::from("Nick"))
            .call()
            .await
            .unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_b256() {
        let (instance, _id) = get_hashing_instance().await;

        let address1 = [
            118, 64, 238, 245, 229, 5, 191, 187, 201, 174, 141, 75, 72, 119, 88, 252, 38, 62, 110,
            176, 51, 16, 126, 190, 233, 136, 54, 127, 90, 101, 230, 168,
        ];
        let address2 = [
            8, 4, 28, 217, 200, 5, 161, 17, 20, 214, 54, 77, 72, 118, 90, 31, 225, 63, 110, 77,
            190, 190, 12, 1, 233, 48, 54, 72, 90, 253, 100, 103,
        ];

        let expected_1 = hash_b256(address1, Hash::Keccak256);
        let expected_2 = hash_b256(address2, Hash::Keccak256);

        let call_1 = instance.keccak256_b256(address1).call().await.unwrap();
        let call_2 = instance.keccak256_b256(address1).call().await.unwrap();
        let call_3 = instance.keccak256_b256(address2).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_tuple() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_tuple(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5],
            Hash::Keccak256,
        );
        let expected_2 = hash_tuple(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 6],
            Hash::Keccak256,
        );

        let call_1 = instance.keccak256_tuple((true, 5)).call().await.unwrap();
        let call_2 = instance.keccak256_tuple((true, 5)).call().await.unwrap();
        let call_3 = instance.keccak256_tuple((true, 6)).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_array() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_array(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5],
            Hash::Keccak256,
        );
        let expected_2 = hash_array(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 6],
            Hash::Keccak256,
        );

        let call_1 = instance.keccak256_array(1, 5).call().await.unwrap();
        let call_2 = instance.keccak256_array(1, 5).call().await.unwrap();
        let call_3 = instance.keccak256_array(1, 6).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_enum() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_enum(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Hash::Keccak256,
        );
        let expected_2 = hash_enum(
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            Hash::Keccak256,
        );

        let call_1 = instance.keccak256_enum(true).call().await.unwrap();
        let call_2 = instance.keccak256_enum(true).call().await.unwrap();
        let call_3 = instance.keccak256_enum(false).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }

    #[tokio::test]
    async fn test_struct() {
        let (instance, _id) = get_hashing_instance().await;

        let expected_1 = hash_struct(
            [
                74, 111, 104, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
            Hash::Keccak256,
        );
        let expected_2 = hash_struct(
            [
                74, 111, 104, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
            Hash::Keccak256,
        );

        let call_1 = instance.keccak256_struct(true).call().await.unwrap();
        let call_2 = instance.keccak256_struct(true).call().await.unwrap();
        let call_3 = instance.keccak256_struct(false).call().await.unwrap();

        assert_eq!(call_1.value, call_2.value);
        assert_ne!(call_1.value, call_3.value);

        assert_eq!(expected_1, call_1.value);
        assert_eq!(expected_2, call_3.value);
    }
}
