use std::collections::HashMap;

pub fn build_graph (df: &DataFrame) -> HashMap<i64, Vec<(i64, f32)>> {
    let mut graph = HashMap::new();

    let pre_col = df.column("pre_pt_root_id")?.i64()?;
    let post_col = df.column("post_pt_root_id")?.i64()?;
    let syn_col = df.column("syn_count")?.i64()?;

    let num_rows = df.height();

    for i in 0..num_rows  {
        let pre_id = pre_col.get(i).unwrap();
        let post_id = post_col.get(i).unwrap();
        let syn_count = syn_col.get(i).unwrap();

        let weight = syn_count as f32;
        graph.entry(pre_id).or_insert(Vec::new()).push((post_id, weight));
    }

    Ok(graph)
    
}