use idna::uts46::{self, Uts46};
use nanoserde::DeJson;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;
use url::Url;

use flate2::read::MultiGzDecoder;

#[derive(DeJson, Debug)]
struct CDXIndex {
    url: String,
    status: String,
}

struct IndexRecordParsed {
    url: String,
    url_length: u16,
    i18_url_length: u16,
    status: u8,
}

fn main() {
    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    let mut invalid_urls: usize = 0;

    if let Ok(lines) = read_lines("cc-index.paths") {
        let line_iterator = lines.map_while(Result::ok);

        const APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

        let client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .timeout(None)
            .build()
            .unwrap();

        for download_path in line_iterator {
            let mut parsed_index_record_list: Vec<IndexRecordParsed> =
                Vec::with_capacity(256_000_000);
            if download_path.ends_with("gz") {
                let full_download_path = format!("https://data.commoncrawl.org/{download_path}");

                println!("downloading {full_download_path}");

                let response = client.get(full_download_path).send().unwrap();

                if response.status().is_success() {
                    let compressed_bytes = response.bytes().unwrap();

                    let mut decoder = MultiGzDecoder::new(&compressed_bytes[..]);
                    // create string with around 5GB capacity
                    let mut uncompressed_bytes = Vec::with_capacity(500_0000_000);

                    std::io::copy(&mut decoder, &mut uncompressed_bytes).unwrap();

                    let uncompressed_string =
                        String::from_utf8(uncompressed_bytes).expect("Found invalid UTF-8");

                    println!(
                        "downloaded and decompressed {} bytes",
                        uncompressed_string.len()
                    );

                    for line in uncompressed_string.lines().enumerate() {
                        // extract the json object from the cdx(j) line
                        let index_json_line: &str = line.1.splitn(3, ' ').nth(2).unwrap();

                        // Deserialise json
                        let index: CDXIndex = DeJson::deserialize_json(index_json_line).unwrap();

                        if let Ok(parsed_url) = Url::parse(&index.url) {
                            let url_length = index.url.len() as u16;

                            let i18_url_length =
                                internationalised_domain_length(&parsed_url) as u16;

                            let status =
                                index.status.chars().next().unwrap().to_digit(10).unwrap() as u8;

                            let parsed_index = IndexRecordParsed {
                                url: index.url,
                                url_length,
                                i18_url_length,
                                status,
                            };

                            parsed_index_record_list.push(parsed_index);
                        } else {
                            // if the url is invalid, skip processing and
                            // increment the invalid_urls list
                            invalid_urls = invalid_urls.wrapping_add(1);
                        }
                    }
                }
            }

            println!("deduplicating urls");
            parsed_index_record_list.dedup_by(|a, b| a.url == b.url);

            let string_list: String = parsed_index_record_list
                .iter()
                .map(|x| format!("{},{},{}", x.url_length, x.i18_url_length, x.status))
                .collect::<Vec<String>>()
                .join("\n");
            
            println!("writing to file");

            // at this point, append index_records_parsed to a file, the code will break here if the file is not already there!
            let mut big_csv = OpenOptions::new().append(true).open("values.csv").unwrap();
            let string_list_bytes = string_list.into_bytes();
            big_csv.write_all(&string_list_bytes).unwrap();

            println!("done!\nmoving to the next file");
        }

        // process_records(&index_record_parsed);

        println!("{invalid_urls} urls were invalid");
    }
}

fn internationalised_domain_length(parsed_url: &Url) -> usize {
    let i18n_domain_length_difference = match parsed_url.domain() {
        Some(raw_url_domain) => {
            let i18n_domain = Uts46::to_unicode(
                &Uts46::new(),
                raw_url_domain.as_bytes(),
                idna::AsciiDenyList::EMPTY,
                uts46::Hyphens::Allow,
            )
            .0;

            raw_url_domain.graphemes(true).count() - i18n_domain.graphemes(true).count()
        }
        None => 0,
    };

    let decoded_length_difference = {
        let raw_url_length = parsed_url.as_str().graphemes(true).count();
        let decoded_url_length = percent_encoding::percent_decode_str(parsed_url.as_str())
            .decode_utf8_lossy()
            .graphemes(true)
            .count();

        raw_url_length - decoded_url_length
    };

    let total_difference: usize = i18n_domain_length_difference + decoded_length_difference;

    parsed_url.as_str().graphemes(true).count() - total_difference
}
