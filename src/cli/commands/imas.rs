use crate::commands::prelude::*;

use pastel::structs::Response;
use pastel::Color;
use std::u8;
use url::form_urlencoded;

pub struct ImasCommand;

impl GenericCommand for ImasCommand {
  fn run(&self, out: &mut Output, matches: &ArgMatches, config: &Config) -> Result<()> {
    let idol_name = matches.value_of("color").expect("required idol name");
    let query = format!(
      r"PREFIX imas: <https://sparql.crssnky.xyz/imasrdf/URIs/imas-schema.ttl#>
PREFIX imasrdf: <https://sparql.crssnky.xyz/imasrdf/RDFs/detail/>

SELECT distinct ?c 
WHERE {{
  imasrdf:{} imas:Color ?c
}}",
      idol_name
    );
    let encoded_query = form_urlencoded::Serializer::new(String::new())
      .append_pair("output", "json")
      .append_pair("force-accept", "text/plain")
      .append_pair("query", &query)
      .finish();
    let sparql_url = format!(
      "https://sparql.crssnky.xyz/spql/imas/query?{}",
      encoded_query
    );

    let res = ureq::get(&sparql_url).call();
    if res.ok() {
      let json_str = res.into_string().unwrap();
      let res_json: Response = serde_json::from_str(&json_str).unwrap();
      let json = res_json.results.bindings;
      if json.len() > 0 {
        for data in json {
          let color_string = &*(data.c.value);
          if color_string.len() >= 6 {
            let red: u8 = u8::from_str_radix(&color_string[0..2], 16).unwrap();
            let green: u8 = u8::from_str_radix(&color_string[2..4], 16).unwrap();
            let blue: u8 = u8::from_str_radix(&color_string[4..6], 16).unwrap();
            let color = Color::from_rgb(red, green, blue);
            out.show_color(&config, &color)?;
          } else {
            println!("Invalid color: {}, {}", idol_name, data.c.value);
          }
        }
      } else {
        println!("No match idol name: {}", idol_name);
      }
    } else {
      println!("Error in requesting to im@sparql...");
    }

    Ok(())
  }
}
