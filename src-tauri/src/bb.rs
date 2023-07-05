use polars::prelude::*;
use std::fs::File;

pub fn bb_csv(df: DataFrame, output_path: &str) -> () {
    let step1: DataFrame = df
        .clone()
        .lazy()
        .filter(
            col("name")
                .str()
                .starts_with(lit("LVLQ 90"))
                .or(col("name").str().starts_with(lit("LVLQ 100")))
                .or(col("name").str().starts_with(lit("LVLQ 144")))
                .or(col("productcode").eq(lit("CE-000135-0006")))
                .or(col("productcode").eq(lit("CE-008805-0001")))
                .or(col("productcode").eq(lit("CE-000546-0003")))
                .or(col("productcode").eq(lit("CE-001130-0003"))),
        )
        .select(&[
            col("materiaal").alias("Materiaal BB"),
            col("productcode").alias("Productcode"),
            col("name").alias("OnderdeelNaam"),
            col("dikte").cast(DataType::Float64).alias("Dikte BB"),
            col("aantal").cast(DataType::Float64).alias("Aantal"),
            col("modulenaam").alias("Modulenaam"),
            col("station").alias("Station"),
        ])
        .collect()
        .unwrap();

    let step2 = step1
        .clone()
        .lazy()
        .groupby([
            col("Materiaal BB"),
            col("Productcode"),
            col("OnderdeelNaam"),
            col("Dikte BB"),
            col("Modulenaam"),
            col("Station"),
        ])
        .agg(vec![col("Aantal").sum().alias("Aantal BB")])
        .collect()
        .unwrap();

    let height = step2.height();
    let inkoopnr = Series::new("InkooporderNr", vec!["IO-120252"; height]);
    let cnc: Series = Series::new("CNC Bewerking", vec![""; height]);
    let nesting: Series = Series::new("Nesting Prioriteit", vec![""; height]);

    let mut step3 = step2
        .clone()
        .lazy()
        .with_columns([
            inkoopnr.lit(),
            cnc.lit(),
            nesting.lit(),
            col("Aantal BB"),
        ])
        .select([
            col("Materiaal BB"),
            col("Productcode"),
            col("OnderdeelNaam"),
            col("Dikte BB").cast(DataType::Int64),
            col("Aantal BB").cast(DataType::Int64),
            col("CNC Bewerking"),
            col("Modulenaam"),
            col("InkooporderNr"),
            col("Nesting Prioriteit"),
            col("Station"),
        ])
        .sort("OnderdeelNaam", Default::default())
        .collect()
        .unwrap();

    println!("{:?}", step3);
    let output_str = format!("{}BB.csv", output_path);
    let mut output_file = File::create(output_str).unwrap();
    CsvWriter::new(&mut output_file)
        .finish(&mut step3)
        .unwrap();
}