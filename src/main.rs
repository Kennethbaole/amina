// Bring all Polars types and traits into scope 
use polars::prelude::*;
use std::collections::HashMap;


mod data; // tells rust to "look for src/data.rs"
mod graph; // tells rust to "look for src/graph.rs"

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let df = data::load_connectome("data/proofread_connections_783.feather")?;
    println!("The full dataset shape: {:?}", df.shape());

    let ga_r = data::filter_region(&df, "GA_R")?;
    println!("GA_R shape: {:?}", ga_r.shape());
    println!("GA_R head: {:?}", ga_r.head(Some(5)));

    let weighted_connections = graph::build_graph(&ga_r)?;
    println!("Receivers in GA_R: {}", weighted_connections.len());

    if let Some((id, inputs)) = weighted_connections.iter().next() {
        println!("Neuron {id} receives {} inputs, first few: {:?}",
                    inputs.len(), &inputs[..inputs.len().min(5)]);
    }

    let forward = graph::build_forward_graph(&ga_r)?;
    println!("Senders in GA_R: {}", forward.len());

    Ok(())
}