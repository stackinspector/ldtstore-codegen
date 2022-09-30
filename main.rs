use std::fs;
use serde::de::DeserializeOwned;
use ldtstore_codegen::config::*;

fn load_base<T: DeserializeOwned>(path: &str) -> T {
    serde_yaml::from_reader(fs::File::open(path).unwrap()).unwrap()
}

fn main() {
    println!("{:?}", load_base::<Vec<Side>>(r"D:\root\repo\public\ldtstore-homepage\public.sides.yml"));
    println!("{:?}", load_base::<TileColumns>(r"D:\root\repo\public\ldtstore-homepage\index.major.yml"));
    println!("{:?}", load_base::<Vec<Side>>(r"D:\root\repo\public\ldtstore-homepage\index.sides.yml"));
    println!("{:?}", load_base::<TileGrids>(r"D:\root\repo\public\ldtstore-homepage\ldtools\index.major.yml"));
    println!("{:?}", load_base::<Vec<Side>>(r"D:\root\repo\public\ldtstore-homepage\ldtools\index.sides.yml"));
    println!("{:?}", load_base::<Vec<ToolGroup>>(r"D:\root\repo\public\ldtstore-homepage\ldtools\index.tools.yml"));
    println!("{:?}", load_base::<Category>(r"D:\root\repo\public\ldtstore-homepage\ldtools\index.category.yml"));
}
