use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use anyhow::Result;

use crate::{dot_product, Vector};

const NUM_THREADS: usize = 4;

// [[1,2], [3,4], [5,6]] -> [1, 2, 3, 4, 5, 6]
// 后面这种方式效率更高

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    /// sender to send the result back
    sender: oneshot::Sender<MsgOutput<T>>,
}

// region:    --- impls
impl<T: Debug> Matrix<T> {
    // 任何数据结构, 只要可以 convert 成 Vec<T>, 那么下面的代码就是可以通过的
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiply error")
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> MsgOutput<T> {
    pub fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

// endregion: --- impls

// region:    --- functions
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("Matrix multiply error: a.col != b.row"));
    }
    // region:    --- old code
    // 不能放具体的类型, 因为 T 是泛型, 因此这里需要用 T::default()
    // let mut data = vec![T::default(); a.row * b.col];
    // for i in 0..a.row {
    //     for j in 0..b.col {
    //         for k in 0..a.col {
    //             data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
    //         }
    //     }
    // }
    // endregion: --- old code

    // region:    --- change to multithreading
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>(); // 一个 Msg<T> 的 channel
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput::new(msg.input.idx, value)) {
                        eprintln!("Send error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();
    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    // map/reduce: map phase
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(rx);
        }
    }

    // map/reduce: reduce phase
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }
    // endregion: --- change to multithreading

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

// endregion: --- functions

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([10, 11, 20, 21, 30, 31], 3, 2);
        let c = a * b;
        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![140, 146, 320, 335]);
        assert_eq!(
            format!("{:?}", c),
            "Matrix(row=2, col=2, {140 146, 320 335})"
        );

        Ok(())
    }

    #[test]
    fn test_matrix_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = a * b;
        assert_eq!(c.data, vec![7, 10, 15, 22]);
        assert_eq!(format!("{}", c), "{7 10, 15 22}");
        Ok(())
    }

    #[test]
    fn test_a_can_not_multiply_b() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b);
        assert!(c.is_err());
    }

    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let _c = a * b;
    }
}
