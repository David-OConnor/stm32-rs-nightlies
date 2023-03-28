#[doc = "Register `DMACCATxDR` reader"]
pub struct R(crate::R<DMACCATX_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCATX_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCATX_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCATX_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccatx_dr](index.html) module"]
pub struct DMACCATX_DR_SPEC;
impl crate::RegisterSpec for DMACCATX_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccatx_dr::R](R) reader structure"]
impl crate::Readable for DMACCATX_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACCATxDR to value 0"]
impl crate::Resettable for DMACCATX_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
