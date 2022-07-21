use clap::{Arg, App};


fn main() {
  // Basic app information
  let app = App::new("deluge-email-generator")
        .version("1.0")
        .about("Translates handlebars to deluge")
        .author("Tyler Louwerse");

  // Define the name command line option
  let name_option = Arg::with_name("name")
        .long("name")
        .takes_value(true)
        .help("Who to say hello to")
        .required(true);

  // now add in the argument we want to parse
  let app = app.arg(name_option);

  // extract the matches
  let matches = app.get_matches();

  // extract the actual name
  let name = matches.value_of("name")
        .expect("This can't be None, we said it was required!");
  
  println!("Hello, {}", name);
}
