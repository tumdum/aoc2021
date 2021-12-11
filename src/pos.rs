use smallvec::SmallVec;
use std::convert::TryInto;
use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Pos<T>(pub T, pub T);

impl<T> Pos<T> 
where
    T: TryInto<usize> + Copy,
{
    #[inline]
    pub fn get_mut<'a, const N: usize>(&self, v: &'a mut [SmallVec<[T; N]>]) -> Option<&'a mut T> {
        v.get_mut(self.0.try_into().ok()?).and_then(|r| r.get_mut(self.1.try_into().ok()?))
    }
}

impl<T> From<(T,T)> for Pos<T> {
    fn from((a,b): (T,T)) -> Self {
        Pos(a,b)
    }
}

impl<T> Add<Pos<T>> for Pos<T>
where
    T: Add<Output=T>
{
    type Output = Pos<T>;
    #[inline]
    fn add(self, other: Pos<T> ) -> Pos<T> {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}
