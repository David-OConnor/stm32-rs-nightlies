#[doc = "Register `DTS_RAMPVALR` reader"]
pub struct R(crate::R<DTS_RAMPVALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_RAMPVALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_RAMPVALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_RAMPVALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TS1_RAMP_COEFF` reader - Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C."]
pub type TS1_RAMP_COEFF_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C."]
    #[inline(always)]
    pub fn ts1_ramp_coeff(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Temperature sensor ramp value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_rampvalr](index.html) module"]
pub struct DTS_RAMPVALR_SPEC;
impl crate::RegisterSpec for DTS_RAMPVALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_rampvalr::R](R) reader structure"]
impl crate::Readable for DTS_RAMPVALR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTS_RAMPVALR to value 0"]
impl crate::Resettable for DTS_RAMPVALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
