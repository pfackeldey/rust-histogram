use std::ops::{Add, AddAssign};

#[derive(Debug, Clone)]
pub enum StorageType {
    Double,
    Int,
    Weight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Storage {
    Double(f32),
    Int(i32),
    Weight((f32, f32)),
}

impl Add for Storage {
    type Output = Storage;

    fn add(self, other: Storage) -> Storage {
        match (self, other) {
            (Storage::Double(a), Storage::Double(b)) => Storage::Double(a + b),
            (Storage::Int(a), Storage::Int(b)) => Storage::Int(a + b),
            (Storage::Weight((a, b)), Storage::Weight((c, d))) => Storage::Weight((a + c, b + d)),
            _ => panic!("Cannot add different storage types"),
        }
    }
}

impl AddAssign for Storage {
    fn add_assign(&mut self, other: Storage) {
        *self = match (self.clone(), other) {
            (Storage::Double(a), Storage::Double(b)) => Storage::Double(a + b),
            (Storage::Int(a), Storage::Int(b)) => Storage::Int(a + b),
            (Storage::Weight((a, b)), Storage::Weight((c, d))) => Storage::Weight((a + c, b + d)),
            _ => panic!("Cannot add different storage types"),
        }
    }
}
