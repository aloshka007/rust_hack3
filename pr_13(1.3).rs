fn gen_shipments(n: usize) -> Vec<u32> {
    let total = 100 * n as u32;
    let avg = total / n as u32;
    
    vec![avg; n]
}
