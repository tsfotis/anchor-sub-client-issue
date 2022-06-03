use anchor_client::solana_sdk::transaction::Transaction;
use anchor_client::solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
};
use anchor_client::Client;
use anchor_client::Cluster;
use anchor_client::EventContext;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use sub_client::{accounts, instruction, MyEvent, ID};

#[test]
fn test() {
    let cluster = Cluster::Custom("http://127.0.0.1:8899".into(), "ws://127.0.0.1:8900".into());
    let payer = Rc::new(Keypair::new());
    let client = Arc::new(
        Client::new_with_options(cluster, payer.clone(), CommitmentConfig::confirmed()).program(ID),
    );

    let sig = client
        .rpc()
        .request_airdrop(&payer.pubkey(), 200000000)
        .unwrap();
    while !client.rpc().confirm_transaction(&sig).unwrap() {
        continue;
    }

    let mut instructions = client
        .request()
        .accounts(accounts::Instruction1 {
            payer: payer.pubkey(),
        })
        .args(instruction::Instruction1)
        .instructions()
        .unwrap();
    instructions.push(
        client
            .request()
            .args(instruction::Instruction2)
            .accounts(accounts::Instruction2 {
                payer: payer.pubkey(),
            })
            .instructions()
            .unwrap()
            .pop()
            .unwrap(),
    );

    let (tx, rx) = std::sync::mpsc::channel();
    let handle = client
        .on(move |_ctx: &EventContext, event: MyEvent| {
            tx.send(event).unwrap();
        })
        .unwrap();

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    let blockhash = client.rpc().get_latest_blockhash().unwrap();
    transaction.sign(&[payer.deref()], blockhash);
    client
        .rpc()
        .send_and_confirm_transaction(&transaction)
        .unwrap();

    while let Ok(my_event) = rx.recv_timeout(std::time::Duration::from_millis(10)) {
        println!("Received event => {:?}", my_event);
    }
    std::thread::spawn(move || {
        drop(handle);
    });
}
