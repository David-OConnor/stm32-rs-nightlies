#[doc = "Register `UR6` reader"]
pub struct R(crate::R<UR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PA_BEG_1` reader - Protected area start address for bank 1"]
pub type PA_BEG_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PA_END_1` reader - Protected area end address for bank 1"]
pub type PA_END_1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Protected area start address for bank 1"]
    #[inline(always)]
    pub fn pa_beg_1(&self) -> PA_BEG_1_R {
        PA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Protected area end address for bank 1"]
    #[inline(always)]
    pub fn pa_end_1(&self) -> PA_END_1_R {
        PA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur6](index.html) module"]
pub struct UR6_SPEC;
impl crate::RegisterSpec for UR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur6::R](R) reader structure"]
impl crate::Readable for UR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR6 to value 0"]
impl crate::Resettable for UR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
