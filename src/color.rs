/// 快速生成彩色字体
/// # Examples
/// ```
/// use color_string::color_string;
/// use color_string::Font::*;
/// let s = color_string! {
///     Red => 123456, "\n";
///     Green,Bold => "hello world";
/// };
/// println!("{s}");
/// assert_eq!("\u{1b}[0;31m123456\n\u{1b}[0;32;1mhello world\u{1b}[0m", s);
/// ```
#[macro_export]
macro_rules! color_string {
    ($( $($font:expr),* => $($s:expr),* );* $(;)?) => {{
        use std::fmt::Write;
        let mut s = String::new();
        $(
            $crate::write_fonts!(&mut s, $($font),*);
            $(write!(&mut s, "{}", $s).unwrap();)*
        )*
        s.push_str("\x1b[0m");
        s
    }};
}
