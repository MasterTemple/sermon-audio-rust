use serde::{Deserialize, Serialize};

/// Information about a sermon series
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Series {
    /// title: Type  
    /// default: lite_sermon_series  
    #[serde(rename = "type")]
    pub r#type: String,

    /// Seriesid  
    #[serde(rename = "seriesID")]
    pub series_id: i64,

    /// Title  
    #[serde(rename = "title")]
    pub title: String,

    /// Broadcasterid  
    #[serde(rename = "broadcasterID")]
    pub broadcaster_id: String,

    /// Latest  
    #[serde(rename = "latest")]
    pub latest: String,

    /// Earliest  
    #[serde(rename = "earliest")]
    pub earliest: String,

    /// Updated  
    #[serde(rename = "updated")]
    pub updated: i64,

    /// Pickdate  
    #[serde(rename = "pickDate")]
    pub pick_date: String,

    /// Count  
    #[serde(rename = "count")]
    pub count: i64,

    /// Description  
    #[serde(rename = "description")]
    pub description: String,

    /// Podcastenabled  
    #[serde(rename = "podcastEnabled")]
    pub podcast_enabled: bool,

    /// Podcastspeaker  
    #[serde(rename = "podcastSpeaker")]
    pub podcast_speaker: String,

    /// Feedlinks  
    #[serde(rename = "feedLinks")]
    pub feed_links: Vec<SeriesFeedLink>,

    /// Image  
    #[serde(rename = "image")]
    pub image: String,

    /// Imageresizable  
    #[serde(rename = "imageResizable")]
    pub image_resizable: String,

    /// Rssurl  
    #[serde(rename = "rssURL")]
    pub rss_url: String,

    /// Rssatomurl  
    #[serde(rename = "rssAtomURL")]
    pub rss_atom_url: String,

    /// Broadcaster (Minimum broadcaster information contained in LiteSermon object)
    #[serde(rename = "broadcaster")]
    pub broadcaster: Broadcaster,

    /// Followed  
    #[serde(rename = "followed")]
    pub followed: bool,

    /// Languagecode  
    #[serde(rename = "languageCode")]
    pub language_code: String,

    /// Defaultsortby  
    #[serde(rename = "defaultSortBy")]
    pub default_sort_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeriesFeedLink {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "feedLinkID")]
    pub feed_link_id: i64,

    #[serde(rename = "seriesID")]
    pub series_id: i64,

    // TODO: [ tunein, apple, google-play, spotify, stitcher, other ]
    #[serde(rename = "podcastType")]
    pub podcast_type: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "link")]
    pub link: String,
}

/// Minimum broadcaster information contained in LiteSermon object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Broadcaster {
    /// Broadcasterid  
    #[serde(rename = "broadcasterID")]
    pub broadcaster_id: String,

    /// Displayname  
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Shortname  
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,

    /// Imageurl  
    #[serde(rename = "imageURL")]
    pub image_url: Option<String>,

    /// Imageresizable  
    #[serde(rename = "imageURLResizable")]
    pub image_url_resizable: Option<String>,

    /// Location  
    #[serde(rename = "location")]
    pub location: Option<String>,

    /// Solosite  
    #[serde(rename = "soloSite")]
    pub solo_site: Option<bool>,

    /// Solositedisabled  
    #[serde(rename = "soloSiteDisabled")]
    pub solo_site_disabled: Option<bool>,

    /// Disabled  
    #[serde(rename = "disabled")]
    pub disabled: bool,

    /// Canwebcast  
    #[serde(rename = "canWebcast")]
    pub can_webcast: Option<bool>,

    /// Albumarturl  
    #[serde(rename = "albumArtURL")]
    pub album_art_url: Option<String>,

    /// Webcastinprogress  
    #[serde(rename = "webcastInProgress")]
    pub webcast_in_progress: Option<bool>,

    /// Followed  
    #[serde(rename = "followed")]
    pub followed: Option<bool>,
}
