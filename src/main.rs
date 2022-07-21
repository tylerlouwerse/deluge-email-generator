use clap::{Arg, App};
use std::fs;


fn main() {
  // basic app information
  let app = App::new("deluge-email-generator")
        .version("1.0")
        .about("Translates handlebars to deluge")
        .author("Tyler Louwerse");

  // initialize src option
  let src_option = Arg::with_name("src")
        .long("src")
        .takes_value(true)
        .help("File path to the handlebars file to be parsed")
        .required(true);


  // now add in the argument we want to parse
  let app = app.arg(src_option);

  // extract the matches
  let matches = app.get_matches();

  // extract the actual name
  let src_path = matches.value_of("src")
        .expect("This can't be None, we said it was required!");

  // read file
  let contents = fs::read_to_string(src_path)
        .expect("Something went wrong reading the file!");

  println!("File contents, \n{}", contents);
}
