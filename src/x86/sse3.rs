use x86::__m128i;
use v128::*;

#[cfg(test)]
use stdsimd_test::assert_instr;

/// Alternatively add and subtract packed single-precision (32-bit)
/// floating-point elements in `a` to/from packed elements in `b`.
#[inline(always)]
#[target_feature = "+sse3"]
#[cfg_attr(test, assert_instr(addsub))]
pub unsafe fn _mm_addsub_ps(a: f32x4, b: f32x4) -> f32x4 {
    addsubps(a, b)
}

/// Alternatively add and subtract packed double-precision (64-bit)
/// floating-point elements in `a` to/from packed elements in `b`.
#[inline(always)]
#[target_feature = "+sse3"]
#[cfg_attr(test, assert_instr(addsub))]
pub unsafe fn _mm_addsub_pd(a: f64x2, b: f64x2) -> f64x2 {
    addsubpd(a, b)
}

/// Horizontally add adjacent pairs of double-precision (64-bit)
/// floating-point elements in `a` and `b`, and pack the results.
#[inline(always)]
#[target_feature = "+sse3"]
#[cfg_attr(test, assert_instr(hadd))]
pub unsafe fn _mm_hadd_pd(a: f64x2, b: f64x2) -> f64x2 {
    haddpd(a, b)
}

/// Load 128-bits of integer data from unaligned memory.
/// This intrinsic may perform better than `_mm_loadu_si128`
/// when the data crosses a cache line boundary.
#[inline(always)]
#[target_feature = "+sse3"]
#[cfg_attr(test, assert_instr(lddqu))]
pub unsafe fn _mm_lddqu_si128(mem_addr: *const __m128i) -> __m128i {
    lddqu(mem_addr as *const _)
}

#[allow(improper_ctypes)]
extern {
    #[link_name = "llvm.x86.sse3.addsub.ps"]
    fn addsubps(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse3.addsub.pd"]
    fn addsubpd(a: f64x2, b: f64x2) -> f64x2;
    #[link_name = "llvm.x86.sse3.hadd.pd"]
    fn haddpd(a: f64x2, b: f64x2) -> f64x2;
    #[link_name = "llvm.x86.sse3.ldu.dq"]
    fn lddqu(mem_addr: *const i8) -> __m128i;
}


#[cfg(test)]
mod tests {
    use stdsimd_test::simd_test;

    use v128::*;
    use x86::sse3 as sse3;

    #[simd_test = "sse3"]
    unsafe fn _mm_addsub_ps() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse3::_mm_addsub_ps(a, b);
        assert_eq!(r, f32x4::new(99.0, 25.0, 0.0, -15.0));
    }

    #[simd_test = "sse3"]
    unsafe fn _mm_addsub_pd() {
        let a = f64x2::new(-1.0, 5.0);
        let b = f64x2::new(-100.0, 20.0);
        let r = sse3::_mm_addsub_pd(a, b);
        assert_eq!(r, f64x2::new(99.0, 25.0));
    }

    #[simd_test = "sse3"]
    unsafe fn _mm_hadd_pd() {
        let a = f64x2::new(-1.0, 5.0);
        let b = f64x2::new(-100.0, 20.0);
        let r = sse3::_mm_hadd_pd(a, b);
        assert_eq!(r, f64x2::new(4.0, -80.0));
    }

    #[simd_test = "sse3"]
    unsafe fn _mm_lddqu_si128() {
        let a = i8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse3::_mm_lddqu_si128(&a);
        assert_eq!(a, r);
    }
}