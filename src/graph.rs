use polars::prelude::*;
use std::collections::HashMap;

pub fn build_graph(
    df: &DataFrame,
) -> Result<HashMap<i64, Vec<(i64, f32)>>, Box<dyn std::error::Error>> {
    let mut graph: HashMap<i64, Vec<(i64, f32)>> = HashMap::new();

    let pre_col = df.column("pre_pt_root_id")?.i64()?;
    let post_col = df.column("post_pt_root_id")?.i64()?;
    let syn_col = df.column("syn_count")?.i64()?;
    let gaba = df.column("gaba_avg")?.f64()?;
    let ach = df.column("ach_avg")?.f64()?;
    let glut = df.column("glut_avg")?.f64()?;

    let num_rows = df.height();

    for i in 0..num_rows {
        let pre_id = pre_col.get(i).unwrap();
        let post_id = post_col.get(i).unwrap();
        let count = syn_col.get(i).unwrap();
        let g = gaba.get(i).unwrap();
        let a = ach.get(i).unwrap();
        let gl = glut.get(i).unwrap();

        // Determine sign: acetylcholine = excitatory, gaba/glut = inhibitory
        let sign: f32 = if a >= g && a >= gl { 1.0 } else { -1.0 };
        let weight = count as f32 * sign;

        // Insert into the receiver's "mailbox"
        graph.entry(post_id)
            .or_insert_with(Vec::new)
            .push((pre_id, weight));
    }

    Ok(graph)
}

// Forward graph: for each sender, list the neurons it targets.
// Use this when propagating signals: "neuron X fired — who does it send to?"
pub fn build_forward_graph(
    df: &DataFrame,
) -> Result<HashMap<i64, Vec<(i64, f32)>>, Box<dyn std::error::Error>> {
    let mut graph: HashMap<i64, Vec<(i64, f32)>> = HashMap::new();

    let pre_col = df.column("pre_pt_root_id")?.i64()?;
    let post_col = df.column("post_pt_root_id")?.i64()?;
    let syn_col = df.column("syn_count")?.i64()?;
    let gaba = df.column("gaba_avg")?.f64()?;
    let ach = df.column("ach_avg")?.f64()?;
    let glut = df.column("glut_avg")?.f64()?;

    for i in 0..df.height() {
        let pre_id = pre_col.get(i).unwrap();
        let post_id = post_col.get(i).unwrap();
        let count = syn_col.get(i).unwrap();
        let g = gaba.get(i).unwrap();
        let a = ach.get(i).unwrap();
        let gl = glut.get(i).unwrap();

        let sign: f32 = if a >= g && a >= gl { 1.0 } else { -1.0 };
        let weight = count as f32 * sign;

        // Key by the sender this time
        graph.entry(pre_id)
            .or_insert_with(Vec::new)
            .push((post_id, weight));
    }

    Ok(graph)
}