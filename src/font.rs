use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub enum Font {
    /// 重置样式
    Reset,
    /// 字体加粗
    Bold,
    RBold,
    /// 下划线
    Underline,
    RUnderline,
    /// 斜体
    Italic,
    RItalic,
    /// 反色(前景色背景色交换)
    Reverse,
    RReverse,
    /// 删除线
    Delete,
    RDelete,
    /// 黑色字体
    Black,
    RBlack,
    /// 红色字体
    Red,
    RRed,
    /// 绿色字体
    Green,
    RGreen,
    /// 黄色字体
    Yellow,
    RYellow,
    /// 蓝色字体
    Blue,
    RBlue,
    /// 紫色字体
    Purple,
    RPurple,
    /// 青色字体
    Cyan,
    RCyan,
    /// 灰色字体
    Grey,
    RGrey,
    /// 黑色背景
    BgBlack,
    RBgBlack,
    /// 红色背景
    BgRed,
    RBgRed,
    /// 绿色背景
    BgGreen,
    RBgGreen,
    /// 黄色背景
    BgYellow,
    RBgYellow,
    /// 蓝色背景
    BgBlue,
    RBgBlue,
    /// 紫色背景
    BgPurple,
    RBgPurple,
    /// 青色背景
    BgCyan,
    RBgCyan,
    /// 灰色背景
    BgGrey,
    RBgGrey,
    /// 字体颜色 RGB
    Color(u8, u8, u8),
    RColor(u8, u8, u8),
    /// 背景颜色 RGB
    BgColor(u8, u8, u8),
    RBgColor(u8, u8, u8),
}

impl Font {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Font::Reset => Cow::Borrowed("0"),
            Font::Bold => Cow::Borrowed("1"),
            Font::RBold => Cow::Borrowed("0;1"),
            Font::Italic => Cow::Borrowed("3"),
            Font::RItalic => Cow::Borrowed("0;3"),
            Font::Underline => Cow::Borrowed("4"),
            Font::RUnderline => Cow::Borrowed("0;4"),
            Font::Reverse => Cow::Borrowed("7"),
            Font::RReverse => Cow::Borrowed("0;7"),
            Font::Delete => Cow::Borrowed("9"),
            Font::RDelete => Cow::Borrowed("0;9"),
            Font::Black => Cow::Borrowed("30"),
            Font::RBlack => Cow::Borrowed("0;30"),
            Font::Red => Cow::Borrowed("31"),
            Font::RRed => Cow::Borrowed("0;31"),
            Font::Green => Cow::Borrowed("32"),
            Font::RGreen => Cow::Borrowed("0;32"),
            Font::Yellow => Cow::Borrowed("33"),
            Font::RYellow => Cow::Borrowed("0;33"),
            Font::Blue => Cow::Borrowed("34"),
            Font::RBlue => Cow::Borrowed("0;34"),
            Font::Purple => Cow::Borrowed("35"),
            Font::RPurple => Cow::Borrowed("0;35"),
            Font::Cyan => Cow::Borrowed("36"),
            Font::RCyan => Cow::Borrowed("0;36"),
            Font::Grey => Cow::Borrowed("37"),
            Font::RGrey => Cow::Borrowed("0;37"),
            Font::BgBlack => Cow::Borrowed("40"),
            Font::RBgBlack => Cow::Borrowed("0;40"),
            Font::BgRed => Cow::Borrowed("41"),
            Font::RBgRed => Cow::Borrowed("0;41"),
            Font::BgGreen => Cow::Borrowed("42"),
            Font::RBgGreen => Cow::Borrowed("0;42"),
            Font::BgYellow => Cow::Borrowed("43"),
            Font::RBgYellow => Cow::Borrowed("0;43"),
            Font::BgBlue => Cow::Borrowed("44"),
            Font::RBgBlue => Cow::Borrowed("0;44"),
            Font::BgPurple => Cow::Borrowed("45"),
            Font::RBgPurple => Cow::Borrowed("0;45"),
            Font::BgCyan => Cow::Borrowed("46"),
            Font::RBgCyan => Cow::Borrowed("0;46"),
            Font::BgGrey => Cow::Borrowed("47"),
            Font::RBgGrey => Cow::Borrowed("0;47"),
            Font::Color(r, g, b) => Cow::Owned(format!("38;2;{};{};{}", r, g, b)),
            Font::RColor(r, g, b) => Cow::Owned(format!("0;38;2;{};{};{}", r, g, b)),
            Font::BgColor(r, g, b) => Cow::Owned(format!("48;2;{};{};{}", r, g, b)),
            Font::RBgColor(r, g, b) => Cow::Owned(format!("0;48;2;{};{};{}", r, g, b)),
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

/// 组合多种字体
/// # Example
/// ```
/// use color_string::fonts;
/// use color_string::Font::*;
/// let fonts = fonts!(Red, Bold, Underline, BgColor(1, 2, 3));
/// assert_eq!("\x1b[31;1;4;48;2;1;2;3m", fonts)
/// ```
#[macro_export]
macro_rules! fonts {
    ($ ($font:expr),*) => {{
        let mut s = String::new();
        $crate::wf!(&mut s, $($font),*);
        s
    }};
}

/// 写入多种字体
/// # Example
/// ```
/// use color_string::wf;
/// use color_string::Font::*;
/// let mut fonts = String::new();
/// wf!(&mut fonts, Red, Bold, Underline, BgColor(1, 2, 3));
/// println!("{} hello world! {}", fonts, Reset);
/// assert_eq!("\x1b[31;1;4;48;2;1;2;3m", fonts)
/// ```
#[macro_export]
macro_rules! wf {
    ($s:expr, $($font:expr),*) => {{
        use std::fmt::Write;
        $s.push_str("\x1b[");
                $(
            let s = $font.as_str();
            if s.starts_with("\x1b[") {
                write!($s, "{};", &s[2..s.len() - 1]).unwrap();
            }else {
                write!($s, "{};", s).unwrap();
            }
        )*
        $s.pop();
        $s.push('m');
    }};
}

pub trait FontTool {
    fn reset(&mut self) -> &Self;

    fn no_reset(self) -> Self;

    fn none_font(&self) -> Self;
}

impl FontTool for String {
    fn reset(&mut self) -> &Self {
        let reset = Font::Reset.to_string();
        if !self.ends_with(&reset) {
            self.push_str(&reset)
        }
        self
    }

    fn no_reset(mut self) -> Self {
        if self.ends_with("\x1b[0m") {
            self.truncate(self.len() - 4);
        }
        self
    }

    fn none_font(&self) -> Self {
        let mut s = String::with_capacity(self.len());
        let mut flag = false;
        for c in self.chars() {
            match c {
                '\x1b' => flag = true,
                'm' if flag => flag = false,
                _ if !flag => s.push(c),
                _ => {}
            }
        }
        s
    }
}

#[test]
fn aaa() {
    use crate::cs;
    use crate::Colored;
    let s = cs!(Font::Red => "hello");
    println!("{s}");
    println!("{}", s.none_font());
    println!("{}456", "123".red().no_reset());
}
