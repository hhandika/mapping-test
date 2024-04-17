use minimap2::Aligner;

fn main() {
    let seq = b">ratete-330\nAAGGTTCCAA\n>tattaa\nGGTT\n";
    let aligner = Aligner::builder().short().with_seq(seq).unwrap();
    let query = b"GGTT";
    let hits = aligner.map(query, false, false, None, None).unwrap();
    if hits.is_empty() {
        println!("no hits");
    }
    for hit in hits {
        println!("hit: {:?}", hit);
    }
}
