#[doc = "Register `DMACCARxDR` reader"]
pub struct R(crate::R<DMACCARX_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARX_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARX_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARX_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_dr](index.html) module"]
pub struct DMACCARX_DR_SPEC;
impl crate::RegisterSpec for DMACCARX_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccarx_dr::R](R) reader structure"]
impl crate::Readable for DMACCARX_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACCARxDR to value 0"]
impl crate::Resettable for DMACCARX_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
