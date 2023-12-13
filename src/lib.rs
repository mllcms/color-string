//! # 快速构建彩色字符串
//!
//! # Examples
//!
//! ```
//! use color_string::Font::*;
//! use color_string::{cs, fonts, pcs, wcs, wf, Colored, FontTool};
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
//! let s7 = format!("{}Hello World!{}", fonts, Reset);
//!
//! let mut ss = String::new();
//! wf!(&mut ss, Red, Bold);
//! ss.push_str("Hello ");
//! ss.push_str("World!");
//! ss.reset();
//!
//! assert_eq!("\u{1b}[31;1mHello World!\u{1b}[0m", ss,);
//! for s in [s1, s2, s3, s4, s5, s6, s7] {
//! assert_eq!(ss, s)
//! }
//!
//! // Print server start
//! pcs!(Green => "➜ "; RBold => "Local: "; RCyan => "http://127.0.0.1:", 5173.bold());
//! ```

mod color;
mod font;

pub use color::*;
pub use font::*;
