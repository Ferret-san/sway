use fuels::{
    prelude::{Contract, TxParameters, LocalWallet, CallParameters},
    signers::Signer,
    test_helpers,
};
use fuels_abigen_macro::abigen;
use fuel_tx::{ContractId, Salt, AssetId};

abigen!(Escrow, "out/debug/escrow-abi.json");
abigen!(Asset, "tests/artifacts/asset/out/debug/asset-abi.json");

// TODO: if contract storage is exposed then testing should be updated to validate state instead of only the return from a function
// TODO: fix tests so that they mint and send to address correctly

struct Metadata {
    escrow: Escrow,
    asset: Option<Asset>,
    wallet: LocalWallet
}

async fn setup() -> (Metadata, Metadata, Metadata, ContractId) {
    // Deploy the compiled contract
    let salt = Salt::from([0u8; 32]);
    let compiled_escrow = Contract::load_sway_contract("./out/debug/escrow.bin", salt).unwrap();
    let compiled_asset = Contract::load_sway_contract("./tests/artifacts/asset/out/debug/asset.bin", salt).unwrap();

    // Launch a local network and deploy the contract
    let (provider, deployer_wallet) = test_helpers::setup_test_provider_and_wallet().await;
    let (_, buyer_wallet) = test_helpers::setup_test_provider_and_wallet().await;
    let (_, seller_wallet) = test_helpers::setup_test_provider_and_wallet().await;

    let escrow_id = Contract::deploy(&compiled_escrow, &provider, &deployer_wallet, TxParameters::default())
        .await
        .unwrap();
    
    let asset_id = Contract::deploy(&compiled_asset, &provider, &deployer_wallet, TxParameters::default())
        .await
        .unwrap();

    let deployer = Metadata {
        escrow: Escrow::new(escrow_id.to_string(), provider.clone(), deployer_wallet.clone()),
        asset: Some(Asset::new(asset_id.to_string(), provider.clone(), deployer_wallet.clone())),
        wallet: deployer_wallet
    };

    let buyer = Metadata {
        escrow: Escrow::new(escrow_id.to_string(), provider.clone(), buyer_wallet.clone()),
        asset: None,
        wallet: buyer_wallet
    };

    let seller = Metadata {
        escrow: Escrow::new(escrow_id.to_string(), provider.clone(), seller_wallet.clone()),
        asset: None,
        wallet: seller_wallet
    };

    (deployer, buyer, seller, asset_id)
}

mod constructor {

    use super::*;

    #[tokio::test]
    async fn initializes() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;
    
        assert!(deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap().value);
    }

}

mod deposit {

    use super::*;

    #[tokio::test]
    async fn deposits() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;
        
        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();
        deployer.asset.unwrap().mint_and_send_to_address(amount, buyer.wallet.address()).append_variable_outputs(1).call().await.unwrap();

        // TODO: these should be equal but instead it panics because it does not send the coins. This applies to all subsequent tests
        // assert_eq!(
        //     buyer.wallet
        //         .get_spendable_coins(&AssetId::from(*asset_id), amount)
        //         .await
        //         .unwrap()[0]
        //         .amount,
        //     amount.into()
        // );

        // TODO: add 2 assertions
        //       - 1) buyer has asset amount
        //       - 2) contract does not have asset amount

        // Test
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);    
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)));

        assert!(buyer.escrow.deposit().tx_params(tx_params).call_params(call_params).call().await.unwrap().value);

        // TODO: add 2 assertions
        //       - 1) buyer no longer has asset amount
        //       - 2) contract has asset amount
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_with_incorrect_state() {
        let (_, buyer, _, _) = setup().await;

        // Should panic
        buyer.escrow.deposit().call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_with_incorrect_asset() {
        todo!();
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_with_incorrect_asset_amount() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;

        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();
        deployer.asset.unwrap().mint_and_send_to_address(amount, buyer.wallet.address()).append_variable_outputs(1).call().await.unwrap();
        
        // Should panic
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);    
        let call_params = CallParameters::new(Some(amount-1), Some(AssetId::from(*asset_id)));
        buyer.escrow.deposit().tx_params(tx_params).call_params(call_params).call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_when_sender_is_not_the_correct_address() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;

        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();
        deployer.asset.unwrap().mint_and_send_to_address(amount, deployer.wallet.address()).append_variable_outputs(1).call().await.unwrap();
        
        // Should panic
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);    
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)));
        deployer.escrow.deposit().tx_params(tx_params).call_params(call_params).call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_when_already_deposited() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;

        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();
        deployer.asset.unwrap().mint_and_send_to_address(amount*2, buyer.wallet.address()).append_variable_outputs(1).call().await.unwrap();
        
        let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
        let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);

        let call_params1 = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)));
        let call_params2 = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)));

        buyer.escrow.deposit().tx_params(tx_params1).call_params(call_params1).call().await.unwrap();

        // Should panic
        buyer.escrow.deposit().tx_params(tx_params2).call_params(call_params2).call().await.unwrap();
    }

}

mod approve {

    use super::*;

    #[tokio::test]
    async fn approves() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;

        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();
        deployer.asset.as_ref().unwrap().mint_and_send_to_address(amount, buyer.wallet.address()).append_variable_outputs(1).call().await.unwrap();
        deployer.asset.unwrap().mint_and_send_to_address(amount, seller.wallet.address()).append_variable_outputs(1).call().await.unwrap();
        
        let tx_params1 = TxParameters::new(None, Some(1_000_000), None, None);
        let tx_params2 = TxParameters::new(None, Some(1_000_000), None, None);

        let call_params1 = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)));
        let call_params2 = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)));

        buyer.escrow.deposit().tx_params(tx_params1).call_params(call_params1).call().await.unwrap();
        seller.escrow.deposit().tx_params(tx_params2).call_params(call_params2).call().await.unwrap();

        // TODO: add 2 assertions
        //       - 1) buyer & seller no longer has asset amount
        //       - 2) contract has asset 2*amount

        // Test
        assert!(buyer.escrow.approve().call().await.unwrap().value);
        assert!(seller.escrow.approve().append_variable_outputs(2).call().await.unwrap().value);

        // TODO: add 2 assertions
        //       - 1) buyer & seller each have their asset amount back
        //       - 2) contract has no asset amount
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_with_incorrect_state() {
        let (_, buyer, _, _) = setup().await;
        
        // Should panic
        buyer.escrow.approve().call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_when_sender_is_not_the_correct_address() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;
        
        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();

        // Should panic
        deployer.escrow.approve().call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_when_not_deposited() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;

        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();

        // Should panic
        buyer.escrow.approve().call().await.unwrap();
    }

}

mod withdraw {

    use super::*;

    #[tokio::test]
    async fn withdraws() {    
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;

        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();
        deployer.asset.unwrap().mint_and_send_to_address(amount, buyer.wallet.address()).append_variable_outputs(1).call().await.unwrap();

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)));
        buyer.escrow.deposit().tx_params(tx_params).call_params(call_params).call().await.unwrap();
        
        // TODO: add 2 assertions
        //       - 1) buyer no longer has asset amount
        //       - 2) contract has asset amount 

        // Test
        assert!(buyer.escrow.withdraw().append_variable_outputs(1).call().await.unwrap().value);

        // TODO: add 2 assertions
        //       - 1) buyer has their asset amount back
        //       - 2) contract no longer has asset amount
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_with_incorrect_state() {
        let (_, buyer, _, _) = setup().await;
        
        // Should panic
        buyer.escrow.withdraw().call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_when_sender_is_not_the_correct_address() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;

        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();

        // Should panic
        deployer.escrow.withdraw().call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RESERV00")]
    async fn panics_when_not_deposited() {
        let amount: u64 = 100;
        let (deployer, buyer, seller, asset_id) = setup().await;

        // Init conditions
        deployer.escrow.constructor(buyer.wallet.address(), seller.wallet.address(), asset_id, amount).call().await.unwrap();

        // Should panic
        buyer.escrow.withdraw().call().await.unwrap();
    }

}
