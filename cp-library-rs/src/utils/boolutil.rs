//! boolから他の値への変換

pub trait BoolUtil {
    /// `true` → `"Yes"`，`false` → `"No"` に変換
    fn yesno(&self) -> &'static str;
    /// `true` → `"\n"`，`false` → `" "` に変換
    fn endl(&self) -> &'static str;
}

impl BoolUtil for bool {
    fn yesno(&self) -> &'static str {
        if *self {
            "Yes"
        } else {
            "No"
        }
    }
    fn endl(&self) -> &'static str {
        if *self {
            "\n"
        } else {
            " "
        }
    }
}
