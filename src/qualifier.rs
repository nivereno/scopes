use crate::types::Type;



pub fn has_qualifier(T: &Type) -> bool {
    todo!()
}
pub fn try_qualifier(T: &Type) -> Option<&Type> {
 todo!()
}
pub fn get_qualifier(T: &Type) ->  &Type {
    todo!()
}
pub fn strip_qualifier(T: &Type) -> &Type {
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