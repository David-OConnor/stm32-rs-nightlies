#[doc = "Register `LTDC_CPSR` reader"]
pub struct R(crate::R<LTDC_CPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_CPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_CPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_CPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CYPOS` reader - CYPOS"]
pub type CYPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CXPOS` reader - CXPOS"]
pub type CXPOS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CYPOS"]
    #[inline(always)]
    pub fn cypos(&self) -> CYPOS_R {
        CYPOS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CXPOS"]
    #[inline(always)]
    pub fn cxpos(&self) -> CXPOS_R {
        CXPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "LTDC current position status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_cpsr](index.html) module"]
pub struct LTDC_CPSR_SPEC;
impl crate::RegisterSpec for LTDC_CPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_cpsr::R](R) reader structure"]
impl crate::Readable for LTDC_CPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTDC_CPSR to value 0"]
impl crate::Resettable for LTDC_CPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
