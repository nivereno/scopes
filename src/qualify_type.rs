use std::{cell::RefCell, collections::HashSet};

use crate::types::Type;


/*

//------------------------------------------------------------------------------
// QUALIFIER
//------------------------------------------------------------------------------

QualifierKind Qualifier::kind() const { return _kind; }

Qualifier::Qualifier(QualifierKind kind)
    : _kind(kind) {}

//------------------------------------------------------------------------------
// QUALIFY
//------------------------------------------------------------------------------



struct KeyEqual {
    bool operator()( const QualifyType *lhs, const QualifyType *rhs ) const {
        if (lhs->type != rhs->type) return false;
        if (lhs->mask != rhs->mask) return false;
        for (int i = 0; i < QualifierCount; ++i) {
            if (lhs->qualifiers[i] != rhs->qualifiers[i])
                return false;
        }
        return true;
    }
};
} // namespace QualifySet
*/
thread_local!(static qualifys: RefCell<HashSet<&'static QualifyType<'static>>> = RefCell::new(HashSet::new()));

//------------------------------------------------------------------------------
struct QualifyType<'a> {
    T: &'a Type,
    mask: u32,
    //qualifier
}
/*

QualifyType::QualifyType(const Type *_type, const Qualifier * const *_qualifiers)
    : Type(TK_Qualify), type(_type), mask(0) {
    std::size_t h = std::hash<const Type *>{}(type);
    for (int i = 0; i < QualifierCount; ++i) {
        qualifiers[i] = _qualifiers[i];
        if (_qualifiers[i]) {
            mask |= 1 << i;
            h = hash2(h, std::hash<const Qualifier *>{}(qualifiers[i]));
        }
    }
    assert(mask);
    prehash = h;
}

//------------------------------------------------------------------------------
*/
enum QualifierKind {

}
enum QualifierMask {

}
pub struct Qualifier {
    _kind: QualifierKind
}
/*

static const Type *_qualify(const Type *type, const Qualifier * const * quals) {
    QualifyType key(type, quals);
    auto it = qualifys.find(&key);
    if (it != qualifys.end())
        return *it;
    auto result = new QualifyType(type, quals);
    qualifys.insert(result);
    return result;
}

const Type *qualify(const Type *type, const Qualifiers &qualifiers) {
    if (qualifiers.empty())
        return type;
    const Qualifier *quals[QualifierCount];
    if (isa<QualifyType>(type)) {
        auto qt = cast<QualifyType>(type);
        for (int i = 0; i < QualifierCount; ++i) {
            quals[i] = qt->qualifiers[i];
        }
        type = qt->type;
    } else {
        std::memset(quals, 0, sizeof(quals));
    }
    for (auto q : qualifiers) {
        auto kind = q->kind();
        assert(kind < QualifierCount);
        quals[kind] = q;
    }
    return _qualify(type, quals);
}

const Type *copy_qualifiers(const Type *type, const Type *from) {
    auto qt = dyn_cast<QualifyType>(from);
    if (qt) {
        return _qualify(type, qt->qualifiers);
    }
    return type;
}

const Qualifier *get_qualifier(const Type *type, QualifierKind kind) {
    auto qt = cast<QualifyType>(type);
    assert(kind < QualifierCount);
    auto q = qt->qualifiers[kind];
    assert(q);
    return q;
}

const Qualifier *find_qualifier(const Type *type, QualifierKind kind) {
    if (isa<QualifyType>(type)) {
        auto qt = cast<QualifyType>(type);
        assert(kind < QualifierCount);
        return qt->qualifiers[kind];
    }
    return nullptr;
}

bool has_qualifiers(const Type *T, uint32_t mask) {
    if (isa<QualifyType>(T)) {
        auto qt = cast<QualifyType>(T);
        return ((qt->mask & mask) == mask);
    }
    return false;
}

const Type *strip_qualifiers(const Type *T, uint32_t mask) {
    if (isa<QualifyType>(T)) {
        auto qt = cast<QualifyType>(T);
        if (!(qt->mask & mask))
            return T;
        uint32_t outmask = 0;
        const Qualifier *quals[QualifierCount];
        for (int i = 0; i < QualifierCount; ++i) {
            if ((mask & (1 << i)) || !qt->qualifiers[i]) {
                quals[i] = nullptr;
            } else {
                quals[i] = qt->qualifiers[i];
                outmask |= (1 << i);
            }
        }
        if (!outmask)
            return qt->type;
        else
            return _qualify(qt->type, quals);
    }
    return T;
}

const Type *strip_qualifier(const Type *T, QualifierKind kind) {
    return strip_qualifiers(T, 1 << kind);
}
*/