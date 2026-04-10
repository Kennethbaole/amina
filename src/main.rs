// Bring all Polars types and traits into scope 
use polars::prelude::*;
use std::collections::HashMap;
use polars::prelude::*;

// Return type lets us use `?` for error handling — if anything fails, it exits with the error
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the FlyWire connectome file, relative to where we run `cargo run`
    let path = "data/proofread_connections_783.feather";

    // Open the file — returns a std::fs::File handle (or errors if file not found)
    let file = std::fs::File::open(path)?;

    // IpcReader reads Arrow IPC format (which is what .feather files are)
    // .finish() actually reads and parses the file into a DataFrame
    let df = IpcReader::new(file).finish()?;

    // Print the first 5 rows — Some(5) because the method takes an Option<usize>
    /*
    println!("{}", df.head(Some(5)));
    println!("Columns: {:?}", df.get_column_names());
    println!("Shape: {:?}", df.shape());
    */

    /*
    // unique neurons:
    let pre = df.column("pre_pt_root_id")?.n_unique()?;
    println!("Unique pre neurons: {pre}");
    let post = df.column("post_pt_root_id")?.n_unique()?;
    println!("Unique post neurons: {post}");

    // printing unique values in neuropil
    let neurop = df.column("neuropil")?.n_unique()?;
    println!("Unique neuropils: {neurop}");
    println!("{:?}", df.column("neuropil")?.unique()?);

    // filtering SEZ_R
    let small = df.clone().lazy()
        .filter(col("neuropil").eq(lit("SEZ_R")))
        .collect()?;
    println!("SEZ_R shaoeL {:?}", small.shape());

    for region in ["PB", "AL_L", "AL_R", "GA_R"] {
        let subset = df.clone().lazy()
            .filter(col("neuropil").eq(lit(region)))
            .collect()?;
        println!("{}: {} connections", region, subset.shape().0);
    }

    */
    // filter to ga_z
    let ga_r = df.clone().lazy()
        .filter(col("neuropil").eq(lit("GA_R")))
        .collect()?;

    println!("Shape: {:?}", ga_r.shape()); // number of connections
    println!("{}", ga_r.head(Some(5)));

    // unique neurons in this subset 
    let one = ga_r.head(Some(1));
    println!("gaba_avg:  {:?}", one.column("gaba_avg")?);
    println!("ach_avg:   {:?}", one.column("ach_avg")?);
    
    // building the hashmap of neuron connections
    // sender, recipient, weight if fired last timestep (syn_count x sign)
    // source_neuron_id, weight 

    let counts = ga_r.column("syn_count")?.i64()?; // getting i64 column as a ChunkedArray
    println!("Counts: {:?}", counts);
    let first_value = counts.get(0);
    println!("First syn_count: {:?}", first_value);

    let pre_col = ga_r.column("pre_pt_root_id")?.i64()?;
    let post_col = ga_r.column("post_pt_root_id")?.i64()?;
    let syn_col = ga_r.column("syn_count")?.i64()?;
    let gaba = ga_r.column("gaba_avg")?.f64()?;
    let ach = ga_r.column("ach_avg")?.f64()?;
    let glut = ga_r.column("glut_avg")?.f64()?;

    let num_rows = ga_r.height();

    for i in 0..3 {
        let pre_id = pre_col.get(i).unwrap();
        let post_id = post_col.get(i).unwrap();
        let count = syn_col.get(i).unwrap();
        let g = gaba.get(i).unwrap();
        let a = ach.get(i).unwrap();
        let gl = glut.get(i).unwrap();
    
        println!("Row {i}: {pre_id} → {post_id}, syn_count={count}");
        println!("  gaba={g:.4}, ach={a:.4}, glut={gl:.4}");
    
        // Which wins?
        if a >= g && a >= gl {
            println!("  → EXCITATORY (ach wins), weight = +{count}");
        } else {
            println!("  → INHIBITORY (gaba/glut wins), weight = -{count}");
        }
        println!();
    }
    /*
    let neurop = ga_r.column("neuropil")?.i64()?;
    let oct = ga_r.column("oct_avg")?.f64()?;
    let ser = ga_r.column("ser_avg")?.f64()?;
    let da = ga_r.column("da_avg")?.f64()?;
    */

    /*
    for col in df.iter() {
        println!("Column name: {}", col.name());
    }
    */

    Ok(())
}