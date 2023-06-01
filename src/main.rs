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
        let mut filtered = record.iter().enumerate().filter_map(|(i, field)| {
            if unwanted_idx.contains(&i) {
                None
            } else {
                Some(field.to_string())
            }
        });
        if filtered.all(|field| !field.is_empty()) {
            let final_record: Vec<String> = filtered.collect();
            final_records.push(final_record);
        }
    }
    final_records
}

fn main() {
    // read in csv file
    let file_path = "data/train.csv";
    let file = File::open(file_path).unwrap();
    let reader = csv::Reader::from_reader(file);
    // remove unwanted columns (PassengerId, Name, Ticket, Cabin
    let final_records = clean_data(reader);
    assert_eq!(final_records.len(), 712);
    // convert categorical data into numeric
    // normalize age and prices
    // create random parameters for each column
    // create ones column
    // calculate prediction
    // calculate loss
    // optimize with gradient descent
}
