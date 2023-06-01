use std::fs::File;

fn main() {
    // read in csv file
    let file_path = "data/train.csv";
    let file = File::open(file_path).unwrap();
    let mut reader = csv::Reader::from_reader(file);
    // read in data, filtering out rows with empty data
    for (i, row) in reader.records().enumerate() {
        let record = row.unwrap();
        if record.iter().any(|field| field == "") {
            println!("record with empty field {:?}", record);
        } else {
            println!("{:?}", record);
        }
        if i > 5 {
            break;
        }
    }
    // remove unwanted columns (PassengerId, Name, Ticket, Cabin
    // convert categorical data into numeric
    // normalize age and prices
    // create random parameters for each column
    // create ones column
    // calculate prediction
    // calculate loss
    // optimize with gradient descent
}
