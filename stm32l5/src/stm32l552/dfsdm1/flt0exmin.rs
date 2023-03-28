#[doc = "Register `FLT0EXMIN` reader"]
pub struct R(crate::R<FLT0EXMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT0EXMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT0EXMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT0EXMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXMINCH` reader - Extremes detector minimum data channel"]
pub type EXMINCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXMIN` reader - EXMIN"]
pub type EXMIN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - Extremes detector minimum data channel"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0exmin](index.html) module"]
pub struct FLT0EXMIN_SPEC;
impl crate::RegisterSpec for FLT0EXMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt0exmin::R](R) reader structure"]
impl crate::Readable for FLT0EXMIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLT0EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for FLT0EXMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff_ff00;
}
