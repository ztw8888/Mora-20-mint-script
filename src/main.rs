use candid::{ Principal, Encode, Decode, Deserialize, CandidType };
use ic_agent::{identity, agent::http_transport};

mod did;
use crate::did::{ArticleArgs, ArticleStatus, ArticleType, OpResult};

fn build_agent(pem_identity_path: &str) -> ic_agent::Agent {
    let url = "https://icp-api.io".to_string();
    let identity = identity::Secp256k1Identity::from_pem_file(String::from(pem_identity_path)).expect("not found identity pem");
    let transport = http_transport::ReqwestTransport::create(&url).expect("create transport error");
    let agent = ic_agent::Agent::builder()
        .with_url(url)
        .with_transport(transport)
        .with_identity(identity)
        .build()
        .expect("build agent error");
    agent
}

async fn mint(
    user_mora_canister_id_text: &str,
    mint_title: &str,
    mint_content: &str
) {
    let user_mora_canister_id = Principal::from_text(user_mora_canister_id_text).unwrap();
    
    let response_blob = build_agent("./identity.pem")
        .update(
            &user_mora_canister_id,
"addArticle"
        )
        .with_arg(Encode!(&ArticleArgs{
            id: "".to_string(),
            status: ArticleStatus::Public,
            thumb: "".to_string(),
            title: mint_title.to_string(),
            content: mint_content.to_string(),
            subcate: candid::Nat::from(0usize),
            atype: ArticleType::Article,
            cate: candid::Nat::from(0usize),
            tags: [].to_vec(),
            fromurl: "".to_string(),
            r#abstract: mint_content.to_string(),
            allowComment: true,
            original: true
        }).unwrap())
        .call_and_wait()
        .await
        .expect("response error");
    
    let result = Decode!(&response_blob, OpResult).unwrap();

    match result {
        OpResult::Ok{data} => println!("Mint Ok : {:?}\n", data),
        OpResult::Err(err) => println!("Mint Err : {:?}\n", err)
    }

}

#[tokio::main]
async fn main() {
    const MORA_CANISTER_ID: &str = "erpbi-cyaaa-aaaan-qdccq-cai"; // 修改为你自己的 Mora 星球 Canister Id
    const MINT_TITLE: &str = "NNSDAO"; // 修改为你要打铭文要求的文章 Title
    // 修改为你要打铭文要求的 Mint 命令
    const MINT_CONTENT: &str = "{\"p\": \"mora-20\", \"op\": \"mint\", \"tick\": \"nnsdao\", \"amt\": \"1000\"}";
    const MINT_AMOUNT: usize = 10; // 修改为你需要批量打的张数
    let mut handles = Vec::new();

    for i in 0..MINT_AMOUNT {
        let task = tokio::spawn(mint(
            MORA_CANISTER_ID,
            MINT_TITLE,
            MINT_CONTENT
        ));
        handles.push(task);
    }

    for handle in handles {
        handle.await.expect("Task execution failed");
    }

    println!("All Mint Task Completed !");
}
