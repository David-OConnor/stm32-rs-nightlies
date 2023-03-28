#[doc = "Register `RAMCFG_M2SEAR` reader"]
pub struct R(crate::R<RAMCFG_M2SEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMCFG_M2SEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMCFG_M2SEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMCFG_M2SEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ESEA` reader - ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error."]
pub type ESEA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error."]
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG memory 2 ECC single error address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg_m2sear](index.html) module"]
pub struct RAMCFG_M2SEAR_SPEC;
impl crate::RegisterSpec for RAMCFG_M2SEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramcfg_m2sear::R](R) reader structure"]
impl crate::Readable for RAMCFG_M2SEAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAMCFG_M2SEAR to value 0"]
impl crate::Resettable for RAMCFG_M2SEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
