use std::fs::File;
// use std::io::{self, BufRead, BufReader};
use std::path::Path;

use flate2;
use flate2::read::GzDecoder;

use async_compression::futures::bufread::GzipDecoder;
// use futures::{
//     io::{self, BufReader, ErrorKind},
//     prelude::*,
// };

#[tokio::main]
async fn main() {
    if let Ok(lines) = read_lines("cc-index.paths") {
        let line_iterator = lines.map_while(Result::ok);

        const APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        for download_path in line_iterator {
            if download_path.ends_with("gz") {
                let full_download_path = format!("https://data.commoncrawl.org/{download_path}");

                println!("downloading {full_download_path}");

                // download the file, this returns compressed nonsense
                // let body = client
                //     .get(full_download_path)
                //     .send()
                //     .unwrap()
                //     .bytes()
                //     .unwrap();

                let response = reqwest::get("http://localhost:8000/test.txt.gz")
                    .await
                    .unwrap();
                let reader = response
                    .bytes_stream()
                    .map_err(|e| println!("{e}"))
                    .into_async_read();
                let mut decoder = GzipDecoder::new(futures::io::BufReader::new(reader));
                let mut data = String::new();
                decoder.read_to_string(&mut data).await.unwrap();
                println!("{data:?}");

                // let mut decoder = GzDecoder::new(BufReader::new(body));

                // now, decode the bytes in the response
                // let reader = GzDecoder::new(body);

                // for line in body.lines() {
                //     println!("{line}");
                // }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
