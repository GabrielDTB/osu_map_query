pub struct Filedata {
    pub file_format: Option<u8>,
    pub audio_filename: Option<String>,
    pub audio_lead_in: Option<i64>,
    pub audio_hash: Option<String>, // Deprecated
    pub preview_time: Option<i64>,
    pub countdown_offset: Option<i64>,
}
