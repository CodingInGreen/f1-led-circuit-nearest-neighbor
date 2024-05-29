use csv::ReaderBuilder;
use csv::Writer;
use std::error::Error;
use std::fs::File;

// Struct to hold the coordinates and designator from the second dataset
struct Neighbor {
    x: f64,
    y: f64,
    designator: String,
}

// Function to calculate the Euclidean distance between two points
fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn main() -> Result<(), Box<dyn Error>> {
    let first_dataset_path = "zandvoort_data_tsunoda.csv";
    let second_dataset_path = "zandvoort_led_coordinates_normalized_check_for_inversion.csv";
    let output_path = "led_designator_labeled_zandvoort_data_tsunoda.csv";

    // Read the second dataset
    let second_dataset_file = File::open(second_dataset_path)?;
    let mut second_dataset_reader = ReaderBuilder::new().from_reader(second_dataset_file);
    let mut neighbors = Vec::new();
    for result in second_dataset_reader.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        let designator = record[2].to_string();
        neighbors.push(Neighbor { x, y, designator });
    }

    // Read the first dataset
    let first_dataset_file = File::open(first_dataset_path)?;
    let mut first_dataset_reader = ReaderBuilder::new().from_reader(first_dataset_file);
    let headers = first_dataset_reader.headers()?.clone();
    let mut records: Vec<_> = first_dataset_reader.records().collect::<Result<_, _>>()?;

    // Add the new column header
    let mut output_headers = headers.clone();
    output_headers.push_field("designator");

    // Create the output writer
    let output_file = File::create(output_path)?;
    let mut writer = Writer::from_writer(output_file);

    // Write the headers to the output file
    writer.write_record(&output_headers)?;

    // Find the nearest neighbor for each record in the first dataset and write to the output file
    for record in records.iter_mut() {
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        
        let nearest_neighbor = neighbors.iter()
            .min_by(|a, b| {
                euclidean_distance(x, y, a.x, a.y)
                    .partial_cmp(&euclidean_distance(x, y, b.x, b.y))
                    .unwrap()
            })
            .unwrap();
        
        record.push_field(&nearest_neighbor.designator);
        writer.write_record(record.iter())?;
    }

    writer.flush()?;
    Ok(())
}
