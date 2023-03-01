use crate::{types::Type, all_types::All_types, qualify_type::find_qualifier};



pub fn has_qualifier(T: All_types) -> bool {
    if let Some(q) = find_qualifier(T) {
        return true
    }
    return false
}
pub fn try_qualifier(T: All_types) -> Option<All_types> {
    if let Some(q) = find_qualifier(T) {
        return Some(All_types::qualifier(q))
    }
    return None
}
pub fn get_qualifier(T: All_types) ->  All_types {
    let q = crate::qualify_type::get_qualifier(T);
    return All_types::qualifier(q)
}
pub fn strip_qualifier(T: All_types) -> All_types {
    //return crate::qualify_type::strip_qualifier(T, )
    todo!()
}


/*
template<typename T>
static bool has_qualifier(const Type *type) {
    return find_qualifier(type, (QualifierKind)T::Kind) != nullptr;
}

template<typename T>
static const T *try_qualifier(const Type *type) {
    auto q = find_qualifier(type, (QualifierKind)T::Kind);
    if (!q) return nullptr;
    return cast<T>(q);
}

template<typename T>
static const T *get_qualifier(const Type *type) {
    auto q = get_qualifier(type, (QualifierKind)T::Kind);
    return cast<T>(q);
}

template<typename T>
static const Type *strip_qualifier(const Type *type) {
    return strip_qualifier(type, (QualifierKind)T::Kind);
}*/