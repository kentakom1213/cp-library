/// Cスタイルのfor文
/// ```text
/// cfor! {define ; finish ; increment ;; {
///     block
/// }}
/// ```
#[macro_export]
macro_rules! cfor {
    ($def:stmt ; $fin:expr ; $incr:stmt ;; $bl:block) => {{
        $def
        while $fin {
            $bl
            $incr
        }
    }}
}
