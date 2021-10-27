use clap::{App, Arg};

const MAXPOOL_SIZE: usize = 2;

fn main() {
    let matches = App::new("cnnBlast - a rust cnn accelerator")
        .version("1.0")
        .author("Sebastian Hellgren")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true),
        )
        .get_matches();

    if let Some(input_file) = matches.value_of("input") {
        println!("Value for input: {}", input_file);

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(input_file)
            .unwrap(); // TODO: some error checking here

        // TODO: Check that input count is evenly divisible of maxpool
        // TODO: Add row length as input
        // TODO: Use zip to iterate over two vectors at the same time

        let result = reader.records();

        /*
         * result is a vector of Result<StringRecord, E>
         * unwrap: Result -> StringRecord
         * as_slice: StringRecord -> str
         * parse: str -> Result<i32, E>
         * unwrap: Result -> i32
         * collect: iter<i32> -> Vec<i32>
         */
        let numbers: Vec<i32> = result
            .map(|s| s.unwrap().as_slice().parse::<i32>().unwrap())
            .collect();

        /*
         * Cut the vector into MAXPOOL_SIZE large windows 
         * check max of each chunk
         * 
         */
        let maxpooled: Vec<i32> = numbers
            .chunks(MAXPOOL_SIZE)
            .map(|chunk| chunk.iter().max().unwrap()).cloned().collect();

        for item in maxpooled {
            println!("{}", item);
        }
    }
}
