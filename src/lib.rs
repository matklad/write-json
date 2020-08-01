//! Simple dependency-less macro-less trait-less JSON serialization.
//!
//! # Example
//!
//! ```
//! let mut buf = String::new();
//!
//! {
//!     let mut obj = write_json::object(&mut buf);
//!     obj.string("name", "Peter").number("favorite number", 92.0);
//!     obj.array("films")
//!         .string("Drowning By Numbers")
//!         .string("A Zed & Two Noughts");
//!     obj.null("suitcase");
//! }
//!
//! assert_eq!(
//!     buf,
//!     r#"{"name":"Peter","favorite number":92,"films":["Drowning By Numbers","A Zed & Two Noughts"],"suitcase":null}"#
//! )
//! ```

#[inline]
pub fn null(buf: &mut String) {
    encode_null(buf, ());
}
#[inline]
pub fn bool(buf: &mut String, value: bool) {
    encode_bool(buf, value);
}
#[inline]
pub fn number(buf: &mut String, number: f64) {
    encode_number(buf, number);
}
#[inline]
pub fn string(buf: &mut String, string: &str) {
    encode_str(buf, string);
}
#[inline]
pub fn object(buf: &mut String) -> Object<'_> {
    Object::new(buf)
}
#[inline]
pub fn array(buf: &mut String) -> Array<'_> {
    Array::new(buf)
}

pub struct Object<'a> {
    buf: &'a mut String,
    first: bool,
}

impl<'a> Object<'a> {
    #[inline]
    fn new(buf: &'a mut String) -> Self {
        buf.push('{');
        Object { buf, first: true }
    }
    #[inline]
    fn key(&mut self, key: &str) {
        if !self.first {
            self.buf.push(',');
        }
        self.first = false;
        encode_str(&mut self.buf, key);
        self.buf.push(':');
    }
    #[inline]
    fn field<T, F: FnOnce(&mut String, T)>(&mut self, key: &str, enc: F, value: T) -> &mut Self {
        self.key(key);
        enc(&mut self.buf, value);
        self
    }

    #[inline]
    pub fn null(&mut self, key: &str) -> &mut Self {
        self.field(key, encode_null, ())
    }
    #[inline]
    pub fn bool(&mut self, key: &str, value: bool) -> &mut Self {
        self.field(key, encode_bool, value)
    }
    #[inline]
    pub fn number(&mut self, key: &str, value: f64) -> &mut Self {
        self.field(key, encode_number, value)
    }
    #[inline]
    pub fn string(&mut self, key: &str, value: &str) -> &mut Self {
        self.field(key, encode_str, value)
    }
    #[inline]
    pub fn object(&mut self, key: &str) -> Object<'_> {
        self.key(key);
        Object::new(self.buf)
    }
    #[inline]
    pub fn array(&mut self, key: &str) -> Array<'_> {
        self.key(key);
        Array::new(self.buf)
    }
}

impl Drop for Object<'_> {
    #[inline]
    fn drop(&mut self) {
        self.buf.push('}')
    }
}

pub struct Array<'a> {
    buf: &'a mut String,
    first: bool,
}

impl<'a> Array<'a> {
    #[inline]
    fn new(buf: &'a mut String) -> Self {
        buf.push('[');
        Array { buf, first: true }
    }
    #[inline]
    fn comma(&mut self) {
        if !self.first {
            self.buf.push(',');
        }
        self.first = false;
    }
    #[inline]
    fn element<T, F: FnOnce(&mut String, T)>(&mut self, enc: F, value: T) -> &mut Self {
        self.comma();
        enc(&mut self.buf, value);
        self
    }

    #[inline]
    pub fn null(&mut self) -> &mut Self {
        self.element(encode_null, ())
    }
    #[inline]
    pub fn bool(&mut self, value: bool) -> &mut Self {
        self.element(encode_bool, value)
    }
    #[inline]
    pub fn number(&mut self, value: f64) -> &mut Self {
        self.element(encode_number, value)
    }
    #[inline]
    pub fn string(&mut self, value: &str) -> &mut Self {
        self.element(encode_str, value)
    }
    #[inline]
    pub fn object(&mut self) -> Object<'_> {
        self.comma();
        Object::new(self.buf)
    }
    #[inline]
    pub fn array(&mut self) -> Array<'_> {
        self.comma();
        Array::new(self.buf)
    }
}

impl Drop for Array<'_> {
    #[inline]
    fn drop(&mut self) {
        self.buf.push(']')
    }
}

#[inline]
fn encode_null(buf: &mut String, (): ()) {
    buf.push_str("null")
}
#[inline]
fn encode_bool(buf: &mut String, value: bool) {
    buf.push_str(if value { "true" } else { "false" })
}
#[inline]
fn encode_number(buf: &mut String, number: f64) {
    use std::fmt::Write;
    let _ = write!(buf, "{}", number);
}

#[inline]
fn encode_str(buf: &mut String, s: &str) {
    buf.reserve(s.len() + 2);
    buf.push('\"');
    if s.bytes()
        .all(|b| 32 <= b && b != b'"' && b != b'\\' && b < 128)
    {
        buf.push_str(s)
    } else {
        slow_path(buf, s)
    }
    buf.push('\"');

    #[inline(never)]
    fn slow_path(buf: &mut String, s: &str) {
        for c in s.chars() {
            let b = c as u8;
            match b {
                b'\\' | b'"' => push_escape(buf, c),
                b'\n' => push_escape(buf, 'n'),
                b'\r' => push_escape(buf, 'r'),
                b'\t' => push_escape(buf, 't'),
                0..=0x1f | 0x7f..=0x9f => {
                    push_escape(buf, 'u');
                    buf.push_str("00");
                    buf.push(hex(b & 0xF));
                    buf.push(hex(b >> 4));
                }
                _ => buf.push(c),
            }
        }
    }

    #[inline]
    fn push_escape(buf: &mut String, c: char) {
        buf.push('\\');
        buf.push(c);
    }

    #[inline]
    fn hex(b: u8) -> char {
        (b"0123456789ABCDEF"[(b & 0xF) as usize]) as char
    }
}
