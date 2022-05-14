pub fn build_sorted_dimension_vec(s: &str) -> Vec<u32> {
    let mut split_dims: Vec<u32> = s
        .split("x")
        .filter_map(|e| match e.parse::<u32>().unwrap() {
            0u32 => panic!("Encountered a zero dimension!"),
            _ => Some(e.parse::<u32>().unwrap()),
        })
        .collect::<Vec<u32>>();
    split_dims.sort();
    return split_dims;
}
