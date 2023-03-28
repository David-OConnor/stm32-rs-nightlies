#[doc = "Register `GPIOH_IPIDR` reader"]
pub struct R(crate::R<GPIOH_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOH_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOH_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOH_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IPIDR` reader - IPIDR"]
pub type IPIDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPIDR"]
    #[inline(always)]
    pub fn ipidr(&self) -> IPIDR_R {
        IPIDR_R::new(self.bits)
    }
}
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_ipidr](index.html) module"]
pub struct GPIOH_IPIDR_SPEC;
impl crate::RegisterSpec for GPIOH_IPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioh_ipidr::R](R) reader structure"]
impl crate::Readable for GPIOH_IPIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOH_IPIDR to value 0x000f_0002"]
impl crate::Resettable for GPIOH_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0002;
}
