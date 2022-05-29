use bincode;
use chrono::prelude::*;
use serde::{Deserialize,Serialize};
pub fn my_serialize<T:?Sized>(value:&T)->Vec<u8>
where T:Serialize,
{
    let serialize = bincode::serialize(value).unwrap();
    serialize
}

pub fn my_deserialize<'a, T>(bytes:&'a[u8]) -> T
where T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
//ceshi
#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
struct Point{
    x:i32,
    y:i32,
}
#[cfg(test)]
mod tests{
    //use super::*;
    use crate::coder::Point;
    use crate::coder::{my_deserialize,my_serialize};
    #[test]
    fn coder_works(){
        let p = Point{x:1,y:1};
        let se = my_serialize(&p);
        let de:Point = my_deserialize(&se);
        assert_eq!(p,de);
    }
}