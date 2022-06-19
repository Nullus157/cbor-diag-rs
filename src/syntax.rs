mod tags;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// How many additional bytes are used to encode this integer (in bits).
///
/// See [RFC 7049 § 2][RFC 2].
///
/// [RFC 2]: https://tools.ietf.org/html/rfc7049#section-2
pub enum IntegerWidth {
    /// Parsed from CBOR diagnostic notation without an encoding indicator
    Unknown,
    /// For values <24 encoded directly in the additional data of the first byte
    Zero,
    /// One additional byte
    Eight,
    /// Two additional bytes
    Sixteen,
    /// Four additional bytes
    ThirtyTwo,
    /// Eight additional bytes
    SixtyFour,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// How many additional bytes are used to encode this float (in bits).
///
/// See [RFC 7049 § 2][RFC 2].
///
/// [RFC 2]: https://tools.ietf.org/html/rfc7049#section-2
pub enum FloatWidth {
    /// Parsed from CBOR diagnostic notation without an encoding indicator
    Unknown,
    /// Two additional bytes
    Sixteen,
    /// Four additional bytes
    ThirtyTwo,
    /// Eight additional bytes
    SixtyFour,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// A semantic tag for a CBOR data item.
///
/// See [RFC 7049 § 2.4: Table 3][RFC 2.4] and the [CBOR Tags IANA Registry][IANA]
///
/// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
/// [IANA]: https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml
pub struct Tag(pub u64);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// A "simple value" data item.
///
/// See [RFC 7049 § 2.3: Table 2][RFC 2.3].
///
/// [RFC 2.3]: https://tools.ietf.org/html/rfc7049#section-2.3
pub struct Simple(pub u8);

#[derive(Debug, Eq, PartialEq, Clone)]
/// A string of raw bytes with no direct attached meaning.
///
/// May be assigned a meaning by being enclosed in a [semantic tag](Tag).
///
/// See [RFC 7049 § 2.1: Major type 2][RFC 2.1].
///
/// [RFC 2.1]: https://tools.ietf.org/html/rfc7049#section-2.1
pub struct ByteString {
    /// The raw binary data in this byte string
    pub data: Vec<u8>,
    /// The bitwidth used for encoding the length
    pub bitwidth: IntegerWidth,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// A UTF-8 encoded text string.
///
/// May be assigned further meaning by being enclosed in a [semantic tag](Tag).
///
/// See [RFC 7049 § 2.1: Major type 3][RFC 2.1].
///
/// [RFC 2.1]: https://tools.ietf.org/html/rfc7049#section-2.1
pub struct TextString {
    /// The textual data in this text string
    pub data: String,
    /// The bitwidth used for encoding the length
    pub bitwidth: IntegerWidth,
}

#[derive(Debug, PartialEq, Clone)]
/// A CBOR data item.
///
/// See [RFC 7049 § 1.2: Data item][RFC 1.2].
///
/// [RFC 1.2]: https://tools.ietf.org/html/rfc7049#section-1.2
pub enum DataItem {
    /// An unsigned integer.
    ///
    /// See [RFC 7049 § 2.1: Major type 0][RFC 2.1].
    ///
    /// [RFC 2.1]: https://tools.ietf.org/html/rfc7049#section-2.1
    Integer {
        /// The value of this unsigned integer.
        value: u64,

        /// The bitwidth used for encoding this integer.
        bitwidth: IntegerWidth,
    },

    /// A negative integer.
    ///
    /// See [RFC 7049 § 2.1: Major type 0][RFC 2.1].
    ///
    /// [RFC 2.1]: https://tools.ietf.org/html/rfc7049#section-2.1
    Negative {
        /// The encoded value of this negative integer, the real value is `-1 -
        /// value` (requires use of `i128` for full range support).
        value: u64,

        /// The bitwidth used for encoding this integer.
        bitwidth: IntegerWidth,
    },

    /// A string of raw bytes with no direct attached meaning.
    ///
    /// See the docs for [`ByteString`] for more details.
    ByteString(ByteString),

    /// A UTF-8 encoded text string.
    ///
    /// See the docs for [`TextString`] for more details.
    TextString(TextString),

    /// A series of [`ByteString`] chunks encoded as an indefinite length byte
    /// string.
    ///
    /// See [RFC 7049 § 2.2.2][RFC 2.2.2].
    ///
    /// [RFC 2.2.2]: https://tools.ietf.org/html/rfc7049#section-2.2.2
    IndefiniteByteString(Vec<ByteString>),

    /// A series of [`TextString`] chunks encoded as an indefinite length text
    /// string.
    ///
    /// See [RFC 7049 § 2.2.2][RFC 2.2.2].
    ///
    /// [RFC 2.2.2]: https://tools.ietf.org/html/rfc7049#section-2.2.2
    IndefiniteTextString(Vec<TextString>),

    /// An array of data items.
    ///
    /// See [RFC 7049 § 2.1: Major type 4][RFC 2.1].
    ///
    /// [RFC 2.1]: https://tools.ietf.org/html/rfc7049#section-2.1
    Array {
        /// The data items in this array.
        data: Vec<DataItem>,

        /// The bitwidth used for encoding the array length.
        ///
        /// If has the value [`None`] then this array is encoded using the
        /// indefinite length form, see [RFC 7049 § 2.2.1][RFC 2.2.1].
        ///
        /// [RFC 2.2.1]: https://tools.ietf.org/html/rfc7049#section-2.2.1
        bitwidth: Option<IntegerWidth>,
    },

    /// A map of pairs of data items.
    ///
    /// See [RFC 7049 § 2.1: Major type 5][RFC 2.1].
    ///
    /// [RFC 2.1]: https://tools.ietf.org/html/rfc7049#section-2.1
    Map {
        /// The pairs of data items in this map.
        data: Vec<(DataItem, DataItem)>,

        /// The bitwidth used for encoding the map length.
        ///
        /// If has the value [`None`] then this map is encoded using the
        /// indefinite length form, see [RFC 7049 § 2.2.1][RFC 2.2.1].
        ///
        /// [RFC 2.2.1]: https://tools.ietf.org/html/rfc7049#section-2.2.1
        bitwidth: Option<IntegerWidth>,
    },

    /// Semantic tagging of another data item.
    ///
    /// See the docs for [`Tag`] for more details.
    Tag {
        /// The semantic tag to be applied to [`value`](#Tag.v.value.v).
        tag: Tag,

        /// The bitwidth used to encode the semantic tag.
        bitwidth: IntegerWidth,

        /// The data item which has the semantic tag applied to it.
        value: Box<DataItem>,
    },

    /// A floating point value.
    ///
    /// See [RFC 7049 § 2.3][RFC 2.3].
    ///
    /// [RFC 2.3]: https://tools.ietf.org/html/rfc7049#section-2.3
    Float {
        /// The floating point value.
        value: f64,

        /// The bitwidth used for encoding the value.
        bitwidth: FloatWidth,
    },

    /// A "simple value" data item.
    ///
    /// See the docs for [`Simple`] for more details.
    Simple(Simple),
}

impl Simple {
    /// The simple value "False", equivalent to [`false`](bool).
    ///
    /// Defined in [RFC 7049 § 2.3: Table 2][RFC 2.3].
    ///
    /// [RFC 2.3]: https://tools.ietf.org/html/rfc7049#section-2.3
    pub const FALSE: Simple = Simple(20);

    /// The simple value "True", equivalent to [`true`](bool).
    ///
    /// Defined in [RFC 7049 § 2.3: Table 2][RFC 2.3].
    ///
    /// [RFC 2.3]: https://tools.ietf.org/html/rfc7049#section-2.3
    pub const TRUE: Simple = Simple(21);

    /// The simple value "Null", equivalent to a generic [`None`].
    ///
    /// Defined in [RFC 7049 § 2.3: Table 2][RFC 2.3].
    ///
    /// [RFC 2.3]: https://tools.ietf.org/html/rfc7049#section-2.3
    pub const NULL: Simple = Simple(22);

    /// The simple value "Undefined value", not really equivalent to any Rust
    ///
    /// Defined in [RFC 7049 § 2.3: Table 2][RFC 2.3].
    ///
    /// [RFC 2.3]: https://tools.ietf.org/html/rfc7049#section-2.3
    /// concept.
    pub const UNDEFINED: Simple = Simple(23);
}
