#![allow(non_camel_case_types)]

pub type Map<T> = indexmap::IndexMap<String, T>; // Vec<(String, T)>;

pub mod option;
pub mod config;
pub mod data;
