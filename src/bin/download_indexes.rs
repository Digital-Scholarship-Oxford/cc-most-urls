use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("cc-index.paths") {
        let line_iterator = lines.map_while(Result::ok);

        const APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

        let client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        for download_path in line_iterator {
            if download_path.ends_with("gz") {
                let full_download_path = format!("https://data.commoncrawl.org/{download_path}");

                println!("downloading {full_download_path}");

                let response = client.get(full_download_path).send().unwrap();

                if response.status().is_success() {
                    let mut decoder = MultiGzDecoder::new(response);
                    // create string with around 5GB capacity
                    let mut uncompressed_bytes = Vec::with_capacity(5000000000);
                    std::io::copy(&mut decoder, &mut uncompressed_bytes).unwrap();

                    let lines = String::from_utf8(uncompressed_bytes).expect("Found invalid UTF-8");

                    println!("downloaded and decompressed {} bytes", lines.len());
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}
