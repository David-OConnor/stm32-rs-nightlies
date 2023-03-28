#[doc = "Register `DTS_T0VALR1` reader"]
pub struct R(crate::R<DTS_T0VALR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_T0VALR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_T0VALR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_T0VALR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TS1_FMT0` reader - TS1_FMT0"]
pub type TS1_FMT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS1_T0` reader - TS1_T0"]
pub type TS1_T0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - TS1_FMT0"]
    #[inline(always)]
    pub fn ts1_fmt0(&self) -> TS1_FMT0_R {
        TS1_FMT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - TS1_T0"]
    #[inline(always)]
    pub fn ts1_t0(&self) -> TS1_T0_R {
        TS1_T0_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_t0valr1](index.html) module"]
pub struct DTS_T0VALR1_SPEC;
impl crate::RegisterSpec for DTS_T0VALR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_t0valr1::R](R) reader structure"]
impl crate::Readable for DTS_T0VALR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTS_T0VALR1 to value 0"]
impl crate::Resettable for DTS_T0VALR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
