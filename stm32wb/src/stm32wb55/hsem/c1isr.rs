#[doc = "Register `C1ISR` reader"]
pub struct R(crate::R<C1ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISFm` reader - CPU(n) semaphore m status bit before enable (mask)"]
pub type ISFM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU(n) semaphore m status bit before enable (mask)"]
    #[inline(always)]
    pub fn isfm(&self) -> ISFM_R {
        ISFM_R::new(self.bits)
    }
}
#[doc = "HSEM Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1isr](index.html) module"]
pub struct C1ISR_SPEC;
impl crate::RegisterSpec for C1ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1isr::R](R) reader structure"]
impl crate::Readable for C1ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C1ISR to value 0"]
impl crate::Resettable for C1ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
