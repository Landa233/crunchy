use std::{
    fmt,
    ops::{Add, Deref, Rem, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Ind<const N: usize> {
    pub data: [usize; N],
}

impl<const N: usize> Add for Ind<N> {
    type Output = Ind<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [0; N];
        for i in 0..N {
            data[i] = self.data[i] + rhs.data[i];
        }
        Ind { data }
    }
}

impl<const N: usize> Add<[usize; N]> for Ind<N> {
    type Output = Ind<N>;

    fn add(self, rhs: [usize; N]) -> Self::Output {
        let mut data = [0; N];
        for i in 0..N {
            data[i] = self.data[i] + rhs[i];
        }
        Ind { data }
    }
}

impl<const N: usize> Sub for Ind<N> {
    type Output = Ind<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [0; N];
        for i in 0..N {
            data[i] = self.data[i] - rhs.data[i];
        }
        Ind { data }
    }
}

impl<const N: usize> Ind<N> {
    pub fn new(data: [usize; N]) -> Self {
        Self { data }
    }

    pub fn prepend(&self, value: usize) -> Ind<{ N + 1 }>
    where
        [(); N + 1]:,
    {
        let mut res = [0; N + 1];
        res[0] = value;
        for i in 0..N {
            res[i + 1] = self.data[i];
        }

        return Ind::new(res);
    }
}

impl<const N: usize> Rem<[usize; N]> for Ind<N> {
    type Output = Ind<N>;

    fn rem(self, rhs: [usize; N]) -> Self::Output {
        let mut data = [0; N];
        for i in 0..N {
            data[i] = self.data[i] % rhs[i];
        }
        Ind { data }
    }
}

impl<const N: usize> Deref for Ind<N> {
    type Target = [usize; N];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<const N: usize> fmt::Display for Ind<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
