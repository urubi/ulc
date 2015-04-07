//! مكتبة لتكتيل البيانات
//! ================
//! 
//! مصطلحات
//! 1. نوع: Type
//! 2. سمة: Trait
//! 3. قاموس: Hash Table
//! 4. دالة: Function
//! 5. وحدة: Module
//! 6. سائب النوع: Generic Type
//! 7. كتلة: Binary Blob
#![allow(trivial_numeric_casts)]
#![allow(trivial_casts)]
#![feature(core)]
#![feature(io)]
#![feature(convert)]
#![feature(collections)]

#[macro_use]
extern crate ulc22;

pub mod utilities;
pub mod unsigned;
pub mod blob;
pub mod archive;

