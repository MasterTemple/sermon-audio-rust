use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationOption {
    // An enumeration.
    pub method: String,

    /// title: Label
    /// access_level: 5
    /// null: false
    #[serde(rename = "label")]
    pub label: String,

    /// title: Email
    /// access_level: 5
    /// null: false
    #[serde(rename = "email")]
    pub email: String,

    /// title: Url
    /// access_level: 5
    /// null: false
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAccount {
    /// title: Url
    /// access_level: 5
    /// null: false
    #[serde(rename = "url")]
    pub url: String,

    /// title: Label
    /// access_level: 5
    /// null: false
    #[serde(rename = "label")]
    pub label: String,
}

/// Minimum speaker information contained in LiteSermon object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimarySpeaker {
    /// Type identifier for the speaker object.
    /// Default: "lite_sermon_speaker"
    /// Access level: 5
    /// Indicates object type
    #[serde(rename = "type")]
    pub speaker_type: String,

    /// Unique identifier for the speaker.
    /// Access level: 5
    #[serde(rename = "speakerID")]
    pub speaker_id: i32,

    /// The speaker's display name.
    /// NOTE: this currently doubles as the speakerID / speakerName when used in other URLs.
    /// Access level: 5
    #[serde(rename = "displayName")]
    pub display_name: String,

    /// The rectangular, portrait-style image of the speaker that would be used,
    /// for example, on a speaker detail page.
    /// Access level: 5
    #[serde(rename = "portraitURL")]
    pub portrait_url: String,

    /// The large, square, stylized speaker image that is used for
    /// Player album art.
    /// Access level: 5
    #[serde(rename = "albumArtURL")]
    pub album_art_url: String,

    /// The small, rounded thumbnail images that would be used,
    /// for example, in a mobile app list view.
    /// Access level: 5
    #[serde(rename = "roundedThumbnailImageURL")]
    pub rounded_thumbnail_image_url: String,

    /// Resizable version of the portrait URL.
    /// Access level: 5
    #[serde(rename = "portraitURLResizable")]
    pub portrait_url_resizable: String,

    /// Resizable version of the rounded thumbnail image URL.
    /// Access level: 5
    #[serde(rename = "roundedThumbnailImageURLResizable")]
    pub rounded_thumbnail_image_url_resizable: String,

    /// Indicates whether the speaker is being followed by the current user.
    /// Access level: 5
    pub followed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoloSite {
    // title: Solo Id
    // access_level: 5
    // null: false
    #[serde(rename = "solo_id")]
    pub solo_id: String,

    // title: Broadcaster Id
    // access_level: 5
    // null: false
    #[serde(rename = "broadcaster_id")]
    pub broadcaster_id: String,

    // title: Domain
    // access_level: 5
    // null: false
    #[serde(rename = "domain")]
    pub domain: String,

    // title: SoloStatus
    // access_level: 5
    // null: false
    // The status of a solo site
    // Enum:
    // Array [ 5 ]
    #[serde(rename = "status")]
    pub status: String,

    // title: Create Token
    // access_level: 5
    // null: false
    #[serde(rename = "create_token")]
    pub create_token: String,

    // title: Create Date
    // access_level: 5
    // null: false
    #[serde(rename = "create_date")]
    pub create_date: u32,

    // title: Update Date
    // access_level: 5
    // null: false
    #[serde(rename = "update_date")]
    pub update_date: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChurchServiceTime {
    // TODO: This is probably an enum
    /// title: Servicetypeid
    /// access_level: 5
    /// null: false
    #[serde(rename = "serviceTypeID")]
    pub service_type_id: u32,

    /// title: Title
    /// access_level: 5
    /// null: false
    #[serde(rename = "title")]
    pub title: String,

    /// title: Starttime
    /// access_level: 5
    /// null: false
    #[serde(rename = "startTime")]
    pub start_time: String,

    /// title: Dayofweek
    /// access_level: 5
    /// null: false
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BroadCaster {
    /// Information about a broadcaster
    #[serde(rename = "description")]
    pub description: String,

    /// title: Type
    /// default: broadcaster
    /// access_level: 5
    /// null: false
    /// indicates object type
    #[serde(rename = "type")]
    pub r#type: String,

    /// title: Lite Type
    /// default: true
    /// access_level: 5
    /// null: false
    #[serde(rename = "lite_type")]
    pub lite_type: String,

    /// title: Broadcasterid
    /// access_level: 5
    /// null: false
    #[serde(rename = "broadcasterID")]
    pub broadcaster_id: String,

    /// title: Displayname
    /// access_level: 5
    /// null: false
    /// The broadcaster’s full-length name (appropriate for most devices).
    #[serde(rename = "displayName")]
    pub display_name: String,

    /// title: Location
    /// access_level: 5
    /// null: false
    /// A verbal description of the location (i.e. Greenville, South Carolina).
    #[serde(rename = "location")]
    pub location: String,

    /// title: Minister
    /// access_level: 5
    /// null: true
    /// The name of the primary speaking minister.
    #[serde(rename = "minister")]
    pub minister: String,

    /// title: Latitude
    /// access_level: 5
    /// null: false
    /// The broadcaster’s latitude (useful for displaying on maps)
    #[serde(rename = "latitude")]
    pub latitude: f64,

    /// title: Longitude
    /// access_level: 5
    /// null: false
    /// The broadcaster’s longitude (useful for displaying on maps)
    #[serde(rename = "longitude")]
    pub longitude: f64,

    /// title: Imageurl
    /// access_level: 5
    /// null: false
    /// The rectangular image for the broadcaster.
    #[serde(rename = "imageURL")]
    pub image_url: String,

    /// title: Imageurlresizable
    /// access_level: 5
    /// null: false
    #[serde(rename = "imageURLResizable")]
    pub image_urlresizable: String,

    /// title: Denomination
    /// access_level: 5
    /// null: true
    /// The denomination of the broadcaster (usually an abbreviation).
    #[serde(rename = "denomination")]
    pub denomination: String,

    /// title: Webcastinprogress
    /// access_level: 5
    /// null: false
    /// Indicates if a webcast is currently in progress for the broadcaster.
    #[serde(rename = "webcastInProgress")]
    pub webcast_in_progress: bool,

    /// title: Phone
    /// access_level: 5
    /// null: true
    /// The broadcaster’s phone number.
    #[serde(rename = "phone")]
    pub phone: String,

    /// title: Followed
    /// access_level: 5
    /// null: false
    #[serde(rename = "followed")]
    pub followed: bool,

    /// title: Donatable
    /// access_level: 5
    /// null: false
    #[serde(rename = "donatable")]
    pub donatable: bool,

    /// title: Miles
    /// access_level: 5
    /// null: false
    #[serde(rename = "miles")]
    pub miles: f64,

    /// title: Languagecode
    /// access_level: 5
    /// null: false
    #[serde(rename = "languageCode")]
    pub language_code: String,

    /// title: Aboutus
    /// access_level: 5
    /// null: true
    /// An informational blurb about the broadcaster. NOTE: May contain HTML.
    #[serde(rename = "aboutUs")]
    pub about_us: String,

    /// title: Numberofsermons
    /// default: 0
    /// access_level: 5
    /// null: false
    #[serde(rename = "numberOfSermons")]
    pub number_of_sermons: u32,

    /// title: Joined
    /// access_level: 5
    /// null: false
    #[serde(rename = "joined")]
    pub joined: u32,

    /// title: Idcode
    /// access_level: 5
    /// null: false
    #[serde(rename = "idCode")]
    pub id_code: String,

    /// title: Shortname
    /// access_level: 5
    /// null: false
    /// The broadcaster’s shorter display name.
    #[serde(rename = "shortName")]
    pub short_name: String,

    /// title: Address
    /// access_level: 5
    /// null: false
    /// The physical address for the broadcaster.
    #[serde(rename = "address")]
    pub address: String,

    /// title: Addressmail
    /// access_level: 5
    /// null: false
    /// The mailing address for the broadcaster.
    #[serde(rename = "addressMail")]
    pub address_mail: String,

    /// title: Country
    /// access_level: 5
    /// null: true
    /// The English name of the broadcaster’s country.
    #[serde(rename = "country")]
    pub country: String,

    /// title: Countryisocode
    /// access_level: 5
    /// null: false
    #[serde(rename = "countryISOCode")]
    pub country_isocode: String,

    /// title: Facebookusername
    /// access_level: 5
    /// null: true
    /// The Facebook username for the broadcaster, this is useful to link to a broadcaster’s Facebook page.
    #[serde(rename = "facebookUsername")]
    pub facebook_username: String,

    /// title: Twitterusername
    /// access_level: 5
    /// null: true
    /// The Twitter username for the broadcaster, this is useful to link to a broadcaster’s Twitter page.
    #[serde(rename = "twitterUsername")]
    pub twitter_username: String,

    /// title: Homepageurl
    /// access_level: 5
    /// null: true
    /// The broadcaster’s website URL.
    #[serde(rename = "homePageURL")]
    pub home_page_url: String,

    /// title: Albumarturl
    /// access_level: 5
    /// null: false
    /// The square album art URL format for this broadcaster.
    #[serde(rename = "albumArtURL")]
    pub album_art_url: String,

    /// title: Bibleversion
    /// access_level: 5
    /// null: true
    /// The canonical abbreviation for the broadcaster’s preferred translation. Primarily used for display purposes.
    #[serde(rename = "bibleVersion")]
    pub bible_version: String,

    /// title: Servicetimesarepreformatted
    /// access_level: 5
    /// null: false
    /// Indicates if the broadcaster has added formatting to serviceTimes field.
    #[serde(rename = "serviceTimesArePreformatted")]
    pub service_times_are_preformatted: String,

    /// title: Servicetimes
    /// access_level: 5
    /// null: true
    /// A free-form description of service times of a broadcaster.
    #[serde(rename = "serviceTimes")]
    pub service_times: String,

    /// title: Servicetimeslist
    /// access_level: 5
    /// null: true
    /// A list of service times of a broadcaster.
    #[serde(rename = "serviceTimesList")]
    pub service_times_list: Vec<ChurchServiceTime>,

    /// title: Canwebcast
    /// access_level: 5
    /// null: false
    /// Indicates if a broacaster has webcasting enabled on their account.
    pub can_webcast: bool,

    /// title: Listenlinenumber
    /// access_level: 5
    /// null: true
    /// The broadcaster’s listen line phone number.
    #[serde(rename = "listenLineNumber")]
    pub listen_line_number: String,

    /// title: Vacantpulpit
    /// access_level: 5
    /// null: false
    /// Indicates if the broadcaster is currently seeking a minister.
    #[serde(rename = "vacantPulpit")]
    pub vacant_pulpit: bool,

    /// title: Categories
    /// access_level: 5
    /// null: false
    #[serde(rename = "categories")]
    pub categories: u32,

    /// title: Welcomevideoid
    /// access_level: 5
    /// null: true
    #[serde(rename = "welcomeVideoID")]
    pub welcome_video_id: String,

    /// title: Disabled
    /// access_level: 5
    /// null: false
    #[serde(rename = "disabled")]
    pub disabled: bool,

    /// title: Solositedisabled
    /// access_level: 5
    /// null: false
    #[serde(rename = "soloSiteDisabled")]
    pub solo_site_disabled: bool,

    /// solo_id*	Solo Id[...]
    /// broadcaster_id*	Broadcaster Id[...]
    /// domain*	Domain[...]
    /// status*	SoloStatus[...]
    /// create_token	Create Token[...]
    /// create_date	Create Date[...]
    /// update_date	Update Date[...]
    /// }
    #[serde(rename = "soloSite")]
    pub solo_site: SoloSite,

    /// title: Rssfeedurl
    /// access_level: 5
    /// null: false
    #[serde(rename = "rssFeedURL")]
    pub rss_feed_url: String,

    /// title: Rssfeedatomurl
    /// access_level: 5
    /// null: false
    #[serde(rename = "rssFeedAtomURL")]
    pub rss_feed_atom_url: String,

    /// title: Churchsize
    /// access_level: 5
    /// null: false
    #[serde(rename = "churchSize")]
    pub church_size: u32,

    /// description:
    /// Minimum speaker information contained in LiteSermon object.
    #[serde(rename = "primarySpeaker")]
    pub primary_speaker: Vec<PrimarySpeaker>,

    /// title: Groups
    /// access_level: 5
    /// null: false
    #[serde(rename = "groups")]
    groups: Vec<String>,

    /// title: Socialaccounts
    /// access_level: 5
    /// null: false
    #[serde(rename = "socialAccounts")]
    pub social_accounts: Vec<SocialAccount>,

    /// title: Bannerimageurl
    /// access_level: 5
    /// null: false
    #[serde(rename = "bannerImageURL")]
    pub banner_image_url: String,

    /// title: Donationoptions
    /// access_level: 5
    /// null: false
    #[serde(rename = "donationOptions")]
    pub donation_options: Vec<DonationOption>,
}
