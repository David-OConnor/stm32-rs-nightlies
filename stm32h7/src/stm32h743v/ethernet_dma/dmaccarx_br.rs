#[doc = "Register `DMACCARxBR` reader"]
pub struct R(crate::R<DMACCARX_BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARX_BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARX_BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARX_BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer"]
pub type CURRBUFAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application receive buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_br](index.html) module"]
pub struct DMACCARX_BR_SPEC;
impl crate::RegisterSpec for DMACCARX_BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccarx_br::R](R) reader structure"]
impl crate::Readable for DMACCARX_BR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACCARxBR to value 0"]
impl crate::Resettable for DMACCARX_BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
