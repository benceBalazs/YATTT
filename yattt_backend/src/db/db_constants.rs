// Database related constants
#[cfg(feature = "test")]
pub const NAMESPACE: &str = "test";
#[cfg(feature = "test")]
pub const DATABASE: &str = "test";

#[cfg(not(feature = "test"))]
pub const NAMESPACE: &str = "yattt_backend";
#[cfg(not(feature = "test"))]
pub const DATABASE: &str = "yattt_backend";


// Table related constants
pub const TABLE_USER: &str = "User";
pub const TABLE_ATTENDANCE: &str = "Attendance";
pub const TABLE_CARD: &str = "Card";
pub const TABLE_LECTURE: &str = "Lecture";

// Table entry related constants
pub const ENTRY_USERNAME: &str = "username";
pub const ENTRY_PASSWORD: &str = "password";
pub const ENTRY_USER_ID: &str = "user_id";
pub const ENTRY_DEVICE_ID: &str = "device_id";
pub const ENTRY_TAG_ID: &str = "tag_id";
pub const ENTRY_CARD_NAME: &str = "card_name";
pub const ENTRY_CHECK_IN_TIME: &str = "check_in_time";
pub const ENTRY_CHECK_OUT_TIME: &str = "check_out_time";
pub const ENTRY_DURATION: &str = "duration";
pub const ENTRY_LV_NAME: &str = "lv_name";
pub const ENTRY_START_TIME: &str = "start_time";
pub const ENTRY_END_TIME: &str = "end_time";
