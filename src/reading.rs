use std::fs::File;

pub fn read_large_file() {
    //Open the file
    let file_ptr =
        File::open("large-file.json").expect("oh shit something went wrong opening that");

    let _json: serde_json::Value =
        serde_json::from_reader(file_ptr).expect("improperly formatted json file...");

    println!("Loaded file successfully");
}
