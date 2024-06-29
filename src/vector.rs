use anyhow::Result;
use std::ops::{Add, AddAssign, Deref, Mul};
pub struct Vector<T> {
    data: Vec<T>,
}

// region:    --- impls
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
// endregion: --- impls

// region:    --- functions
// 将 &[T] 转换成 Vec<T>, 因为考虑到多线程, 需要传入一个 owned value
/// Calculate the dot product of two vectors
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Dot product error: a.len != b.len"));
    }
    let mut result = T::default();
    for i in 0..a.len() {
        // 使 Vector<T> 可以通过 index 访问
        // 方法1: 为 Vector<T> 实现 Index trait
        // 方法2: 为 Vector<T> 实现 Deref trait (recommend)
        result += a[i] * b[i];
    }
    Ok(result)
}

// endregion: --- functions
