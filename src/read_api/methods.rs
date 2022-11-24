use crate::{read_api::CreatorsResponse, spinner::create_spinner, theindexio::JRPCRequest};
// use crate::spinner::create_alt_spinner;

use anyhow::Result;
use serde_json::json;

use super::Asset;

pub async fn get_assets_by_creator(creator: &str) -> Result<()> {
    let method = "get_assets_by_creator";
    let url = "https://rpc-devnet.aws.metaplex.com/";

    let mut page = 1;
    let mut assets: Vec<Asset> = Vec::new();
    let client = reqwest::Client::new();

    let spinner = create_spinner("Getting assets...");
    loop {
        let params = json!([[creator.to_string()], "created", 1000, page, "", ""]);
        let jrpc = JRPCRequest::new(method, params);

        let response = client.post(url).json(&jrpc).send().await?;
        let res: CreatorsResponse = response.json().await?;

        if res.result.items.len() == 0 {
            break;
        }

        assets.extend(res.result.items);

        page += 1;
    }
    spinner.finish();

    let mut mints = get_mint_ids_from_assets(&assets);
    mints.sort_unstable();

    let f = std::fs::File::create("mints.json")?;
    serde_json::to_writer_pretty(f, &mints)?;

    Ok(())
}

fn get_mint_ids_from_assets(assets: &Vec<Asset>) -> Vec<String> {
    let mut mints: Vec<String> = Vec::new();
    for asset in assets {
        mints.push(asset.id.clone());
    }
    mints
}
