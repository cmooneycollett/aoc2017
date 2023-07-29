use itertools::Itertools;

/// Calculates the sparse hash of the given strand. Returns the resulting strand, final cursor
/// value, and final skip value.
pub fn calculate_sparse_hash(
    strand: &[u64],
    lengths: &[usize],
    cursor: usize,
    skip: usize,
) -> (Vec<u64>, usize, usize) {
    let mut strand = strand.to_vec();
    let mut cursor = cursor;
    let mut skip = skip;
    for &len in lengths {
        // Reverse target segment
        let mut segment: Vec<u64> = vec![];
        for delta in 0..len {
            let i = (cursor + delta) % strand.len();
            segment.push(strand[i]);
        }
        segment.reverse();
        for (j, &segment_elem) in segment.iter().enumerate() {
            let i = (cursor + j) % strand.len();
            strand[i] = segment_elem;
        }
        // Update cursor location and increment skip value
        cursor = (cursor + len + skip) % strand.len();
        skip += 1;
    }
    (strand, cursor, skip)
}

/// Calculates the knot hash of the input string, including input processing (length sequence suffix
/// append), 64 rounds of sparse algorithm and output processing (dense hash calculation and
/// conversion to hexadecimal string).
pub fn calculate_knot_hash(input_string: &str) -> String {
    // Input processing
    let mut lengths = input_string
        .chars()
        .map(|c| c as usize)
        .collect::<Vec<usize>>();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    // Apply 64 rounds of the sparse hash algorithm
    let mut strand = (0..=255).collect::<Vec<u64>>();
    let mut cursor = 0;
    let mut skip = 0;
    for _ in 0..64 {
        (strand, cursor, skip) = calculate_sparse_hash(&strand, &lengths, cursor, skip);
    }
    // Convert to dense hash
    let mut dense_hash: Vec<u64> = vec![];
    for i in (0..strand.len()).step_by(16) {
        let mut xor = strand[i];
        for delta in 1..=15 {
            xor ^= strand[i + delta];
        }
        dense_hash.push(xor);
    }
    // Convert dense hash to hexadecimal representation
    dense_hash.iter().map(|val| format!("{:02x}", val)).join("")
}