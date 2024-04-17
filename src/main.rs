use minimap2::Aligner;

fn main() {
    let mut aligner = Aligner::builder()
        .map_ont()
        .qith_seq(seq.as_bytes())
        .build()
        .unwrap();
    let hits = aligner.map(query, None);
    assert!(hits.len() > 0);
}
