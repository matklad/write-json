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

pub fn null(buf: &mut String) {
    encode_null(buf, ());
}
pub fn bool(buf: &mut String, value: bool) {
    encode_bool(buf, value);
}
pub fn number(buf: &mut String, number: f64) {
    encode_number(buf, number);
}
pub fn string(buf: &mut String, string: &str) {
    encode_str(buf, string);
}
pub fn object(buf: &mut String) -> Object<'_> {
    Object::new(buf)
}
pub fn array(buf: &mut String) -> Array<'_> {
    Array::new(buf)
}

pub struct Object<'a> {
    buf: &'a mut String,
    first: bool,
}

impl<'a> Object<'a> {
    fn new(buf: &'a mut String) -> Self {
        buf.push('{');
        Object { buf, first: true }
    }
    fn key(&mut self, key: &str) {
        if !self.first {
            self.buf.push(',');
        }
        self.first = false;
        encode_str(&mut self.buf, key);
        self.buf.push(':');
    }
    fn field<T, F: FnOnce(&mut String, T)>(&mut self, key: &str, enc: F, value: T) -> &mut Self {
        self.key(key);
        enc(&mut self.buf, value);
        self
    }

    pub fn null(&mut self, key: &str) -> &mut Self {
        self.field(key, encode_null, ())
    }
    pub fn bool(&mut self, key: &str, value: bool) -> &mut Self {
        self.field(key, encode_bool, value)
    }
    pub fn number(&mut self, key: &str, value: f64) -> &mut Self {
        self.field(key, encode_number, value)
    }
    pub fn string(&mut self, key: &str, value: &str) -> &mut Self {
        self.field(key, encode_str, value)
    }
    pub fn object(&mut self, key: &str) -> Object<'_> {
        self.key(key);
        Object::new(self.buf)
    }
    pub fn array(&mut self, key: &str) -> Array<'_> {
        self.key(key);
        Array::new(self.buf)
    }
}

impl Drop for Object<'_> {
    fn drop(&mut self) {
        self.buf.push('}')
    }
}

pub struct Array<'a> {
    buf: &'a mut String,
    first: bool,
}

impl<'a> Array<'a> {
    fn new(buf: &'a mut String) -> Self {
        buf.push('[');
        Array { buf, first: true }
    }
    fn comma(&mut self) {
        if !self.first {
            self.buf.push(',');
        }
        self.first = false;
    }
    fn element<T, F: FnOnce(&mut String, T)>(&mut self, enc: F, value: T) -> &mut Self {
        self.comma();
        enc(&mut self.buf, value);
        self
    }

    pub fn null(&mut self) -> &mut Self {
        self.element(encode_null, ())
    }
    pub fn bool(&mut self, value: bool) -> &mut Self {
        self.element(encode_bool, value)
    }
    pub fn number(&mut self, value: f64) -> &mut Self {
        self.element(encode_number, value)
    }
    pub fn string(&mut self, value: &str) -> &mut Self {
        self.element(encode_str, value)
    }
    pub fn object(&mut self) -> Object<'_> {
        self.comma();
        Object::new(self.buf)
    }
    pub fn array(&mut self) -> Array<'_> {
        self.comma();
        Array::new(self.buf)
    }
}

impl Drop for Array<'_> {
    fn drop(&mut self) {
        self.buf.push(']')
    }
}

fn encode_null(buf: &mut String, (): ()) {
    buf.push_str("null")
}
fn encode_bool(buf: &mut String, value: bool) {
    buf.push_str(if value { "true" } else { "false" })
}
fn encode_number(buf: &mut String, number: f64) {
    use std::fmt::Write;
    let _ = write!(buf, "{}", number);
}
fn encode_str(buf: &mut String, s: &str) {
    buf.reserve(s.len() + 2);
    buf.push('\"');
    for c in s.chars() {
        match c {
            '\\' | '"' => push_escape(buf, c),
            '\n' => push_escape(buf, 'n'),
            '\r' => push_escape(buf, 'r'),
            '\t' => push_escape(buf, 't'),
            c if c.is_control() => {
                buf.extend(c.escape_unicode().filter(|c| !matches!(c, '{' | '}')));
            }
            c => buf.push(c),
        }
    }
    buf.push('\"');

    fn push_escape(buf: &mut String, c: char) {
        buf.push('\\');
        buf.push(c);
    }
}
