use crate::dom::AttributeValue;

pub trait ToAttributeValue {
    fn to_attribute_value(self) -> AttributeValue;
}

impl ToAttributeValue for AttributeValue {
    #[inline(always)]
    fn to_attribute_value(self) -> Self {
        self
    }
}

impl ToAttributeValue for () {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::ImplicitTrue
    }
}

impl<N: ToAttributeValue> ToAttributeValue for Option<N> {
    fn to_attribute_value(self) -> AttributeValue {
        match self {
            None => AttributeValue::ImplicitFalse,
            Some(n) => n.to_attribute_value(),
        }
    }
}

impl ToAttributeValue for &'static str {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::Text(self)
    }
}

impl ToAttributeValue for i64 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::SignedInteger(self)
    }
}

impl ToAttributeValue for i32 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::SignedInteger(self as i64)
    }
}

impl ToAttributeValue for i16 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::SignedInteger(self as i64)
    }
}

impl ToAttributeValue for i8 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::SignedInteger(self as i64)
    }
}

impl ToAttributeValue for u64 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::UnsignedInteger(self)
    }
}

impl ToAttributeValue for u32 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::UnsignedInteger(self as u64)
    }
}

impl ToAttributeValue for u16 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::UnsignedInteger(self as u64)
    }
}

impl ToAttributeValue for u8 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::UnsignedInteger(self as u64)
    }
}

impl ToAttributeValue for isize {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::SignedInteger(self as i64)
    }
}

impl ToAttributeValue for usize {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::UnsignedInteger(self as u64)
    }
}

impl ToAttributeValue for f64 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::Float(self)
    }
}

impl ToAttributeValue for f32 {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::Float(self as f64)
    }
}

impl ToAttributeValue for bool {
    fn to_attribute_value(self) -> AttributeValue {
        if self {
            AttributeValue::ImplicitTrue
        } else {
            AttributeValue::ImplicitFalse
        }
    }
}
