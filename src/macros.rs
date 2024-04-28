macro_rules! impl_from_f {
    ($struct:ident, [$($im:ident),*]$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> From<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn from(float: F) -> Self {
                Self::from_re(T::from(float))
            }
        }
    };
}

macro_rules! impl_zero_one {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Zero for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn zero() -> Self {
                Self::from_re(T::zero())
            }

            #[inline]
            fn is_zero(&self) -> bool {
                self.re.is_zero()
            }
        }

        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> One for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn one() -> Self {
                Self::from_re(T::one())
            }

            #[inline]
            fn is_one(&self) -> bool {
                self.re.is_one()
            }
        }
    };
}

macro_rules! impl_add_sub_rem {
    ($struct:ident, [$($im:ident),*]$(, [$($dim:tt),*])?) => {
        impl<'a, 'b, T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Add<&'a $struct<T, F$($(, $dim)*)?>>
            for &'b $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = $struct<T, F$($(, $dim)*)?>;
            #[inline]
            fn add(self, other: &$struct<T, F$($(, $dim)*)?>) -> $struct<T, F$($(, $dim)*)?> {
                Self::Output::new(self.re.clone() + &other.re, $(self.$im.clone() + &other.$im,)*)
            }
        }

        impl<'a, 'b, T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Sub<&'a $struct<T, F$($(, $dim)*)?>>
            for &'b $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = $struct<T, F$($(, $dim)*)?>;
            #[inline]
            fn sub(self, other: &$struct<T, F$($(, $dim)*)?>) -> $struct<T, F$($(, $dim)*)?> {
                Self::Output::new(self.re.clone() - &other.re, $(self.$im.clone() - &other.$im,)*)
            }
        }

        impl<'a, 'b, T: DualNum<F>, F$($(, $dim: Dim)*)?> Rem<&'a $struct<T, F$($(, $dim)*)?>> for &'b $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = $struct<T, F$($(, $dim)*)?>;
            #[inline]
            fn rem(self, _other: &$struct<T, F$($(, $dim)*)?>) -> $struct<T, F$($(, $dim)*)?> {
                unimplemented!()
            }
        }
    };
}

macro_rules! forward_binop {
    ($struct:ident, $trt:ident, $operator:tt, $mth:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> $trt<$struct<T, F$($(, $dim)*)?>> for &$struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = $struct<T, F$($(, $dim)*)?>;
            #[inline]
            fn $mth(self, rhs: $struct<T, F$($(, $dim)*)?>) -> Self::Output {
                self $operator &rhs
            }
        }

        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> $trt<&$struct<T, F$($(, $dim)*)?>> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = $struct<T, F$($(, $dim)*)?>;
            #[inline]
            fn $mth(self, rhs: &$struct<T, F$($(, $dim)*)?>) -> Self::Output {
                &self $operator rhs
            }
        }

        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> $trt for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = $struct<T, F$($(, $dim)*)?>;
            #[inline]
            fn $mth(self, rhs: $struct<T, F$($(, $dim)*)?>) -> Self::Output {
                &self $operator &rhs
            }
        }
    };
}

macro_rules! impl_neg {
    ($struct:ident, [$($im:ident),*]$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Neg for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                -&self
            }
        }

        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Neg for &$struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = $struct<T, F$($(, $dim)*)?>;
            #[inline]
            fn neg(self) -> Self::Output {
                <$struct<T, F$($(, $dim)*)?>>::new(-self.re.clone(), $(-self.$im.clone()),*)
            }
        }
    };
}

macro_rules! impl_assign_ops {
    ($struct:ident, [$($im:ident),*]$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> MulAssign for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                *self = self.clone() * other;
            }
        }

        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> DivAssign for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn div_assign(&mut self, other: Self) {
                *self = self.clone() / other;
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> AddAssign for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                self.re += other.re;
                $(self.$im += other.$im;)*
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> SubAssign for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.re -= other.re;
                $(self.$im -= other.$im;)*
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> RemAssign for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn rem_assign(&mut self, _other: Self) {
                unimplemented!()
            }
        }
    };
}

macro_rules! impl_scalar_op {
    ($struct:ident, [$($im:ident),*]$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: DualNumFloat$($(, $dim: Dim)*)?> Mul<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn mul(mut self, other: F) -> Self {
                self *= other;
                self
            }
        }

        impl<T: DualNum<F>, F: DualNumFloat$($(, $dim: Dim)*)?> MulAssign<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn mul_assign(&mut self, other: F) {
                self.re *= other;
                $(self.$im *= T::from(other);)*
            }
        }

        impl<T: DualNum<F>, F: DualNumFloat$($(, $dim: Dim)*)?> Div<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn div(mut self, other: F) -> Self {
                self /= other;
                self
            }
        }

        impl<T: DualNum<F>, F: DualNumFloat$($(, $dim: Dim)*)?> DivAssign<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn div_assign(&mut self, other: F) {
                self.re /= other;
                $(self.$im /= T::from(other);)*
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> Add<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn add(mut self, other: F) -> Self {
                self.re += other;
                self
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> AddAssign<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn add_assign(&mut self, other: F)  {
                self.re += other;
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> Sub<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn sub(mut self, other: F) -> Self {
                self.re -= other;
                self
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> SubAssign<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn sub_assign(&mut self, other: F)  {
                self.re -= other;
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> Rem<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn rem(self, _other: F) -> Self {
                unimplemented!()
            }
        }

        impl<T: DualNum<F>, F$($(, $dim: Dim)*)?> RemAssign<F> for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn rem_assign(&mut self, _other: F) {
                unimplemented!()
            }
        }
    };
}

macro_rules! impl_scalar_op_lhs {
    ($f:ty, $struct:ident, [$($im:ident),*]$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<$f>$($(, $dim: Dim)*)?,> Mul<$struct<T, $f $($(, $dim)*)?>> for $f
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>,)?
        {
            type Output = $struct<T, $f$($(, $dim)*)?>;
            #[inline]
            fn mul(self, mut other: $struct<T, $f$($(, $dim)*)?>) -> Self::Output {
                other *= self;
                other
            }
        }

        impl<T: DualNum<$f>$($(, $dim: Dim)*)?,> Div<$struct<T, $f $($(, $dim)*)?>> for $f
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>,)?
        {
            type Output = $struct<T, $f$($(, $dim)*)?>;
            #[inline]
            #[allow(clippy::suspicious_arithmetic_impl)]
            fn div(self, other: $struct<T, $f$($(, $dim)*)?>) -> Self::Output {
                self * other.inv()
            }
        }

        impl<T: DualNum<$f>$($(, $dim: Dim)*)?,> Add<$struct<T, $f $($(, $dim)*)?>> for $f
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>,)?
        {
            type Output = $struct<T, $f$($(, $dim)*)?>;
            #[inline]
            fn add(self, mut other: $struct<T, $f$($(, $dim)*)?>) -> Self::Output {
                other.re += self;
                other
            }
        }

        impl<T: DualNum<$f>$($(, $dim: Dim)*)?,> Sub<$struct<T, $f $($(, $dim)*)?>> for $f
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>,)?
        {
            type Output = $struct<T, $f$($(, $dim)*)?>;
            #[inline]
            fn sub(self, other: $struct<T, $f$($(, $dim)*)?>) -> Self::Output {
                self + -other
            }
        }
    };
}

macro_rules! impl_inv {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: DualNumFloat$($(, $dim: Dim)*)?> Inv for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn inv(self) -> Self {
                self.recip()
            }
        }
    };
}

macro_rules! impl_iterator {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Sum for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::zero(), |acc, c| acc + c)
            }
        }

        impl<'a, T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Sum<&'a $struct<T, F$($(, $dim)*)?>>
            for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a $struct<T, F$($(, $dim)*)?>>,
            {
                iter.fold(Self::zero(), |acc, c| acc + c)
            }
        }
        impl<T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Product for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::one(), |acc, c| acc * c)
            }
        }
        impl<'a, T: DualNum<F>, F: Float$($(, $dim: Dim)*)?> Product<&'a $struct<T, F$($(, $dim)*)?>>
            for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a $struct<T, F$($(, $dim)*)?>>,
            {
                iter.fold(Self::one(), |acc, c| acc * c)
            }
        }
    };
}

macro_rules! impl_from_primitive {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: Float + FromPrimitive$($(, $dim: Dim)*)?> FromPrimitive for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn from_isize(n: isize) -> Option<Self> {
                F::from_isize(n).map(|f| f.into())
            }

            #[inline]
            fn from_i8(n: i8) -> Option<Self> {
                F::from_i8(n).map(|f| f.into())
            }

            #[inline]
            fn from_i16(n: i16) -> Option<Self> {
                F::from_i16(n).map(|f| f.into())
            }

            #[inline]
            fn from_i32(n: i32) -> Option<Self> {
                F::from_i32(n).map(|f| f.into())
            }

            #[inline]
            fn from_i64(n: i64) -> Option<Self> {
                F::from_i64(n).map(|f| f.into())
            }

            #[inline]
            fn from_i128(n: i128) -> Option<Self> {
                F::from_i128(n).map(|f| f.into())
            }

            #[inline]
            fn from_usize(n: usize) -> Option<Self> {
                F::from_usize(n).map(|f| f.into())
            }

            #[inline]
            fn from_u8(n: u8) -> Option<Self> {
                F::from_u8(n).map(|f| f.into())
            }

            #[inline]
            fn from_u16(n: u16) -> Option<Self> {
                F::from_u16(n).map(|f| f.into())
            }

            #[inline]
            fn from_u32(n: u32) -> Option<Self> {
                F::from_u32(n).map(|f| f.into())
            }

            #[inline]
            fn from_u64(n: u64) -> Option<Self> {
                F::from_u64(n).map(|f| f.into())
            }

            #[inline]
            fn from_u128(n: u128) -> Option<Self> {
                F::from_u128(n).map(|f| f.into())
            }

            #[inline]
            fn from_f32(n: f32) -> Option<Self> {
                F::from_f32(n).map(|f| f.into())
            }

            #[inline]
            fn from_f64(n: f64) -> Option<Self> {
                F::from_f64(n).map(|f| f.into())
            }
        }
    };
}

macro_rules! impl_signed {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: DualNumFloat$($(, $dim: Dim)*)?> Signed for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn abs(&self) -> Self {
                if self.is_positive() {
                    self.clone()
                } else {
                    -self
                }
            }

            #[inline]
            fn abs_sub(&self, other: &Self) -> Self {
                if self.re() > other.re() {
                    self - other
                } else {
                    Self::zero()
                }
            }

            #[inline]
            fn signum(&self) -> Self {
                if self.is_positive() {
                    Self::one()
                } else if self.is_zero() {
                    Self::zero()
                } else {
                    -Self::one()
                }
            }

            #[inline]
            fn is_positive(&self) -> bool {
                self.re.is_positive()
            }

            #[inline]
            fn is_negative(&self) -> bool {
                self.re.is_negative()
            }
        }
    };
}

macro_rules! impl_float_const {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F>, F: Float + FloatConst$($(, $dim: Dim)*)?> FloatConst for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[allow(non_snake_case)]
            fn E() -> Self {
                Self::from(F::E())
            }

            #[allow(non_snake_case)]
            fn FRAC_1_PI() -> Self {
                Self::from(F::FRAC_1_PI())
            }

            #[allow(non_snake_case)]
            fn FRAC_1_SQRT_2() -> Self {
                Self::from(F::FRAC_1_SQRT_2())
            }

            #[allow(non_snake_case)]
            fn FRAC_2_PI() -> Self {
                Self::from(F::FRAC_2_PI())
            }

            #[allow(non_snake_case)]
            fn FRAC_2_SQRT_PI() -> Self {
                Self::from(F::FRAC_2_SQRT_PI())
            }

            #[allow(non_snake_case)]
            fn FRAC_PI_2() -> Self {
                Self::from(F::FRAC_PI_2())
            }

            #[allow(non_snake_case)]
            fn FRAC_PI_3() -> Self {
                Self::from(F::FRAC_PI_3())
            }

            #[allow(non_snake_case)]
            fn FRAC_PI_4() -> Self {
                Self::from(F::FRAC_PI_4())
            }

            #[allow(non_snake_case)]
            fn FRAC_PI_6() -> Self {
                Self::from(F::FRAC_PI_6())
            }

            #[allow(non_snake_case)]
            fn FRAC_PI_8() -> Self {
                Self::from(F::FRAC_PI_8())
            }

            #[allow(non_snake_case)]
            fn LN_10() -> Self {
                Self::from(F::LN_10())
            }

            #[allow(non_snake_case)]
            fn LN_2() -> Self {
                Self::from(F::LN_2())
            }

            #[allow(non_snake_case)]
            fn LOG10_E() -> Self {
                Self::from(F::LOG10_E())
            }

            #[allow(non_snake_case)]
            fn LOG2_E() -> Self {
                Self::from(F::LOG2_E())
            }

            #[allow(non_snake_case)]
            fn PI() -> Self {
                Self::from(F::PI())
            }

            #[allow(non_snake_case)]
            fn SQRT_2() -> Self {
                Self::from(F::SQRT_2())
            }
        }
    };
}

macro_rules! impl_num {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F> + Signed, F: Float$($(, $dim: Dim)*)?> Num for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type FromStrRadixErr = F::FromStrRadixErr;
            #[inline]
            fn from_str_radix(_str: &str, _radtix: u32) -> Result<Self, Self::FromStrRadixErr> {
                unimplemented!()
            }
        }
    };
}

macro_rules! impl_trait {
    ($struct:ident$(, [$($dim:tt),*])?, $trait:ident, $fn:ident) => {
        impl<T: DualNum<F>, F: DualNumFloat$($(, $dim: Dim)*)?> geo_traits::$trait for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn $fn(self) -> Self::Output {
                <$struct<T, F$($(, $dim)*)?> as DualNum<F>>::$fn(&self)
            }
        }
    };
}

macro_rules! impl_trait_trig {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T: DualNum<F> + geo_traits::Trig, F: DualNumFloat$($(, $dim: Dim)*)?> geo_traits::Trig for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;
            #[inline]
            fn tau() -> Self {
                Self::from_re(T::tau())
            }
            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                <$struct<T, F$($(, $dim)*)?> as DualNum<F>>::sin_cos(&self)
            }
            #[inline]
            fn sin(self) -> Self::Output {
                <$struct<T, F$($(, $dim)*)?> as DualNum<F>>::sin(&self)
            }
            #[inline]
            fn cos(self) -> Self::Output {
                <$struct<T, F$($(, $dim)*)?> as DualNum<F>>::cos(&self)
            }
            #[inline]
            fn tan(self) -> Self::Output {
                <$struct<T, F$($(, $dim)*)?> as DualNum<F>>::tan(&self)
            }
            #[inline]
            fn sinh(self) -> Self::Output {
                <$struct<T, F$($(, $dim)*)?> as DualNum<F>>::sinh(&self)
            }
            #[inline]
            fn cosh(self) -> Self::Output {
                <$struct<T, F$($(, $dim)*)?> as DualNum<F>>::cosh(&self)
            }
            #[inline]
            fn tanh(self) -> Self::Output {
                <$struct<T, F$($(, $dim)*)?> as DualNum<F>>::tanh(&self)
            }
        }
    };
}

macro_rules! impl_trait_inv_trig {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        impl<T, F$($(, $dim: Dim)*)?> geo_traits::InvTrig for $struct<T, F$($(, $dim)*)?>
        where
            Self: DualNum<F>,
            T: DualNum<F> + PartialOrd + geo_traits::InvTrig<Output = T> + Copy + num_traits::Zero,
            F: num_traits::Float,
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            type Output = Self;

            fn atan2(self, x: Self) -> Self {
                let y = self;
                let two = Self::from_re(T::from_f64(2.0).unwrap());
                let pi = Self::from_re(T::from_f64(std::f64::consts::PI).unwrap());
                let nan = Self::from_re(T::from_f64(f64::NAN).unwrap());

                let (Some(x_cmp), Some(y_cmp)) = (x.re.partial_cmp(&T::zero()), y.re.partial_cmp(&T::zero()) ) else {
                    return nan;
                };
                let sqrt_x2_y2 = (&x * &x + &y * &y).sqrt();
                use std::cmp::Ordering::*;
                match (x_cmp, y_cmp) {
                    (Greater, _) => geo_traits::InvTrig::atan(y / (sqrt_x2_y2 + x)) * two,
                    (Less | Equal, Greater | Less) => geo_traits::InvTrig::atan((sqrt_x2_y2 - x) / y) * two,
                    (Less, Equal) => geo_traits::InvTrig::atan(-y / (sqrt_x2_y2 - x)) * two + pi,
                    (Equal, Equal) => nan,
                }
            }

            fn asin(self) -> Self::Output {
                DualNum::asin(&self)
            }

            fn acos(self) -> Self::Output {
                DualNum::acos(&self)
            }

            fn atan(self) -> Self::Output {
                DualNum::atan(&self)
            }

            fn asinh(self) -> Self::Output {
                DualNum::asinh(&self)
            }

            fn acosh(self) -> Self::Output {
                DualNum::acosh(&self)
            }

            fn atanh(self) -> Self::Output {
                DualNum::atanh(&self)
            }
        }
    };
}

macro_rules! impl_trait_no_output {
    ($struct:ident$(, [$($dim:tt),*])?, $trait:ident, $fn:ident, $fn1:ident) => {
        impl<T: DualNum<F>, F: DualNumFloat$($(, $dim: Dim)*)?> geo_traits::$trait for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn $fn(value: f64) -> Self {
                Self::$fn1(T::from_f64(value).unwrap())
            }
        }
    };
}

macro_rules! impl_partial_ord {
    ($struct:ident$(, [$($dim:tt),*])?) => {
        /// Like PartialEq, comparisons are only made based on the real part. This allows the code to follow the
        /// same execution path as real-valued code would.
        impl<T: DualNum<F> + PartialOrd, F: Float$($(, $dim: Dim)*)?> PartialOrd for $struct<T, F$($(, $dim)*)?>
        where
            $($(DefaultAllocator: Allocator<T, $dim> + Allocator<T, U1, $dim> + Allocator<T, $dim, $dim>,)*
            DefaultAllocator: Allocator<T$(, $dim)*>)?
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.re.partial_cmp(&other.re)
            }
        }
    };
}

macro_rules! impl_dual {
    ($struct:ident, [$($im:ident),*]$(, [$($dim:tt),*])?) => {
        impl_from_f!($struct, [$($im),*]$(, [$($dim),*])?);
        impl_zero_one!($struct$(, [$($dim),*])?);
        impl_add_sub_rem!($struct, [$($im),*]$(, [$($dim),*])?);
        forward_binop!($struct, Add, +, add$(, [$($dim),*])?);
        forward_binop!($struct, Sub, -, sub$(, [$($dim),*])?);
        forward_binop!($struct, Mul, *, mul$(, [$($dim),*])?);
        forward_binop!($struct, Div, /, div$(, [$($dim),*])?);
        forward_binop!($struct, Rem, %, rem$(, [$($dim),*])?);
        impl_neg!($struct, [$($im),*]$(, [$($dim),*])?);
        impl_assign_ops!($struct, [$($im),*]$(, [$($dim),*])?);
        impl_scalar_op!($struct, [$($im),*]$(, [$($dim),*])?);
        impl_scalar_op_lhs!(f32, $struct, [$($im),*]$(, [$($dim),*])?);
        impl_scalar_op_lhs!(f64, $struct, [$($im),*]$(, [$($dim),*])?);
        impl_inv!($struct$(, [$($dim),*])?);
        impl_iterator!($struct$(, [$($dim),*])?);
        impl_from_primitive!($struct$(, [$($dim),*])?);
        impl_signed!($struct$(, [$($dim),*])?);
        impl_num!($struct$(, [$($dim),*])?);
        impl_float_const!($struct$(, [$($dim),*])?);
        impl_trait!($struct$(, [$($dim),*])?, Sqrt, sqrt);
        impl_trait!($struct$(, [$($dim),*])?, Ln, ln);
        impl_trait!($struct$(, [$($dim),*])?, Exp, exp);
        impl_trait_no_output!($struct$(, [$($dim),*])?, FromF64, from_f64, from_re);
        impl_trait_trig!($struct$(, [$($dim),*])?);
        impl_trait_inv_trig!($struct$(, [$($dim),*])?);
        impl_partial_ord!($struct$(, [$($dim),*])?);
    };
}
