#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IntegerWidth {
    /// When parsed from CBOR diagnostic notation without an encoding indicator.
    Unknown,
    /// For values <24 encoded in the additional data
    Zero,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FloatWidth {
    /// When parsed from CBOR diagnostic notation without an encoding indicator.
    Unknown,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Tag(pub u64);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Simple(pub u8);

#[derive(Debug, PartialEq, Clone)]
pub struct ByteString {
    pub data: Vec<u8>,
    /// The bitwidth used for encoding the length
    pub bitwidth: IntegerWidth,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextString {
    pub data: String,
    /// The bitwidth used for encoding the length
    pub bitwidth: IntegerWidth,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer {
        value: u64,
        bitwidth: IntegerWidth,
    },

    Negative {
        value: u64,
        bitwidth: IntegerWidth,
    },

    ByteString(ByteString),

    TextString(TextString),

    IndefiniteByteString(Vec<ByteString>),

    IndefiniteTextString(Vec<TextString>),

    Array {
        data: Vec<Value>,
        /// The bitwidth used for encoding the length, if none then indefinite
        /// length
        bitwidth: Option<IntegerWidth>,
    },

    Map {
        data: Vec<(Value, Value)>,
        /// The bitwidth used for encoding the length, if none then indefinite
        /// length
        bitwidth: Option<IntegerWidth>,
    },

    Tag {
        tag: Tag,
        bitwidth: IntegerWidth,
        value: Box<Value>,
    },

    Float {
        value: f64,
        bitwidth: FloatWidth,
    },

    Simple(Simple),
}

impl Simple {
    pub const FALSE: Simple = Simple(20);
    pub const TRUE: Simple = Simple(21);
    pub const NULL: Simple = Simple(22);
    pub const UNDEFINED: Simple = Simple(23);
}

impl Tag {
    pub const DATETIME: Tag = Tag(0);
    pub const EPOCH_DATETIME: Tag = Tag(1);
    pub const POSITIVE_BIGNUM: Tag = Tag(2);
    pub const NEGATIVE_BIGNUM: Tag = Tag(3);
    pub const DECIMAL_FRACTION: Tag = Tag(4);
    pub const BIGFLOAT: Tag = Tag(5);
    pub const ENCODED_BASE64URL: Tag = Tag(21);
    pub const ENCODED_BASE64: Tag = Tag(22);
    pub const ENCODED_BASE16: Tag = Tag(23);
    pub const ENCODED_CBOR: Tag = Tag(24);
    pub const URI: Tag = Tag(32);
    pub const BASE64URL: Tag = Tag(33);
    pub const BASE64: Tag = Tag(34);
    pub const REGEX: Tag = Tag(35);
    pub const MIME: Tag = Tag(36);
    pub const SELF_DESCRIBE_CBOR: Tag = Tag(55799);
}

impl Value {
    pub fn simple(value: u8) -> Value {
        Value::Simple(Simple(value))
    }
}
