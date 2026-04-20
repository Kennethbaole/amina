// Bring all Polars types and traits into scope 
use polars::prelude::*;
use std::collections::HashMap;


mod data; // tells rust to "look for src/data.rs"
mod graph; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = data::load_connectome("data/proofread_connections_783.feather")?;
    println!("The full dataset shape: {:?}", df.shape());

    let ga_r = data::filter_region(&df, "GA_R")?;
    println!("GA_R shape: {:?}", ga_r.shape());
    Ok(())
}