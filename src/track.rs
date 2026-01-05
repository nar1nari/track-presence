#[derive(Debug)]
pub struct Track {
    pub player: String,
    pub title: String,
    pub url: Option<String>,
    pub artists: Option<Vec<String>>,
    pub position: Option<std::time::Duration>,
    pub length: Option<std::time::Duration>,
    pub paused: bool,
}

type Fingerprint<'a> = (
    &'a str,
    &'a str,
    Option<&'a str>,
    Option<&'a [String]>,
    Option<std::time::Duration>,
    Option<std::time::Duration>,
    bool,
);

impl Track {
    fn fingerprint(&self) -> Fingerprint<'_> {
        (
            &self.player,
            &self.title,
            self.url.as_deref(),
            self.artists.as_deref(),
            self.position,
            self.length,
            self.paused,
        )
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.fingerprint() == other.fingerprint()
    }
}
