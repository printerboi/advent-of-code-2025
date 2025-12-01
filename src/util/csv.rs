use csv::ReaderBuilder;


pub fn parse_csv_file(name: String, storage: &mut Vec<Vec<String>>) {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(name).expect("failed to open data.csv");

    for result in rdr.records() {
        let record = result.expect("bad CSV row");
        let fields: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        storage.push(fields);
    }
}