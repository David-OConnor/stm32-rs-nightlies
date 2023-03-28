#[doc = "Register `GICD_SPISR6` reader"]
pub struct R(crate::R<GICD_SPISR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPISR6` reader - SPISR6"]
pub type SPISR6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR6"]
    #[inline(always)]
    pub fn spisr6(&self) -> SPISR6_R {
        SPISR6_R::new(self.bits)
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr6](index.html) module"]
pub struct GICD_SPISR6_SPEC;
impl crate::RegisterSpec for GICD_SPISR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spisr6::R](R) reader structure"]
impl crate::Readable for GICD_SPISR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_SPISR6 to value 0"]
impl crate::Resettable for GICD_SPISR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
