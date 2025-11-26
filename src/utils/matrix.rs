use std::fmt::Debug;

use crate::utils::vector2::Vector2;

pub struct Matrix<T> {
    _values: Vec<T>,
    _n: usize,
    _m: usize,
    _size: usize
}

impl<T> Matrix<T> {
    pub fn rows(self) -> usize {
        self._n
    }

    pub fn cols(self) -> usize {
        self._m
    }

    pub fn size(self) -> usize {
        self._size
    }

    pub fn get(&self, i: usize, j: usize) -> Result<&T, &'static str> {
        if i >= self._n || j >= self._m {
            return Err("Index out of bounds!");
        }
        Ok(&self._values[i * self._m + j])
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Result<&mut T, &'static str> {
        if i >= self._n || j >= self._m {
            return Err("Index out of bounds!");
        }
        Ok(&mut self._values[i * self._m + j])
    }

    pub fn set(&mut self, i: usize, j: usize, item: T) -> Result<(), &'static str> {
        if i >= self._n || j >= self._m {
            return Err("Index out of bounds!");
        }
        self._values[i * self._m + j] = item;
        Ok(())
    }

    pub fn v_get(&self, v: Vector2<usize>) -> Result<&T, &'static str> {
        if v.y >= self._n || v.x >= self._m {
            return Err("Index out of bounds!");
        }
        Ok(&self._values[v.y * self._m + v.x])
    }

    pub fn v_get_mut(&mut self, v: Vector2<usize>) -> Result<&mut T, &'static str> {
        if v.y >= self._n || v.x >= self._m {
            return Err("Index out of bounds!");
        }
        Ok(&mut self._values[v.y * self._m + v.x])
    }

    pub fn v_set(&mut self, v: Vector2<usize>, item: T) -> Result<(), &'static str> {
        if v.y >= self._n || v.x >= self._m {
            return Err("Index out of bounds!");
        }
        self._values[v.y * self._m + v.x] = item;
        Ok(())
    }
}

impl <T: Clone> Matrix<T> {
    pub fn from_values(values: Vec<T>, n: usize, m: usize) -> Self {
        let size = n * m;
        if values.len() != size {
            panic!("Matrix size must match the number of elements.");
        }
        Matrix { _values: values.clone(), _n: n, _m: m, _size: size }
    }

    pub fn from_default(default: T, n: usize, m: usize) -> Self {
        let size = n * m;
        Matrix { _values: vec![default; size], _n: n, _m: m, _size: size }
    }
}

impl <T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix ({}x{}):", self._n, self._m)?;
        for i in 0..self._n {
            write!(f, "[")?;
            for j in 0..self._m {
                let idx = i * self._m + j;
                write!(f, "{:?}", self._values[idx])?;
                if j < self._m - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}