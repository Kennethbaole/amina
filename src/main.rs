// Bring all Polars types and traits into scope 
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
        .filter(col("neuropil").eq(lit("GA_Z")))
        .collect()?;

    println!("Shape: {:?}", ga_r.shape()); // number of connections
    println!("{}", ga_r.head(Some(5)));

    
    
    Ok(())
}