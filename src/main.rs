mod utils;
use utils::{read_remote_data, Data};

fn main() -> Result<(), reqwest::Error> {
    let data: Data = match read_remote_data() {
        Ok(data) => data,
        Err(err) => panic!(),
    };

    // let input_file = File::open("data.json.br").unwrap();
    // let mut input_buf = BufReader::new(input_file);

    // let mut output_buf = Vec::new();

    // let mut brotli_decoder = Decompressor::new(&mut input_buf, 4096);
    // brotli_decoder.read_to_end(&mut output_buf).unwrap();

    // // The decompressed data is now in `output_buf`
    // let decompressed_data = String::from_utf8(output_buf).unwrap();
    // let data: Data = serde_json::from_str(&decompressed_data).unwrap();

    // println!("{}", data.rss.len());

    Ok(())
}
