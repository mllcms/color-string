use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub enum Font {
    /// 重置样式
    Reset,
    /// 字体加粗
    Bold,
    /// 下划线
    Underline,
    /// 斜体
    Italic,
    /// 反色(前景色背景色交换)
    Reverse,
    /// 删除线
    Delete,
    /// 黑色字体
    Black,
    /// 红色字体
    Red,
    /// 绿色字体
    Green,
    /// 黄色字体
    Yellow,
    /// 蓝色字体
    Blue,
    /// 紫色字体
    Purple,
    /// 青色字体d
    Cyan,
    /// 灰色字体
    Grey,
    /// 黑色背景
    BgBlack,
    /// 红色背景
    BgRed,
    /// 绿色背景
    BgGreen,
    /// 黄色背景
    BgYellow,
    /// 蓝色背景
    BgBlue,
    /// 紫色背景
    BgPurple,
    /// 青色背景
    BgCyan,
    /// 灰色背景
    BgGrey,
    /// 字体颜色 RGB
    Color(u8, u8, u8),
    /// 背景颜色 RGB
    BgColor(u8, u8, u8),
}

impl Font {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Font::Reset => Cow::Borrowed("0"),
            Font::Bold => Cow::Borrowed("1"),
            Font::Italic => Cow::Borrowed("3"),
            Font::Underline => Cow::Borrowed("4"),
            Font::Reverse => Cow::Borrowed("7"),
            Font::Delete => Cow::Borrowed("9"),
            Font::Black => Cow::Borrowed("30"),
            Font::Red => Cow::Borrowed("31"),
            Font::Green => Cow::Borrowed("32"),
            Font::Yellow => Cow::Borrowed("33"),
            Font::Blue => Cow::Borrowed("34"),
            Font::Purple => Cow::Borrowed("35"),
            Font::Cyan => Cow::Borrowed("36"),
            Font::Grey => Cow::Borrowed("37"),
            Font::BgBlack => Cow::Borrowed("40"),
            Font::BgRed => Cow::Borrowed("41"),
            Font::BgGreen => Cow::Borrowed("42"),
            Font::BgYellow => Cow::Borrowed("43"),
            Font::BgBlue => Cow::Borrowed("44"),
            Font::BgPurple => Cow::Borrowed("45"),
            Font::BgCyan => Cow::Borrowed("46"),
            Font::BgGrey => Cow::Borrowed("47"),
            Font::Color(r, g, b) => Cow::Owned(format!("38;2;{};{};{}", r, g, b)),
            Font::BgColor(r, g, b) => Cow::Owned(format!("48;2;{};{};{}", r, g, b)),
        }
    }
}

impl Display for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m", self.as_str())
    }
}

#[test]
fn font_is_work() {
    assert_eq!("\x1b[30m", format!("{}", Font::Black));
    assert_eq!("\x1b[38;2;1;2;3m", format!("{}", Font::Color(1, 2, 3)));
}

#[macro_export]
macro_rules! fonts {
    ($ ($font:expr),*) => {{
        let mut s = String::new();
        $crate::write_fonts!(&mut s, $($font),*);
        s
    }};
}

#[macro_export]
macro_rules! write_fonts {
    ($s:expr, $($font:expr),*) => {{
        use std::fmt::Write;
        $s.push_str("\x1b[");
        $(write!($s, "{};", $font.as_str()).unwrap();)*
        $s.pop();
        $s.push('m');
    }};
}

#[test]
fn fonts_is_work() {
    let fonts = fonts!(
        Font::Red,
        Font::Bold,
        Font::Underline,
        Font::BgColor(1, 2, 3)
    );
    assert_eq!("\u{1b}[31;1;4;48;2;1;2;3m", fonts)
}
