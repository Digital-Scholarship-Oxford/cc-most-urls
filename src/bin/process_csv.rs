use polars::prelude::*;
use std::fs::File;

fn main() {
    let csv_schema = Schema::from_iter(vec![
        Field::new("raw_characters".into(), DataType::UInt16),
        Field::new("i18n_characters".into(), DataType::UInt16),
        Field::new("status_code".into(), DataType::UInt8),
    ]);

    let lazy_frame = LazyCsvReader::new("values.csv")
        .with_has_header(false)
        .with_schema(Some(csv_schema.into()))
        .finish()
        .unwrap();

    let mut grouped_df = lazy_frame
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

    let mut file = File::create("frequency.csv").expect("could not create file");

    CsvWriter::new(&mut file)
        .finish(&mut grouped_df)
        .expect("could not write to file");

    println!("{grouped_df:?}");
}

// fn process_records(index_record_parsed: &IndexRecordParsed) {
//     // build the lazyframe
//     let initial_lazyframe = DataFrame::new(vec![
//         Column::new("url".into(), &index_record_parsed.url_column),
//         Column::new(
//             "raw_characters".into(),
//             &index_record_parsed.url_length_column,
//         ),
//         Column::new(
//             "i18n_characters".into(),
//             &index_record_parsed.i18_url_length_column,
//         ),
//         Column::new("status_code".into(), &index_record_parsed.status_column),
//     ])
//     .unwrap()
//     .lazy();
//     // .unique(Some(vec!["url".into()]), UniqueKeepStrategy::Any);

//     fn group_lazyframe(
//         lazyframe: LazyFrame,
//         column: &str,
//     ) -> Result<polars::prelude::DataFrame, PolarsError> {
//         lazyframe
//             .group_by([column])
//             .agg([
//                 col(column)
//                     .filter(col(column).eq(lit(1u8)))
//                     .count()
//                     .alias("informational"),
//                 col(column)
//                     .filter(col("status_code").eq(lit(2u8)))
//                     .count()
//                     .alias("successful"),
//                 col(column)
//                     .filter(col("status_code").eq(lit(3u8)))
//                     .count()
//                     .alias("redirection"),
//                 col(column)
//                     .filter(col("status_code").eq(lit(4u8)))
//                     .count()
//                     .alias("client_error"),
//                 col(column)
//                     .filter(col("status_code").eq(lit(5u8)))
//                     .count()
//                     .alias("server_error"),
//                 col(column).count().alias("total"),
//             ])
//             .sort([column], SortMultipleOptions::default())
//             .collect()
//     }

//     let mut raw_chars_grouped =
//         group_lazyframe(initial_lazyframe.clone(), "raw_characters").unwrap();
//     let mut raw_chars_grouped_csv =
//         File::create("raw_chars_grouped.csv").expect("could not create file");

//     println!("Raw characters table {raw_chars_grouped}");

//     CsvWriter::new(&mut raw_chars_grouped_csv)
//         .include_header(true)
//         .with_separator(b',')
//         .finish(&mut raw_chars_grouped)
//         .unwrap();

//     let mut i18n_chars_grouped =
//         group_lazyframe(initial_lazyframe.clone(), "i18n_characters").unwrap();
//     let mut i18n_chars_grouped_csv =
//         File::create("i18n_chars_grouped.csv").expect("could not create file");

//     println!("Internationalised characters table {i18n_chars_grouped}");

//     CsvWriter::new(&mut i18n_chars_grouped_csv)
//         .include_header(true)
//         .with_separator(b',')
//         .finish(&mut i18n_chars_grouped)
//         .unwrap();
// }
