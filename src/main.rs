use ndarray::Array2;
use std::fs::File;

fn clean_data(mut reader: csv::Reader<File>) -> Vec<Vec<String>> {
    // remove unwanted columns (PassengerId, Name, Ticket, Cabin
    let headers = reader.headers().unwrap();
    let mut unwanted_idx = Vec::new();
    for (i, h) in headers.iter().enumerate() {
        match h {
            "PassengerId" => unwanted_idx.push(i),
            "Name" => unwanted_idx.push(i),
            "Ticket" => unwanted_idx.push(i),
            "Cabin" => unwanted_idx.push(i),
            _ => continue,
        }
    }
    let mut final_records: Vec<Vec<String>> = Vec::new();
    // read in data, filtering out rows with empty data
    for row in reader.records() {
        let record = row.unwrap();
        let filtered = record.iter().enumerate().filter_map(|(i, field)| {
            // remove unwanted columns
            if unwanted_idx.contains(&i) {
                None
            } else {
                Some(field.to_string())
            }
        });
        // remove entries with an empty field
        if filtered.clone().any(|field| field.is_empty()) {
            println!("Empty field");
            continue;
        } else {
            let temp_record: Vec<String> = filtered.collect();
            println!("temp_record : {:?}", temp_record);
            let pclass_record = encode_pclass(temp_record);
            println!("encoding pclass {:?}", pclass_record);
            let final_record = pclass_record;
            final_records.push(final_record);
        }
    }
    final_records
}
fn encode_pclass(record: Vec<String>) -> Vec<String> {
    let mut pclass_record = vec![];
    match record[1].as_str() {
        "1" => pclass_record = [record.as_slice(), &["1".to_string(), "0".to_string()]].concat(),
        "2" => pclass_record = [record.as_slice(), &["0".to_string(), "1".to_string()]].concat(),
        "3" => pclass_record = [record.as_slice(), &["0".to_string(), "0".to_string()]].concat(),
        _ => pclass_record = [record.as_slice(), &["0".to_string(), "0".to_string()]].concat(),
    }
    // remove the pclass
    pclass_record.remove(1);
    pclass_record
}

fn main() {
    // read in csv file
    let file_path = "data/train.csv";
    let file = File::open(file_path).unwrap();
    let reader = csv::Reader::from_reader(file);
    // remove unwanted columns (PassengerId, Name, Ticket, Cabin
    let final_records = clean_data(reader);
    assert_eq!(final_records.len(), 712);
    // convert categorical data into numeric and strings to numbers and put into ndarray
    // normalize age and prices
    // create random parameters for each column
    // create ones column
    // calculate prediction
    // calculate loss
    // optimize with gradient descent
}
