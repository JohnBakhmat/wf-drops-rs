use scraper::{Selector,Html, ElementRef};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.warframe.com/droptables";

    let site_text = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&site_text);
    
    let title_selector = Selector::parse("h3").unwrap();
    let mut titles = document
        .select(&title_selector)
        .map(|x| x.inner_html())
        .collect::<Vec<String>>();

    titles.drain(0..2); 
   
    let titles = titles.iter()
        .map(|t| t[..t.len()-1].to_string())
        .collect::<Vec<String>>();



    let table_selector = Selector::parse("h3 + table").unwrap();
    let tables = document.select(&table_selector).collect::<Vec<ElementRef>>();


    let missions = parse_table(tables[0]);
    println!("{}",missions);


    Ok(())
}


fn parse_table(table:ElementRef)->String{

    let header_selector = Selector::parse("th").unwrap();
    let headers = table
        .select(&header_selector)
        .map(|x| x.inner_html())
        .collect::<Vec<String>>();

    let mission = headers.first().unwrap();

    println!("Misson: {}", mission);

    mission.to_string()
}
