mod json;
mod parser;

const OBJECT_TEST_FILE_PATH: &str =
    "/Users/djprice/Code/rust_json_parser/src/data/object_test.json";
const ARRAY_TEST_FILE_PATH: &str = "/Users/djprice/Code/rust_json_parser/src/data/array_test.json";

fn main() -> std::io::Result<()> {
    let json_object = json::parse_from_file(OBJECT_TEST_FILE_PATH)?;
    println!(
        "======== BEGIN OBJECT ========\n\n{}\n\n ======== END OBJECT ========\n",
        json_object
    );

    let json_array = json::parse_from_file(ARRAY_TEST_FILE_PATH)?;
    println!(
        "======== BEGIN ARRAY ========\n\n{}\n\n ======== END ARRAY ========\n",
        json_array
    );

    return Ok(());
}
