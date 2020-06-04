#[allow(unused_imports)]
use log::{debug,info,error,trace};

static DEFAULT_CHARSET:&str = "utf8";

async fn callback()->Result<(),Box<dyn std::error::Error>>{
    let address = dotenv::var("rss_url")?;
    let charset = dotenv::var("rss_charset").unwrap_or(DEFAULT_CHARSET.into());

    let mut parser = future_rss::RssParser::new();
    parser.title_tag = dotenv::var("rss_title_tag").unwrap_or(future_rss::RSS_DEFAULT_TITLE_TAG.into());
    parser.link_tag = dotenv::var("rss_link_tag").unwrap_or(future_rss::RSS_DEFAULT_LINK_TAG.into());
    parser.author_tag = dotenv::var("rss_author_tag").unwrap_or(future_rss::RSS_DEFAULT_AUTHOR_TAG.into());
    parser.description_tag = dotenv::var("rss_description_tag").unwrap_or(future_rss::RSS_DEFAULT_DESC_TAG.into());
    parser.guid_tag = dotenv::var("rss_guid_tag").unwrap_or(future_rss::RSS_DEFAULT_GUID_TAG.into());
    parser.publish_tag = dotenv::var("rss_publish_tag").unwrap_or(future_rss::RSS_DEFAULT_PUBLISH_TAG.into());

    let xml = parser.request_xml(address.as_str(),charset.as_str()).await?;
    parser.set_xml(xml);

    let datetime = chrono::Local::now();
    let now = datetime.format("%Y_%m_%d").to_string();
    if dotenv::var("rss_output_format")?.eq_ignore_ascii_case("json") {
        let data = parser.parse_json().await?;
        let filename = format!("{}{}_{}.json",
                               dotenv::var("rss_save_path")?,
                               dotenv::var("rss_save_prefix")?,
                               now,
        );
        std::fs::write(filename,data)?;
    }else{
        let data = parser.get_xml();
        let filename = format!("{}{}_{}.xml",
                               dotenv::var("rss_save_path")?,
                               dotenv::var("rss_save_prefix")?,
                               now,
        );
        std::fs::write(filename,data)?;
    };

    Ok(())
}

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    dotenv::dotenv()?;

    let mut builder = env_logger::Builder::new();
    let level = dotenv::var("rss_level").unwrap_or("".into());
    match level {
        _ => {
            if level.eq_ignore_ascii_case("debug") {
                builder.filter_level(log::LevelFilter::Debug);
            } else if level.eq_ignore_ascii_case("error") {
                builder.filter_level(log::LevelFilter::Error);
            } else if level.eq_ignore_ascii_case("trace") {
                builder.filter_level(log::LevelFilter::Trace);
            } else if level.eq_ignore_ascii_case("info") {
                builder.filter_level(log::LevelFilter::Info);
            };
        }
    }
    builder.try_init()?;

    let seconds = dotenv::var("rss_check_seconds").unwrap_or("1".into());
    debug!("Tick Seconds = {}",seconds);

    let seconds = seconds.parse()?;
    loop{
        tokio::time::delay_for(std::time::Duration::from_secs(seconds)).await;
        callback().await?;
    }


    Ok(())
}
