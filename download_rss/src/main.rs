#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{

    let url = match std::env::args().nth(1) {
        Some(param)  => param,
        None => panic!("Usage :\r\n\tapplication https://www.examples.com/rss /tmp/example.xml"),
    };

    let path = match std::env::args().nth(2) {
        Some(param)  => param,
        None => panic!("Usage :\r\n\tapplication https://www.examples.com/rss /tmp/example.xml"),
    };

    let parser = future_rss::RssParser::from_url(url.as_str(),"utf8").await?;
    let xml = parser.get_xml();
    std::fs::write(path.as_str(),xml.as_str())?;

    Ok(())
}
