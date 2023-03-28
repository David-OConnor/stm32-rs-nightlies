#[doc = "Register `VHSACCR` reader"]
pub struct R(crate::R<VHSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHSACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HSA` reader - Horizontal Synchronism Active duration"]
pub type HSA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host Video HSA Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhsaccr](index.html) module"]
pub struct VHSACCR_SPEC;
impl crate::RegisterSpec for VHSACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vhsaccr::R](R) reader structure"]
impl crate::Readable for VHSACCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VHSACCR to value 0"]
impl crate::Resettable for VHSACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
