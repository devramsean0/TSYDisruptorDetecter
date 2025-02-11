use std::ops::Deref;
use scraper::selectable::Selectable;

struct PartialDisruption {
    title: String,
    dates: String,
}
struct Disruption {
    title: String,
    posted_on: String,
    dates: String,
    reason: String,
    description: String,
    impact: String,
    details: Vec<String>,
}

fn do_fetch() {
    let mut disruptions: Vec<Disruption> = vec![];
    let mut partial_disruptions: Vec<PartialDisruption> = vec![];
    let client = reqwest::blocking::Client::new();
    let response = client
            .get("https://www.travelsouthyorkshire.com/en-gb/newsupdates/Disruptions")
            .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:135.0) Gecko/20100101 Firefox/135.0")
            .header(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            .header(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=0.5")
            .header(reqwest::header::CACHE_CONTROL, "no-cache")
            .header(reqwest::header::CONNECTION, "keep-alive")
            .header(reqwest::header::COOKIE, "uzmx=7f9000ec0197af-a413-403e-8a46-87d8d16a28041-17392639059892751748-59ddb4064557df6052; __uzma=aadeb7e7-6d54-4f28-855b-48dbd664fd46; __uzmb=1739263906; __uzme=2815; __uzmc=988904910196; __uzmd=1739266657; __uzmf=7f6000a669aaa6-3ffd-476e-b13d-f043a42f025417392639063972751340-88588644f271c12b49; __ssds=2; __ssuzjsr2=a9be0cd8e; __uzmaj2=aadeb7e7-6d54-4f28-855b-48dbd664fd46; __uzmbj2=1739263907; __uzmcj2=644332589255; __uzmdj2=1739266655; __uzmlj2=4ds/SfBPmlHV9C+OWsp3LyFv2ILaPEHcZKr5WHh/FuY=; __uzmfj2=7f6000a669aaa6-3ffd-476e-b13d-f043a42f025417392639072862748237-ec00b821dddf400725; uzmxj=7f9000ec0197af-a413-403e-8a46-87d8d16a28041-17392639072862748237-ee3731fc1188c62325")
            .header(reqwest::header::HOST, "www.travelsouthyorkshire.com")
            .header(reqwest::header::PRAGMA, "no-cache")
            .header(reqwest::header::UPGRADE_INSECURE_REQUESTS, "1")
            .send();
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    // Needed Selectors
    let disruption_title_container_selector =
        scraper::Selector::parse("div.col-7.disruptionsDiv").unwrap();
    let disruption_data_container_selector =
        scraper::Selector::parse("div.disruptionsCollapse").unwrap();
    let disruption_data_details_selector = scraper::Selector::parse("tr.consCollapse td").unwrap();
    // Do the title parse
    let disruption_containers = document.select(&disruption_title_container_selector);
    for disruption in disruption_containers {
        let title_element = disruption
            .select(&scraper::Selector::parse("div.font-weight-bold, p").unwrap())
            .next()
            .map(|p| p.text().collect::<String>());
        let title = title_element.unwrap_or_default();
        let split_title = title.trim().split("-").collect::<Vec<_>>()[0].replace("\u{a0}", "");
        let split_dates = title.trim().split("-").collect::<Vec<_>>()[1].replace("\n", "").replace(" ", "").replace("to", " to ").replace(";", ", ");
        println!("Disruption found: {}: {}", split_title, split_dates);
        partial_disruptions.push(PartialDisruption {
            title: split_title,
            dates: split_dates
        })
    }
    // Do the data parse
    let mut count = 0;
    let disruption_containers = document.select(&disruption_data_container_selector);
    for disruption in disruption_containers {
        // Posted On
        let posted_on_element = disruption
            .select(&scraper::Selector::parse("p.m-2.float-right").unwrap())
            .next()
            .map(|p| p.text().collect::<String>());
        let posted_on = posted_on_element.unwrap_or_default();
        let concat_posted_on = posted_on.trim().replace("Posted: ", "");
        // Reason
        let reason_element = disruption
            .select(&scraper::Selector::parse("table.table tbody tr:nth-child(2)").unwrap())
            .next()
            .map(|p| p.text().collect::<String>());
        let reason = reason_element.unwrap_or_default();
        let concat_reason = reason.replace("\n", "").trim().replace("Reason:", "");
        // description
        let description_element = disruption
            .select(&scraper::Selector::parse("table.table tbody tr:nth-child(3)").unwrap())
            .next()
            .map(|p| p.text().collect::<String>());
        let description = description_element.unwrap_or_default();
        let concat_description = description.trim().replace("Description: ", "");
        // Impact
        let impact_element = disruption
            .select(&scraper::Selector::parse("table.table tbody tr:nth-child(4)").unwrap())
            .next()
            .map(|p| p.text().collect::<String>());
        let impact = impact_element.unwrap_or_default();
        let concat_impact = impact.trim().replace("Impact: ", "");
        // Details
        let mut details_blocks: Vec<String> = vec![];
        let details_elements = disruption.select(&disruption_data_details_selector);
        for detail_div in details_elements {
            let detail_element = detail_div
                .select(&scraper::Selector::parse("pre").unwrap())
                .next()
                .map(|pre| pre.text().collect::<String>());
            let detail = detail_element.unwrap_or_default();
            let concat_detail = detail.trim().to_string();
            details_blocks.push(concat_detail)
        }
        // Get PartialDisruption
        let partial = partial_disruptions.get(count).unwrap().deref();
        disruptions.push(Disruption {
            title: partial.title,
            dates: partial.dates,
            posted_on: concat_posted_on,
            reason: concat_reason,
            description: concat_description,
            impact: concat_impact,
            details: details_blocks
        });
        println!("Disruption Information found");
    }
}

fn main() {
    do_fetch()
}
