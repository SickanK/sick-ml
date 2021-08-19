use crate::vector::inline_vector::InlineVector;

pub trait IntoVec<T, const N: usize> {
    fn into_vec(self) -> Vec<T>;
}

impl<T, const N: usize> IntoVec<T, N> for Vec<T> {
    fn into_vec(self) -> Vec<T> {
        self
    }
}

impl<T, const N: usize> IntoVec<T, N> for [T; N]
where
    T: Copy,
{
    fn into_vec(self) -> Vec<T> {
        let mut converted_vec: Vec<T> = Vec::with_capacity(N);
        for t in self.iter() {
            converted_vec.push(*t);
        }

        converted_vec
    }
}

impl<T, const N: usize> IntoVec<T, N> for InlineVector<T, N>
where
    T: Copy,
{
    fn into_vec(self) -> Vec<T> {
        self.to_vec()
    }
}
