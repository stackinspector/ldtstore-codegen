use std::borrow::Cow;

/// deprecated
pub /* const */ fn map2<T, U, V, F>(opt: Option<T>, optb: Option<U>, f: F) -> Option<V>
where
    F: /* ~const */ FnOnce(T, U) -> V,
    // F: ~const std::marker::Destruct,
{
    if let Some(opt) = opt {
        if let Some(optb) = optb {
            Some(f(opt, optb))
        } else {
            None
        }
    } else {
        None
    }
}

/// deprecated
#[inline]
pub /* const */ fn from_bool_and<T>(b: bool, x: T) -> Option<T>
where
    // T: ~const Destruct,
{
    b.then_some(x)
}

/// deprecated
#[inline]
pub /* const */ fn from_bool_and_then<T, F>(b: bool, f: F) -> Option<T>
where
    F: /* ~const */ FnOnce() -> T,
    // F: ~const std::marker::Destruct,
{
    b.then(f)
}

/// deprecated
pub fn filter_none<T>(opts: Vec<Option<T>>) -> Vec<T> {
    let mut res = Vec::new();
    for opt in opts {
        if let Some(opt) = opt {
            res.push(opt)
        }
    }
    res
}

/// deprecated
pub fn filter_none_attr<T, U>(attrs: Vec<(T, Option<U>)>) -> Vec<(T, U)> {
    let mut res = Vec::new();
    for (k, v) in attrs {
        if let Some(v) = v {
            res.push((k, v))
        }
    }
    res
}

#[inline]
pub /* const */ fn from_str<'a>(s: Cow<'a, str>) -> Option<Cow<'a, str>> {
    (s.len() == 0).then_some(s)
}


#[inline]
pub const fn from_bool(b: bool) -> Option<bool> {
    // (!b).then_some(b)
    if b { Some(true) } else { None }
}

#[inline]
pub /* const */ fn into_str<'a>(opt: Option<Cow<'a, str>>) -> Cow<'a, str> {
    opt.unwrap_or_default()
}

#[inline]
pub /* const */ fn into_bool(opt: Option<bool>) -> bool {
    opt.unwrap_or(false)
}