pub trait Step: Copy {
    fn one() -> Self;
    fn add_step(self) -> (Self, bool);
    fn sub_step(self) -> (Self, bool);
}

macro_rules! impl_step_int {
    ( $( $t:ty ),* ) => {
        $(
            impl Step for $t {
                #[inline]
                fn one() -> Self { 1 as $t }

                #[inline]
                fn add_step(self) -> (Self, bool) {
                    self.overflowing_add(1 as $t)
                }

                #[inline]
                fn sub_step(self) -> (Self, bool) {
                    self.overflowing_sub(1 as $t)
                }
            }
        )*
    };
}

impl_step_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl Step for f32 {
    #[inline]
    fn one() -> Self { 1.0 }

    #[inline]
    fn add_step(self) -> (Self, bool) {
        (self + 1.0, false)
    }

    #[inline]
    fn sub_step(self) -> (Self, bool) {
        (self - 1.0, false)
    }
}

impl Step for f64 {
    #[inline]
    fn one() -> Self { 1.0 }

    #[inline]
    fn add_step(self) -> (Self, bool) {
        (self + 1.0, false)
    }

    #[inline]
    fn sub_step(self) -> (Self, bool) {
        (self - 1.0, false)
    }
}
