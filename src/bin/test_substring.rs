fn main() {
    let download_path = "cc-index/collections/CC-MAIN-2025-18/indexes/cdx-00259.gz";
    let truncated_path = download_path[49..54].to_owned() + ".csv";
    println!("{truncated_path}");
}