use web3::types::{Block,BlockId, BlockNumber};
use web3::types::U64;
use std::fs;
use std::fs::File;
use std::io::{ Write};
use std::io;
use std::path::Path;
use std::error::Error;
use std::fs::OpenOptions;
use regex::Regex;
use std::io::prelude::*;
use std::env;
use sqlx::mysql::{MySqlPool,MySqlConnection};
use sqlx::prelude::*;

#[tokio::main]
async fn main() -> web3::Result<()>{
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    let mut pool = MySqlConnection::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();

    let blockNum = web3.eth().block_number().await?;
    println!("high is : {:?}",blockNum);
    let blockNum2 = blockNum.to_string() ;
    let blockNum3 = blockNum2.parse::<i64>().unwrap();
    //let result = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(1)))).await?;
    //let mut f = File::create("./word/info").unwrap();
    //write!(f,"{:?}",result);
    //println!("第一个区块信息：{:?}",result);
    let mut i = 1;
    let l = blockNum3+1;
    let mut f = File::create("./word/1").unwrap();
    while i != l {
        //let mut f = File::create("./word/1").unwrap();
        fs::copy("./word/1","./word/2");
        let mut f2 = File::create("./word/2").unwrap();
        let i_str = i.to_string();
        let v:Vec<&str> = i_str.split(".").collect();
        let filename = "word/block".to_owned() + v[0] + ".word";
        let result = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(i)))).await?;
        let result2 = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(i)))).await?;
        if let Some(data) = result {
            let mut eth_block = serde_json::to_value(&data).unwrap();
            let mut json = serde_json::to_string_pretty(&data).unwrap();

            let hash = eth_block["hash"].as_str().unwrap();
            let parent_hash = eth_block["parentHash"].as_str().unwrap();
            let sha3_uncles = eth_block["sha3Uncles"].as_str().unwrap();
            let miner = eth_block["miner"].as_str().unwrap();
            let state_root = eth_block["stateRoot"].as_str().unwrap();
            let transactions_root = eth_block["transactionsRoot"].as_str().unwrap();
            let receipts_root = eth_block["receiptsRoot"].as_str().unwrap();
            let number = eth_block["number"].as_str().unwrap();
            let gas_used = eth_block["gasUsed"].as_str().unwrap();
            let gas_limit = eth_block["gasLimit"].as_str().unwrap();
            let base_fee_per_gas = "null";
            let extra_data = eth_block["extraData"].as_str().unwrap();
            let logs_bloom = eth_block["logsBloom"].as_str().unwrap();
            let timestamp = eth_block["timestamp"].as_str().unwrap();
            let difficulty = eth_block["difficulty"].as_str().unwrap();
            let total_difficulty = eth_block["totalDifficulty"].as_str().unwrap();
            let seal_fields = "[]";
            let uncles = "[]";
            let size = eth_block["size"].as_str().unwrap();
            let mix_hash = "null";
            let nonce = "null";
            let tx = eth_block["transactions"].as_array().unwrap();
            let transactions = tx.len().to_string();
            let size = eth_block["size"].as_str().unwrap();
            let mix_hash = "null";
            let nonce = "null";
            for i in tx {
                // println!("{}", i);//single tx.

                let hash = i["hash"].as_str().unwrap();
                let nonce = i["nonce"].as_str().unwrap();
                let block_hash = i["blockHash"].as_str().unwrap();
                let block_number = i["blockNumber"].as_str().unwrap();
                let transaction_index = i["transactionIndex"].as_str().unwrap();
                let from_addr = i["from"].as_str().unwrap();
                let to_addr = i["to"].as_str().unwrap();
                let value = i["value"].as_str().unwrap();
                let gas_price = i["gasPrice"].as_str().unwrap();
                let gas = i["gas"].as_str().unwrap();
                let input = i["input"].as_str().unwrap();
                let v = i["v"].as_str().unwrap();
                let r = i["r"].as_str().unwrap();
                let s = i["s"].as_str().unwrap();
                let raw = i["raw"].as_str().unwrap();
                let sql="INSERT INTO Tx VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
                let count=sqlx::query(sql).bind(hash).bind(nonce).bind(block_hash).bind(block_number).bind(transaction_index).bind(from_addr)
                    .bind(to_addr).bind(value).bind(gas_price).bind(gas).bind(input).bind(v).bind(r).bind(s)
                    .bind(raw).execute(&mut pool).await.unwrap();
               //println!("{}",count);
            }
        let sql2 = "INSERT INTO Head VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let count2 = sqlx::query(sql2).bind(hash).bind(parent_hash).bind(sha3_uncles).bind(miner).bind(state_root).bind(transactions_root)
            .bind(receipts_root).bind(number).bind(gas_used).bind(gas_limit).bind(base_fee_per_gas).bind(extra_data).bind(logs_bloom).bind(timestamp)
            .bind(difficulty).bind(total_difficulty).bind(transactions).bind(size).bind(mix_hash).bind(nonce).execute(&mut pool).await.unwrap();
        //println!("{}", count2);
        }

        write!(f2,"{:?}",result2);
        if !Path::new("word").exists(){
            fs::create_dir("word")?;
        }
        let mut f3 = File::open("./word/2").unwrap();
        let mut buf = String::new();
        f3.read_to_string(&mut buf).unwrap();
        let re = Regex::new(r"Some").unwrap();
        let re2 = Regex::new(r"[()]").unwrap();
        let buf2 = re.replace_all(&*buf, "");
        let buf3 = re2.replace_all(&*buf2,"");
        fs::rename("./word/2",filename)?;
        //println!("第{}个区块信息：{:?}",i, result);
        i += 1;
    }
    //let result2 = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(blockNum)))).await?;
    //println!("第{:?}个区块信息:{:?}",blockNum,result2);
    /*let path = Path::new("./word");
    for entry in fs::read_dir(path).expect("读取目录失败") {
        if let Ok(entry) = entry{
            let file = entry.path();
            let mut f = File::open(file).unwrap();
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            let re = Regex::new(r"Some").unwrap();
            let re2 = Regex::new(r"[()]").unwrap();
            let buf2 = re.replace_all(&*buf, "");
            let buf3 = re2.replace_all(&*buf2, "");
        }
    }*/
    Ok(())
}
