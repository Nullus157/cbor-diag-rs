use crate::Tag;

impl Tag {
    /// A "Standard date/time string"; must only be applied to a [text
    /// string](DataItem::TextString) (or the [indefinite
    /// variant](DataItem::IndefiniteTextString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.1][RFC 2.4.1] for more details on how to interpret
    /// the string.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.1]: https://tools.ietf.org/html/rfc7049#section-2.4.1
    pub const DATETIME: Tag = Tag(0);

    /// An "Epoch-based date/time"; must only be applied to an [unsigned
    /// integer](DataItem::Integer), [negative integer](DataItem::Negative) or
    /// [floating point](DataItem::Float) data item.
    ///
    /// See [RFC 7049 § 2.4.1][RFC 2.4.1] for more details on how to interpret
    /// the value.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.1]: https://tools.ietf.org/html/rfc7049#section-2.4.1
    pub const EPOCH_DATETIME: Tag = Tag(1);

    /// A "positive bignum"; must only be applied to a [byte
    /// string](DataItem::ByteString) (or the [indefinite
    /// variant](DataItem::IndefiniteByteString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.2][RFC 2.4.2] for more details on how to interpret
    /// the bytes.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.2]: https://tools.ietf.org/html/rfc7049#section-2.4.2
    pub const POSITIVE_BIGNUM: Tag = Tag(2);

    /// A "negative bignum"; must only be applied to a [byte
    /// string](DataItem::ByteString) (or the [indefinite
    /// variant](DataItem::IndefiniteByteString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.2][RFC 2.4.2] for more details on how to interpret
    /// the bytes.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.2]: https://tools.ietf.org/html/rfc7049#section-2.4.2
    pub const NEGATIVE_BIGNUM: Tag = Tag(3);

    /// A "decimal fraction"; must only be applied to an
    /// [array](DataItem::Array) containing exactly two data items, the first
    /// must be either a [unsigned integer](DataItem::Integer) or [negative
    /// integer](DataItem::Negative), the second can be either of those or
    /// additionally a [positive](Tag::POSITIVE_BIGNUM) or
    /// [negative](Tag::NEGATIVE_BIGNUM) bignum.
    ///
    /// See [RFC 7049 § 2.4.3][RFC 2.4.3] for more details on how to interpret
    /// the values.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.3]: https://tools.ietf.org/html/rfc7049#section-2.4.3
    pub const DECIMAL_FRACTION: Tag = Tag(4);

    /// A "bigfloat"; must only be applied to an [array](DataItem::Array)
    /// containing exactly two data items, the first must be either a [unsigned
    /// integer](DataItem::Integer) or [negative integer](DataItem::Negative),
    /// the second can be either of those or additionally a
    /// [positive](Tag::POSITIVE_BIGNUM) or [negative](Tag::NEGATIVE_BIGNUM)
    /// bignum.
    ///
    /// See [RFC 7049 § 2.4.3][RFC 2.4.3] for more details on how to interpret
    /// the values.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.3]: https://tools.ietf.org/html/rfc7049#section-2.4.3
    pub const BIGFLOAT: Tag = Tag(5);

    /// Sets the expected encoding of any [byte strings](DataItem::ByteString)
    /// contained in the data item to be "base64url"; can be applied to any sort
    /// of data item.
    ///
    /// See [RFC 7049 § 2.4.4.2][RFC 2.4.4.2] for more details on how the
    /// expected encoding is applied.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.2]: https://tools.ietf.org/html/rfc7049#section-2.4.4.2
    pub const ENCODED_BASE64URL: Tag = Tag(21);

    /// Sets the expected encoding of any [byte strings](DataItem::ByteString)
    /// contained in the data item to be "base64"; can be applied to any sort of
    /// data item.
    ///
    /// See [RFC 7049 § 2.4.4.2][RFC 2.4.4.2] for more details on how the
    /// expected encoding is applied.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.2]: https://tools.ietf.org/html/rfc7049#section-2.4.4.2
    pub const ENCODED_BASE64: Tag = Tag(22);

    /// Sets the expected encoding of any [byte strings](DataItem::ByteString)
    /// contained in the data item to be "base16"; can be applied to any sort of
    /// data item.
    ///
    /// See [RFC 7049 § 2.4.4.2][RFC 2.4.4.2] for more details on how the
    /// expected encoding is applied.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.2]: https://tools.ietf.org/html/rfc7049#section-2.4.4.2
    pub const ENCODED_BASE16: Tag = Tag(23);

    /// Marks this item as being an encoded CBOR data item; must only be applied
    /// to a [byte string](DataItem::ByteString) (or the [indefinite
    /// variant](DataItem::IndefiniteByteString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.4.1][RFC 2.4.4.1] for more details on what this
    /// means.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.1]: https://tools.ietf.org/html/rfc7049#section-2.4.4.1
    pub const ENCODED_CBOR: Tag = Tag(24);

    /// Marks this item as being a valid URI; must only be applied
    /// to a [text string](DataItem::TextString) (or the [indefinite
    /// variant](DataItem::IndefiniteTextString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.4.3][RFC 2.4.4.3] for more details on what this
    /// means.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.3]: https://tools.ietf.org/html/rfc7049#section-2.4.4.3
    pub const URI: Tag = Tag(32);

    /// Marks this item as being a base64url encoded string; must only be
    /// applied to a [text string](DataItem::TextString) (or the [indefinite
    /// variant](DataItem::IndefiniteTextString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.4.3][RFC 2.4.4.3] for more details on what this
    /// means.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.3]: https://tools.ietf.org/html/rfc7049#section-2.4.4.3
    pub const BASE64URL: Tag = Tag(33);

    /// Marks this item as being a base64 encoded string; must only be applied
    /// to a [text string](DataItem::TextString) (or the [indefinite
    /// variant](DataItem::IndefiniteTextString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.4.3][RFC 2.4.4.3] for more details on what this
    /// means.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.3]: https://tools.ietf.org/html/rfc7049#section-2.4.4.3
    pub const BASE64: Tag = Tag(34);

    /// Marks this item as being a regex; must only be applied to a [text
    /// string](DataItem::TextString) (or the [indefinite
    /// variant](DataItem::IndefiniteTextString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.4.3][RFC 2.4.4.3] for more details on what this
    /// means.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.3]: https://tools.ietf.org/html/rfc7049#section-2.4.4.3
    pub const REGEX: Tag = Tag(35);

    /// Marks this item as being a MIME message; must only be applied to a [text
    /// string](DataItem::TextString) (or the [indefinite
    /// variant](DataItem::IndefiniteTextString) of) data item.
    ///
    /// See [RFC 7049 § 2.4.4.3][RFC 2.4.4.3] for more details on what this
    /// means.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.4.3]: https://tools.ietf.org/html/rfc7049#section-2.4.4.3
    pub const MIME: Tag = Tag(36);

    /// Marks this item as being a binary UUID; must only be applied to a [byte
    /// string](DataItem::ByteString) (or the [indefinite
    /// variant](DataItem::IndefiniteByteString) of) data item.
    ///
    /// See [the spec][UuidSpec] for more details on what this means.
    ///
    /// Defined in [non-RFC specification][UuidSpec].
    ///
    /// [UuidSpec]: https://github.com/lucas-clemente/cbor-specs/blob/master/uuid.md
    pub const UUID: Tag = Tag(37);

    /// Marks this item as being an encoded CBOR sequence; must only be applied
    /// to a [byte string](DataItem::ByteString) (or the [indefinite
    /// variant](DataItem::IndefiniteByteString) of) data item.
    ///
    /// See [RFC 8742][] for more details on what this
    /// means.
    ///
    /// Defined in [draft-bormann-cbor-notable-tags § 2.1][draft-2.1].
    ///
    /// [RFC 8742]: https://tools.ietf.org/html/rfc8742
    /// [draft-2.1]: https://www.ietf.org/archive/id/draft-bormann-cbor-notable-tags-06.html#name-tags-related-to-those-defin
    pub const ENCODED_CBOR_SEQ: Tag = Tag(63);

    /// Number of days since the epoch date 1970-01-01; must only be applied to an [unsigned
    /// integer](DataItem::Integer) or [negative integer](DataItem::Negative) data item.
    ///
    /// Defined in [RFC 8943][].
    ///
    /// [RFC 8943]: https://tools.ietf.org/html/rfc8943
    pub const EPOCH_DATE: Tag = Tag(100);

    /// Marks this item as being a Network Address (IPv4 or IPv6 or MAC
    /// Address); must only be applied to a [byte string](DataItem::ByteString)
    /// (or the [indefinite variant](DataItem::IndefiniteByteString) of) data
    /// item.
    ///
    /// See [the spec][NetworkAddressSpec] for more details on what this means.
    ///
    /// Defined in [non-RFC specification][NetworkAddressSpec].
    ///
    /// [NetworkAddressSpec]: http://www.employees.org/~ravir/cbor-network.txt
    pub const NETWORK_ADDRESS: Tag = Tag(260);

    /// A "Standard date string"; must only be applied to a [text
    /// string](DataItem::TextString) (or the [indefinite
    /// variant](DataItem::IndefiniteTextString) of) data item.
    ///
    /// Defined in [RFC 8943][], uses format from [RFC 3339][].
    ///
    /// [RFC 8943]: https://tools.ietf.org/html/rfc8943
    /// [RFC 3339]: https://tools.ietf.org/html/rfc3339
    pub const DATE: Tag = Tag(1004);

    /// Marks this item as being CBOR, a no-op; can be applied to any type of
    /// data item.
    ///
    /// See [RFC 7049 § 2.4.5][RFC 2.4.5] for more details on why this is
    /// useful.
    ///
    /// Defined in [RFC 7049 § 2.4: Table 3][RFC 2.4].
    ///
    /// [RFC 2.4]: https://tools.ietf.org/html/rfc7049#section-2.4
    /// [RFC 2.4.5]: https://tools.ietf.org/html/rfc7049#section-2.4.5
    pub const SELF_DESCRIBE_CBOR: Tag = Tag(55799);
}
