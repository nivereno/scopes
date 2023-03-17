use std::{cell::RefCell, collections::HashSet};

use crate::{types::Type, all_types::All_types};


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
thread_local!(static qualifys: RefCell<HashSet<Box<QualifyType<'static>>>> = RefCell::new(HashSet::new()));

//------------------------------------------------------------------------------
#[derive(Clone)]
pub struct QualifyType<'a> {
    T: Type,
    this: All_types<'a>,
    mask: u32,
    qualifiers: Vec<Option<&'a Qualifier>>,
    prehash: u64
}
impl std::hash::Hash for QualifyType<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.prehash.hash(state);
    }
}
impl PartialEq for QualifyType<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.T == other.T && self.mask == other.mask && self.qualifiers == other.qualifiers
    }
}
impl Eq for QualifyType<'_> {
}
impl QualifyType<'_> {
    pub fn kind(&self) -> usize {
        todo!() //wrong
    }
    pub fn new<'a>(_T: All_types<'a>, _qualifiers: &Vec<Option<&'a Qualifier>>) -> QualifyType<'a> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut _prehash = 0;
        let mut _mask = 0;
        let mut quals: Vec<Option<&Qualifier>> = vec!();
        for (i, q) in _qualifiers.iter().enumerate() {
            todo!()
        }

        return QualifyType { T: Type::new(crate::types::TypeKind::TK_Qualify), this: _T, mask: _mask, qualifiers: quals, prehash: _prehash }
    }
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
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum QualifierKind {
    QK_Refer,
    QK_Unique,
    QK_View,
    QK_Mutate,
    QK_Key,
    QualifierCount
}
enum QualifierMask {
    QM_UniquenessTags = (1 << QualifierKind::QK_View as u64) | (1 << QualifierKind::QK_Unique as u64) | (1 << QualifierKind::QK_Mutate as u64),
    QM_Annotations = 1 << QualifierKind::QK_Key as u64
}
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Qualifier {
    _kind: QualifierKind
}

impl  Qualifier {
    pub fn new(kind: QualifierKind) -> Qualifier {
        return Qualifier { _kind: kind }
    }

    pub fn kind(&self) -> usize {
        todo!()
    }
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

*/
pub fn qualify<'a>(T: All_types<'a>, qualifiers: Vec<&Qualifier>) -> All_types<'a> {
    if qualifiers.is_empty() {
        return T
    }
    let mut quals: Vec<Option<&Qualifier>> = vec![None; QualifierKind::QualifierCount as usize]; //A bit weird originally initialized to null pointers
    if let All_types::qualify_type(T) = T {
        for i in 0..QualifierKind::QualifierCount as usize {
            quals[i] = T.qualifiers[i];
        }
        //type = T.T
    } 
    for q in qualifiers {
        quals[q.kind() as usize] = Some(q);
    }
    return _qualify(T, &quals)
}
pub fn _qualify<'a>(T: All_types, quals: &Vec<Option<&Qualifier>>) -> All_types<'a> {
    todo!()
    let key = Box::new(QualifyType::new(T, quals));
    qualifys.with(|qtypes| {
        let qtypes = qtypes.borrow_mut();
        if let Some(qt) = qtypes.get(&key) {
            return All_types::qualify_type(qt)
        }
        qtypes.insert(key.clone());
        return All_types::qualify_type(qtypes.get(&key).unwrap());
    });
    /*QualifyType key(type, quals);
    auto it = qualifys.find(&key);
    if (it != qualifys.end())
        return *it;
    auto result = new QualifyType(type, quals);
    qualifys.insert(result);
    return result;*/
    todo!()
}
pub fn copy_qualifiers<'a>(T: All_types<'a>, from: All_types) -> All_types<'a> {
    if let All_types::qualify_type(from) = from {
        return _qualify(T, &from.qualifiers)
    }
    return T
}

pub fn get_qualifier(T: All_types) -> &Qualifier {
    if let All_types::qualify_type(T) = T {
        //does and assert originaly so unwrap seems apt
        return T.qualifiers[T.kind() as usize].unwrap();
    }
    panic!()
}
pub fn find_qualifier(T: All_types) -> Option<&Qualifier> {
    if let All_types::qualify_type(T) = T {
        return T.qualifiers[T.kind() as usize];
    }
    return None
}
pub fn has_qualifiers(T: All_types, mask: u32) -> bool {
    if let All_types::qualify_type(T) = T {
        return (T.mask & mask) == mask
    }
    return false
}
pub fn strip_qualifiers(T: All_types, mask: u32) -> All_types {
    if let All_types::qualify_type(qt) = T {
        if qt.mask & mask == 0 {
            return T
        }
        let outmask: u32 = 0;
        todo!()
    }
    return T
}
pub fn strip_qualifier(T: All_types, kind: QualifierKind) -> All_types {
    strip_qualifiers(T, 1 << kind as u32)
}
/*

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