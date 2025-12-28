use polars::prelude::*;

fn main() {
    let csv_schema = Schema::from_iter(vec![
        Field::new("raw_characters".into(), DataType::UInt16),
        Field::new("graphemes".into(), DataType::UInt16),
        Field::new("status_code".into(), DataType::UInt16),
    ]);

    let lazy_frame = LazyCsvReader::new("output.csv")
        .with_has_header(false)
        .with_truncate_ragged_lines(true)
        .with_low_memory(true)
        .with_schema(Some(csv_schema.into()))
        // .with_n_rows(Some(1000000))
        .finish()
        .unwrap();

    // for column in ["raw_characters", "graphemes"] {
    // let grouped_df = lazy_frame
    //     .clone()
    //     .group_by([column])
    //     .agg([
    //         col(column)
    //             .filter(col("status_code").eq(lit(1u8)))
    //             .count()
    //             .alias("informational"),
    //         col(column)
    //             .filter(col("status_code").eq(lit(2u8)))
    //             .count()
    //             .alias("successful"),
    //         col(column)
    //             .filter(col("status_code").eq(lit(3u8)))
    //             .count()
    //             .alias("redirection"),
    //         col(column)
    //             .filter(col("status_code").eq(lit(4u8)))
    //             .count()
    //             .alias("client_error"),
    //         col(column)
    //             .filter(col("status_code").eq(lit(5u8)))
    //             .count()
    //             .alias("server_error"),
    //         col(column).count().alias("total"),
    //     ])
    //     .sort([column], Default::default())
    //     .collect()
    //     .unwrap();
    // println!("{grouped_df}");
    // }

    // for status_code in [1u8, 2u8, 3u8, 4u8, 5u8] {
    //     let mean_average = lazy_frame
    //         .clone()
    //         .filter(col("status_code").eq(status_code))
    //         .mean()
    //         .collect()
    //         .unwrap();
    //     println!("mean average for {status_code}xx");
    //     println!("{mean_average}");
    // }

    // let total_mean_average = lazy_frame
    //     .clone()
    //     .select([col("raw_characters"), col("graphemes")])
    //     .mean()
    //     .collect()
    //     .unwrap();
    // println!("total mean average is");
    // println!("{total_mean_average}");

    // for status_code in [1u8, 2u8, 3u8, 4u8, 5u8] {
    //     let median_average = lazy_frame
    //         .clone()
    //         .filter(col("status_code").eq(status_code))
    //         .median()
    //         .collect()
    //         .unwrap();
    //     println!("median average for {status_code}xx");
    //     println!("{median_average}");
    // }

    // let total_median_average = lazy_frame
    //     .clone()
    //     .select([col("raw_characters"), col("graphemes")])
    //     .median()
    //     .collect()
    //     .unwrap();
    // println!("total median average is");
    // println!("{total_median_average}");

    let quantile = lazy_frame
        .clone()
        .select([col("raw_characters"), col("graphemes")])
        .quantile(lit(0.90), polars::prelude::QuantileMethod::Nearest)
        .collect()
        .unwrap();
    println!("90th quantile is");
    println!("{quantile}");

    let quantile = lazy_frame
        .clone()
        .select([col("raw_characters"), col("graphemes")])
        .quantile(lit(0.80), polars::prelude::QuantileMethod::Nearest)
        .collect()
        .unwrap();
    println!("80th quantile is");
    println!("{quantile}");

    let quantile = lazy_frame
        .clone()
        .select([col("raw_characters"), col("graphemes")])
        .quantile(lit(0.75), polars::prelude::QuantileMethod::Nearest)
        .collect()
        .unwrap();
    println!("75th quantile is");
    println!("{quantile}");
}
