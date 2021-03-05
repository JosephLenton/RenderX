#[derive(Clone, Debug)]
pub enum AttributeValue {
    /// This is for when the attribute is set,
    /// but we only find out when looking at the value,
    /// that the attribute shouldn't be used at all.
    ImplicitFalse,
    /// This is for keys which don't have a value.
    /// i.e. The `disabled` in `<button disabled>`.
    ImplicitTrue,
    Text(&'static str),
    UnsignedInteger(u64),
    SignedInteger(i64),
    Float(f64),
}
