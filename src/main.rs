use csv;
use mail_parser::*;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs, io,
    path::PathBuf,
    process::exit,
};

#[derive(Debug)]
struct Price {
    date: String,
    price: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();

    if args.len() <= 1 {
        println!(
            "Usage: {} <filename> [filename, ...]",
            env::current_exe()?.display()
        );
        exit(1);
    }

    let mut prices: HashMap<String, Vec<Price>> = HashMap::new();

    for arg in args.skip(1) {
        let contents = fs::read_to_string(PathBuf::from(arg))?;
        for message_contents in contents
            .split("From ")
            .filter(|s| !s.is_empty())
            .map(|s| "From ".to_owned() + s)
        {
            let input = message_contents.into_bytes();
            let message = Message::parse(&input).unwrap();
            let html = message.body_html(0).unwrap();

            let mut started_table: bool = false;
            let mut started_row: bool = false;
            let mut tds: Vec<String> = Vec::new();
            for line in html.lines() {
                if line.contains("Boodschappen") {
                    started_table = true;
                }

                if started_table {
                    if line.contains("</table>") {
                        started_table = false;
                    }

                    if line.contains("<tr>") {
                        started_row = true;
                        tds = Vec::new();
                    }

                    if started_row {
                        if line.contains("</tr>") {
                            started_row = false;
                            if tds.len() == 4 {
                                let name = extract_content(&tds[0]);
                                // let amount = extract_content(&tds[1]);
                                let cost_per_piece = extract_content(&tds[2]);
                                // let cost_total = extract_content(&tds[3]);
                                if cost_per_piece != "&nbsp;" && cost_per_piece != "gew." {
                                    prices.entry(name.to_owned()).or_default().push(Price {
                                        date: message.date().unwrap().to_rfc3339(),
                                        price: cost_per_piece.to_string(),
                                    })
                                }
                            }
                        }

                        if line.trim().starts_with("<td") {
                            tds.push(line.to_string());
                        }
                    }
                }
            }
        }
    }

    for v in prices.values_mut() {
        v.sort_by_key(|p| p.date.clone());
    }

    let mut writer = csv::Writer::from_writer(io::stdout());
    for (product, prices) in prices {
        for price in &prices {
            writer.write_record(&[&product, &price.date, &price.price])?;
        }
    }

    writer.flush()?;

    Ok(())
}

fn extract_content(s: &str) -> &str {
    s.split(">").nth(1).unwrap().split("</td").nth(0).unwrap()
}
