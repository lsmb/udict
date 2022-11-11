use scraper::{Html, Selector};
use std::env;
use std::process;
use termion::{color, style};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut definition_count: usize = 1;
    let term;
    
    if args.len()<2||args[1].to_string() == "--help" {
        println!("Usage: udict [number of terms] <term>");
        process::exit(1);
    } else if args[1].parse::<u16>().is_ok() {
        definition_count = args[1].parse().unwrap();
        term = args[2..].join(" ");
    } else {
        term = args[1..].join(" ");
    }

    let resp = reqwest::get(format!(
        "https://www.urbandictionary.com/define.php?term={}",
        term
    ))
    .await?
    .text()
    .await?;

    let body = Html::parse_document(&resp);

    let suggestions_sel = Selector::parse(".suggestions").unwrap();
    let definition_sel = Selector::parse(".definition").unwrap();
    let word_sel = Selector::parse(".word").unwrap();
    let meaning_sel = Selector::parse(".meaning").unwrap();
    let examples_sel = Selector::parse(".example").unwrap();
    let contributor_sel = Selector::parse(".contributor").unwrap();

    for element in body.select(&suggestions_sel) {
        let element_text: String = element.text().collect();
        if element_text.contains("Sorry, we couldn't find") {
            println!("Unforunately this definition doesn't exist.");
            process::exit(1);
        }
    }

    for (i, element) in body
        .select(&definition_sel)
        .take(definition_count)
        .enumerate()
    {
        let word = element.select(&word_sel).next().unwrap();
        let word_string: String = word.text().collect();
        println!(
            "{}{}{} {} \n{}",
            style::Bold,
            color::Bg(color::Cyan),
            color::Fg(color::Black),
            word_string,
            style::Reset
        );

        let meaning = element.select(&meaning_sel).next().unwrap();
        let meaning_string: String = meaning.text().collect();
        let wrapped_meaning = textwrap::wrap(&meaning_string, 64);

        for line in wrapped_meaning {
            println!(" │ {}", line)
        }

        println!("");

        let examples = element.select(&examples_sel).next().unwrap();
        let examples_string: String = examples.text().collect();
        let examples_vec: Vec<&str> = examples_string.split("\n").collect();

        for example in examples_vec {
            println!(" - {}{}", style::Italic, textwrap::fill(example, 60))
        }

        let contributor = element.select(&contributor_sel).next().unwrap();
        let contributor_string: String = contributor.text().collect();
        println!("\n{}{}{}", style::Bold, contributor_string, style::Reset);

        if i + 1 > 0 && i < definition_count - 1 {
            println!()
        }
    }

    Ok(())
}
