use serde::{Serialize, Deserialize};
// use wasm_bindgen::convert::FromWasmAbi;
// use wasm_bindgen::describe::WasmDescribe;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32, 
    pub y: i32,
}

// impl FromWasmAbi for Point {
//     type Abi = [i32; 2]; // Simple flat memory layout

//     unsafe fn from_abi(abi: Self::Abi) -> Self {
//         Point {
//             x: abi[0],
//             y: abi[1],
//         }
//     }
// }

// impl WasmDescribe for Point {
    
// }
