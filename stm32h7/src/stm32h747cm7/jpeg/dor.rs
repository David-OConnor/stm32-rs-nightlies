#[doc = "Register `DOR` reader"]
pub struct R(crate::R<DOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAOUT` reader - Data Output FIFO Output FIFO data register."]
pub type DATAOUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Output FIFO Output FIFO data register."]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
#[doc = "JPEG data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dor](index.html) module"]
pub struct DOR_SPEC;
impl crate::RegisterSpec for DOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dor::R](R) reader structure"]
impl crate::Readable for DOR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DOR to value 0"]
impl crate::Resettable for DOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
