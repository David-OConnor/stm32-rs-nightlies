#[doc = "Register `GICD_SPISR5` reader"]
pub struct R(crate::R<GICD_SPISR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPISR5` reader - SPISR5"]
pub type SPISR5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR5"]
    #[inline(always)]
    pub fn spisr5(&self) -> SPISR5_R {
        SPISR5_R::new(self.bits)
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr5](index.html) module"]
pub struct GICD_SPISR5_SPEC;
impl crate::RegisterSpec for GICD_SPISR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spisr5::R](R) reader structure"]
impl crate::Readable for GICD_SPISR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_SPISR5 to value 0"]
impl crate::Resettable for GICD_SPISR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
