/// color_string 快速生成彩色字符串
/// # Examples
/// ```
/// use color_string::cs;
/// use color_string::Font::*;
/// let s = cs!(Red, Bold; "{:X}", u32::MAX);
/// println!("{s}");
/// assert_eq!("\u{1b}[0;31;1mFFFFFFFF\u{1b}[0m", s);
///
/// let s = cs!{
///     Red => 123456, "\n";
///     Green,Bold => "hello world";
/// };
/// println!("{s}");
/// assert_eq!("\u{1b}[0;31m123456\n\u{1b}[0;32;1mhello world\u{1b}[0m", s);
/// ```
#[macro_export]
macro_rules! cs {
     ($($arg:tt)*) => {{
        let mut s = String::new();
        $crate::wcs!(&mut s, $($arg)*);
        s
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
/// assert_eq!("\u{1b}[0;31;1mFFFFFFFF\u{1b}[0m", s);
///
/// let mut s = String::new();
/// wcs!{
///     &mut s,
///     Red => 123456, "\n";
///     Green,Bold => "hello world";
/// }
/// println!("{s}");
/// assert_eq!("\u{1b}[0;31m123456\n\u{1b}[0;32;1mhello world\u{1b}[0m", s);
/// ```
#[macro_export]
macro_rules! wcs {
     ($buf:expr, $($font:expr),* ; $($arg:tt)*) => {{
        use std::fmt::Write;
        $crate::write_fonts!($buf, $($font),*);
        write!($buf, $($arg)*).unwrap();
        $buf.push_str("\x1b[0m");
    }};

    ($buf:expr, $($($font:expr),* => $($s:expr),* );* $(;)?) => {{
        use std::fmt::Write;
        $(
            $crate::write_fonts!($buf, $($font),*);
            $(write!($buf, "{}", $s).unwrap();)*
        )*
        $buf.push_str("\x1b[0m");
    }};
}
