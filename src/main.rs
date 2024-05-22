pub trait Trait<T> {
    fn f(&self) -> T;
}

pub struct S1<'a>(pub &'a Box<S3<'a>>);

pub struct S2<'a>(pub Vec<S1<'a>>);

pub struct S3<'a>(pub Vec<S2<'a>>);

impl<T> Trait<T> for S1<'_>
where
    for<'a> S1<'a>: Trait<T>,
    for<'a> S3<'a>: Trait<T>,
{
    fn f(&self) -> T {
        self.0.f()
    }
}

impl<T> Trait<T> for S2<'_>
where
    for<'a> S1<'a>: Trait<T>,
{
    fn f(&self) -> T {
        _ = self.0.iter().map(|s| s.f());
        todo!()
    }
}

impl<T> Trait<T> for S3<'_>
where
    for<'a> S2<'a>: Trait<T>,
{
    fn f(&self) -> T {
        _ = self.0.iter().map(|s| s.f());
        todo!()
    }
}

fn main() {
    S3(vec![]).f();
}
