#![allow(non_camel_case_types)]

pub type Map<'a, T> = indexmap::IndexMap<std::borrow::Cow<'a, str>, T>; // Vec<(String, T)>;

pub mod option;
pub mod config;
pub mod data;
pub mod codegen;
