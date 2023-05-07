fn main() {
    let response =
        reqwest::blocking::get("https://www.rabe-bike.de/de/cube-attain-slx-grey-n-black-2023")
            .unwrap()
            .text()
            .unwrap();

    let document = scraper::Html::parse_document(&response);
    let price_selector = scraper::Selector::parse(
        "div.tw-text-2xl.tw-font-medium.tw-tracking-tight.tw-relative.tw-leading-none",
    )
    .unwrap();
    let prices = document.select(&price_selector).map(|x| x.inner_html());

    let price: Vec<String> = prices
        .filter(|p| {
            println!("{}", p);

            if p.contains("€") {
                return true;
            }
            false
        })
        .collect();

    println!("{:?}", price[0]);
}
