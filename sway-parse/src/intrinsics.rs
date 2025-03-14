#[derive(Eq, PartialEq)]
pub enum Intrinsic {
    IsReferenceType,
    SizeOf,
    SizeOfVal,
}

impl Intrinsic {
    pub fn try_from_str(raw: &str) -> Option<Intrinsic> {
        use Intrinsic::*;
        Some(match raw {
            "__is_reference_type" => IsReferenceType,
            "__size_of" => SizeOf,
            "__size_of_val" => SizeOfVal,
            _ => return None,
        })
    }
}
