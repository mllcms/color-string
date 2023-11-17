//! # 快速构建彩色字符串
//!
//! # Examples
//!
//! ```
//! use color_string::Font::*;
//! use color_string::{cs, fonts, wcs, wf, Colored, FontTool};
//!
//! let fonts = fonts!(Red, Bold);
//!
//! let mut s1 = String::new();
//! wcs!(&mut s1, fonts => "Hello World!");
//!
//! let mut s2 = String::new();
//! wcs!(&mut s2, fonts; "{} {}","Hello","World!");
//!
//! let s3 = "Hello World!".fonts(fonts.clone());
//! let s4 = cs!(fonts => "Hello World!");
//! let s5 = cs!(Red,Bold; "{} {}", "Hello","World!");
//! let s6 = cs!(Red,Bold => "Hello ","World!");
//!
//! let mut s7 = String::new();
//! wf!(&mut s7, Red, Bold);
//! s7.push_str("Hello ");
//! s7.push_str("World!");
//! s7.reset();
//!
//! assert_eq!("\u{1b}[0;31;1mHello World!\u{1b}[0m", s7,);
//! for s in [s1, s2, s3, s4, s5, s6] {
//! assert_eq!(s7, s)
//! }
//! ```

mod color;
mod font;

pub use color::*;
pub use font::*;
