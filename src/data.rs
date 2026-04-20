// loading and filtering the feather file

use polars::prelude::*;

// path: &str -> takes a string slice as input (borrowed string)
pub fn load_connectome(path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let df = IpcReader::new(file).finish()?;
    Ok(df)
}

// filtering by region
pub fn filter_region(df: &DataFrame, region: &str) -> Result<DataFrame, PolarsError> {
    let filtered = df.clone().lazy().filter(col("neuropil").eq(lit(region))).collect()?;
    Ok(filtered)
}