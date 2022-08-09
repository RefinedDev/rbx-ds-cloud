mod api;
mod cli_response;

use api::datastore_api;
use api::endpoint_structs::*;
use rbx_ds_cloud::api::response_structs::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "RefinedDev", version = "0.1.1", about = "CLI and Library for Roblox's Datastore Open Cloud API", long_about = None)]
#[clap(propagate_version = true)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ListDataStores(ListDataStores),
    ListEntries(ListEntries),
    GetEntry(GetEntry),
    ListEntryVersions(ListEntryVersions),
    GetEntryVersion(GetEntryVersion),

    SetEntry(SetEntry),
    IncrementEntry(IncrementEntry),
    DeleteEntry(DeleteEntry),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inp = CLI::parse();

    match &inp.command {
        Commands::ListDataStores(list_data_stores) => {
            let res = datastore_api::list_data_stores(list_data_stores).await?;
            let convert_to_json = res.json::<ListDataStoresResponse>().await?;

            cli_response::list_data_stores_respond(convert_to_json).await;
        }

        Commands::ListEntries(list_entries) => {
            let res = datastore_api::list_entries(list_entries).await?;
            let convert_to_json = res.json::<ListEntriesResponse>().await?;

            cli_response::list_entries_response(convert_to_json).await;
        }

        Commands::GetEntry(get_entry) => {
            let res = datastore_api::get_entry(get_entry).await?;

            cli_response::get_entry_response(res).await?;
        }

        Commands::ListEntryVersions(list_entry_versions) => {
            let res = datastore_api::list_entries_version(list_entry_versions).await?;
            let convert_to_json = res.json::<ListEntryVersionResponse>().await?;

            cli_response::list_entries_version_response(convert_to_json).await;
        }

        Commands::GetEntryVersion(get_entry_version) => {
            let res = datastore_api::get_entry_version(get_entry_version).await?;
            let headers = res.headers();

            cli_response::get_entry_version_response(headers).await;
        }

        Commands::SetEntry(set_entry) => {
            let res = datastore_api::set_entry(set_entry).await?;
            cli_response::set_entry_response(res.status()).await;
        }

        Commands::IncrementEntry(increment_entry) => {
            let res = datastore_api::increment_entry(increment_entry).await?;
            cli_response::increment_entry_response(res.status()).await;
        }

        Commands::DeleteEntry(delete_entry) => {
            let res = datastore_api::delete_entry(delete_entry).await?;
            cli_response::delete_entry_response(res.status()).await;
        }
    }

    Ok(())
}
