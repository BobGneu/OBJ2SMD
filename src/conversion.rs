use std::str::SplitWhitespace;
use obj_file::Float3;

pub fn str_to_f64(value: Option<&str>) -> f64 {
    return value.unwrap().parse::<f64>().unwrap();
}

pub fn token_to_float3(mut iterator: SplitWhitespace) -> Float3 {
    return Float3 {x:str_to_f64(iterator.next()), y: str_to_f64(iterator.next()), z: str_to_f64(iterator.next())};
}