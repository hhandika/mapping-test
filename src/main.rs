use minimap2::Aligner;

fn main() {
    let seq = b"AAGGTTCCAA";
    let aligner = Aligner::builder().map_ont().with_seq(seq).unwrap();
    let query = b"GGTT";
    let hits = aligner.map(query, false, false, None, None).unwrap();
    if hits.is_empty() {
        println!("no hits");
    }
    for hit in hits {
        println!("hit: {:?}", hit);
    }
}
