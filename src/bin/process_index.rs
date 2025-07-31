use idna::uts46::{self, Uts46};
use nanoserde::DeJson;
use polars::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use url::Url;

fn internationalised_domain_length(parsed_url: &Url) -> usize {
    let url = parsed_url.as_str();

    let i18n_domain_length_difference = match parsed_url.domain() {
        Some(raw_url_domain) => {
            let i18n_domain = Uts46::to_unicode(
                &Uts46::new(),
                raw_url_domain.as_bytes(),
                idna::AsciiDenyList::EMPTY,
                uts46::Hyphens::Allow,
            )
            .0;

            raw_url_domain.chars().count() - i18n_domain.chars().count()
        }
        None => 0,
    };

    let decoded_length_difference: usize = url.chars().count()
        - percent_encoding::percent_decode_str(url)
            .decode_utf8_lossy()
            .chars()
            .count();

    let total_difference: usize = i18n_domain_length_difference + decoded_length_difference;

    url.chars().count() - total_difference
}

fn main() {
    #[derive(DeJson, Debug)]
    struct CDXIndex {
        url: String,
        status: String,
    }

    struct IndexRecordParsed {
        url_column: Vec<String>,
        url_length_column: Vec<u16>,
        i18_url_length_column: Vec<u16>,
        status_column: Vec<u8>,
    }

    let mut index_record_parsed = IndexRecordParsed {
        url_column: Vec::with_capacity(256000000),       // 246MB
        url_length_column: Vec::with_capacity(64000000), // 64MB
        i18_url_length_column: Vec::with_capacity(64000000), // 64MB
        status_column: Vec::with_capacity(12000000),     // 64MB
    };

    // loop up here on the index

    if let Ok(lines) = read_lines("urls.txt") {
        // Consumes the iterator, returns an (Optional) String

        // let mut url_list = Vec::new();

        let mut invalid_urls: usize = 0;

        let line_iterator = lines.map_while(Result::ok).enumerate();

        for line in line_iterator {
            // extract the json object from the cdx(j) line
            let index_json_line: &str = line.1.splitn(3, ' ').nth(2).unwrap();

            // Deserialise json
            let index: CDXIndex = DeJson::deserialize_json(index_json_line).unwrap();

            let parsed_url = match Url::parse(&index.url) {
                Ok(parsed_url) => parsed_url,
                Err(_) => {
                    invalid_urls = invalid_urls.wrapping_add(1);
                    continue;
                }
            };

            let url_length: u16 = index.url.len() as u16;

            let i18_url_length: u16 = internationalised_domain_length(&parsed_url) as u16;

            let status: u8 = index.status.chars().next().unwrap().to_digit(10).unwrap() as u8;

            // push these values to lists
            index_record_parsed.url_column.push(index.url);
            index_record_parsed.url_length_column.push(url_length);
            index_record_parsed
                .i18_url_length_column
                .push(i18_url_length);
            index_record_parsed.status_column.push(status);
        }
    }

    // build the lazyframe
    let initial_lazyframe = DataFrame::new(vec![
        Column::new("url".into(), &index_record_parsed.url_column),
        Column::new(
            "raw_characters".into(),
            &index_record_parsed.url_length_column,
        ),
        Column::new(
            "i18n_characters".into(),
            &index_record_parsed.i18_url_length_column,
        ),
        Column::new("status_code".into(), &index_record_parsed.status_column),
    ])
    .unwrap()
    .lazy()
    .unique(Some(vec!["url".into()]), UniqueKeepStrategy::Any);

    let grouped_frame = initial_lazyframe
        .group_by(["raw_characters"])
        .agg([
            col("raw_characters")
                .filter(col("status_code").eq(lit(1u8)))
                .count()
                .alias("informational"),
            col("raw_characters")
                .filter(col("status_code").eq(lit(2u8)))
                .count()
                .alias("successful"),
            col("raw_characters")
                .filter(col("status_code").eq(lit(3u8)))
                .count()
                .alias("redirection"),
            col("raw_characters")
                .filter(col("status_code").eq(lit(4u8)))
                .count()
                .alias("client_error"),
            col("raw_characters")
                .filter(col("status_code").eq(lit(5u8)))
                .count()
                .alias("server_error"),
            col("raw_characters").count().alias("total"),
        ])
        .sort(["raw_characters"], Default::default())
        .collect()
        .unwrap();

    println!("{grouped_frame}");
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
