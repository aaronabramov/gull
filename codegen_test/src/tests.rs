use crate::generated::{OpFetch, OpInline, OperationsEnum};
use anyhow::Result;
use k9::*;
use serde;

pub enum E {
    One(Vec<usize>),
    Two(usize),
}

impl E {
    pub fn get_variant_name(&self) -> &'static str {
        match self {
            E::One(_) => "One",
            E::Two(_) => "Two",
        }
    }

    pub fn get_variant_idx(&self) -> u32 {
        match self {
            E::One(_) => 0,
            E::Two(_) => 1,
        }
    }
}

// #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
// const _: () = {
//     #[automatically_derived]
//     impl serde::Serialize for E {
//         fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
//         where
//             __S: serde::Serializer,
//         {
//             let mut map = __serializer.serialize_map(None)?;

//             serde::Serializer::serialize_newtype_variant(
//                 __serializer,
//                 "E",
//                 self.get_variant_idx(),
//                 self.get_variant_name(),
//                 &4,
//             );

//             // serde::ser::SerializeMap::serialize_entry(&mut map, 0)?;
//             let mut map = serde::Serializer::serialize_map(__serializer, None)?;
//             serde::ser::SerializeMap::end(map)

//             // // map.serialize_entry(1, 2);
//             // match *self {
//             //     E::One(ref __field0) => serde::Serializer::serialize_newtype_variant(
//             //         __serializer,
//             //         "E",
//             //         0u32,
//             //         "One",
//             //         __field0,
//             //     ),
//             //     E::Two(ref __field0) => serde::Serializer::serialize_newtype_variant(
//             //         __serializer,
//             //         "E",
//             //         1u32,
//             //         "Two",
//             //         __field0,
//             //     ),
//             // }
//         }
//     }
// };

#[test]
fn serialization() -> Result<()> {
    let e = vec![E::One(vec![2]), E::Two(5)];

    // let s = serde_json::to_string_pretty(&e)?;

    // assert_equal!(MultilineString(s), MultilineString(String::new()));

    Ok(())
}
