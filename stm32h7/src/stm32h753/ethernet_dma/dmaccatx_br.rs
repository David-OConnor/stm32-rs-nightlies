#[doc = "Register `DMACCATxBR` reader"]
pub struct R(crate::R<DMACCATX_BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCATX_BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCATX_BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCATX_BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application transmit buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccatx_br](index.html) module"]
pub struct DMACCATX_BR_SPEC;
impl crate::RegisterSpec for DMACCATX_BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccatx_br::R](R) reader structure"]
impl crate::Readable for DMACCATX_BR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACCATxBR to value 0"]
impl crate::Resettable for DMACCATX_BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
