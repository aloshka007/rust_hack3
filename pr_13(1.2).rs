fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    
    if total % n as u32 != 0 {
        return None;
    }

    let avg = total / n as u32;
    let mut moves = 0;

    let mut diff = 0;
    for &shipment in shipments.iter() {
        diff += (shipment as i32) - (avg as i32);
        moves += diff.abs() as usize;
    }

    Some(moves)
}
