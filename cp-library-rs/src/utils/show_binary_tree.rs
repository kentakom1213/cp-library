//! 2分木を整形して表示する

const LEFT: &str = "  ┌─";
const MID: &str = "  │ ";
const RIGHT: &str = "  └─";
const NULL: &str = "";
const BLANK: &str = "    ";

/// 2分木を整形して表示する
pub trait ShowBinaryTree<P> {
    /// **\<required\>** 左の子のポインタを取得する
    fn get_left(&self, ptr: &P) -> Option<P>;

    /// **\<required\>** 右の子のポインタを取得する
    fn get_right(&self, ptr: &P) -> Option<P>;

    /// **\<required\>** 根を取得する
    fn get_root(&self) -> Option<P>;

    /// **\<required\>** ノードの値を表示する
    fn print_node(&self, ptr: &P) -> String;

    /// 再帰的にprintを行う
    fn print_inner(&self, ptr: P, fill: &mut Vec<&'static str>, last: &'static str) {
        // 表示の調整
        let mut tmp = None;
        if fill.last().is_some_and(|x| x == &last) {
            tmp = fill.pop();
            fill.push(BLANK);
        } else if fill.last().is_some_and(|x| x != &NULL && x != &BLANK) {
            tmp = fill.pop();
            fill.push(MID);
        }
        fill.push(last);

        // 右の子を表示
        if let Some(left) = Self::get_left(self, &ptr) {
            self.print_inner(left, fill, LEFT);
        }

        // 自分を出力
        eprintln!("│{} {}", fill.join(""), self.print_node(&ptr));

        // 右の子を表示
        if let Some(right) = Self::get_right(self, &ptr) {
            self.print_inner(right, fill, RIGHT);
        }

        // 戻す
        fill.pop();
        if let Some(tmp) = tmp {
            fill.pop();
            fill.push(tmp);
        }
    }

    /// 2分木としてフォーマットする
    fn print_as_binary_tree(&mut self) {
        #[cfg(debug_assertions)]
        {
            eprintln!("┌───────────────────────────────────────────────────────");
            if let Some(root) = Self::get_root(self) {
                Self::print_inner(self, root, &mut vec![], NULL);
            }
            eprintln!("└───────────────────────────────────────────────────────");
        }
    }
}
