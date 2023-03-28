#[doc = "Register `DDRPERFM_ISR` reader"]
pub struct R(crate::R<DDRPERFM_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVFF` reader - OVFF"]
pub type OVFF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - OVFF"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DDRPERFM interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_isr](index.html) module"]
pub struct DDRPERFM_ISR_SPEC;
impl crate::RegisterSpec for DDRPERFM_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_isr::R](R) reader structure"]
impl crate::Readable for DDRPERFM_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_ISR to value 0"]
impl crate::Resettable for DDRPERFM_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
