use crate::font::Font;
use std::fmt::Display;

/// color_string 快速生成彩色字符串
/// # Examples
/// ```
/// use color_string::cs;
/// use color_string::Font::*;
/// let s = cs!(Red, Bold; "{:X}", u32::MAX);
/// println!("{s}");
/// assert_eq!("\u{1b}[31;1mFFFFFFFF\u{1b}[0m", s);
///
/// let s = cs!{
///     Red => 123456, "\n";
///     Green,Bold => "hello world";
/// };
/// println!("{s}");
/// assert_eq!("\u{1b}[31m123456\n\u{1b}[32;1mhello world\u{1b}[0m", s);
/// ```
#[macro_export]
macro_rules! cs {
     ($($arg:tt)*) => {{
        let mut s = String::new();
        $crate::wcs!(&mut s, $($arg)*);
        s
    }};
}

/// 输出彩色字体到标准输出
/// # Examples
/// ```
/// use color_string::pcs;
/// use color_string::Font::*;
/// pcs!(Red => "hello world");
/// pcs!(Red; "{} {}","hello","world");
/// ```
#[macro_export]
macro_rules! pcs {
    ($($arg:tt)*) => {{
        let s = $crate::cs!($($arg)*);
        println!("{s}")
    }};
}

/// 输出彩色字体到标准错误
/// ```
/// use color_string::epcs;
/// use color_string::Font::*;
/// epcs!(Red => "hello world");
/// epcs!(Red; "{} {}","hello","world");
/// ```
#[macro_export]
macro_rules! epcs {
    ($($arg:tt)*) => {{
        let s = $crate::cs!($($arg)*);
        eprintln!("{s}")
    }};
}

/// write_color_string 写入彩色字符串
/// # Examples
/// ```
/// use color_string::wcs;
/// use color_string::Font::*;
/// let mut s = String::new();
/// wcs!(&mut s, Red, Bold; "{:X}", u32::MAX);
/// println!("{s}");
/// assert_eq!("\u{1b}[31;1mFFFFFFFF\u{1b}[0m", s);
///
/// let mut s = String::new();
/// wcs!{
///     &mut s,
///     Red => 123456, "\n";
///     Green,Bold => "hello world";
/// }
/// println!("{s}");
/// assert_eq!("\u{1b}[31m123456\n\u{1b}[32;1mhello world\u{1b}[0m", s);
/// ```
#[macro_export]
macro_rules! wcs {
     ($buf:expr, $($font:expr),* ; $($arg:tt)*) => {{
        use std::fmt::Write;
        $crate::wf!($buf, $($font),*);
        write!($buf, $($arg)*).unwrap();
        $buf.push_str("\x1b[0m");
    }};

    ($buf:expr, $($($font:expr),* => $($s:expr),* );* $(;)?) => {{
        use std::fmt::Write;
        $(
            $crate::wf!($buf, $($font),*);
            $(write!($buf, "{}", $s).unwrap();)*
        )*
        $buf.push_str("\x1b[0m");
    }};
}

macro_rules! colored_trait {
    ($($method:ident => $font:expr),*) => {
        pub trait Colored:Display {
            $(
                fn $method(&self) -> String {
                    $crate::cs!($font => self)
                }
            )*

            fn color(&self, r:u8, g:u8, b:u8) -> String {
                $crate::cs!($crate::color::Font::Color(r,g,b) => self)
            }

            fn bg_color(&self, r:u8, g:u8, b:u8) -> String {
                $crate::cs!($crate::color::Font::BgColor(r,g,b) => self)
            }

            /// # Examples
            /// ```rust,ignore
            /// println!("{}","test".fonts("\x1b[1;31m"));
            /// println!("{}","test".fonts(fonts!(Font::Bold,Font::Red)));
            /// ```
            #[allow(clippy::ptr_arg)]
            fn fonts(&self, mut fonts:String) -> String {
                use std::fmt::Write;
                write!(&mut fonts, "{}\x1b[0m", self).unwrap();
                fonts
            }
        }
    };
}

/// 为所有实现 Display 的数据实现 Colored
impl<T> Colored for T where T: Display + ?Sized {}
colored_trait! {
    bold => Font::Bold,
    underline => Font::Underline,
    italic => Font::Italic,
    reverse => Font::Reverse,
    delete => Font::Delete,
    black => Font::Black,
    red => Font::Red,
    green => Font::Green,
    yellow => Font::Yellow,
    blue => Font::Blue,
    purple => Font::Purple,
    cyan => Font::Cyan,
    grey => Font::Grey,
    bg_black => Font::BgBlack,
    bg_red => Font::BgRed,
    bg_green => Font::BgGreen,
    bg_yellow => Font::BgYellow,
    bg_blue => Font::BgBlue,
    bg_purple => Font::BgPurple,
    bg_cyan => Font::BgCyan,
    bg_grey => Font::BgGrey
}
