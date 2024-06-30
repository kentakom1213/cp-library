//! boolから"Yes"/"No"への変換

pub trait YesNo {
    /// `true`->`"Yes"`, `false`->`"No"` に変換
    fn yesno(&self) -> String;
}

impl YesNo for bool {
    fn yesno(&self) -> String {
        if *self {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    }
}
