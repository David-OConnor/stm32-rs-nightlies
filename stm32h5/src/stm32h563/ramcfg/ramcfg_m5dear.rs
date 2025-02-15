#[doc = "Register `RAMCFG_M5DEAR` reader"]
pub struct R(crate::R<RAMCFG_M5DEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMCFG_M5DEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMCFG_M5DEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMCFG_M5DEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EDEA` reader - ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error."]
pub type EDEA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error."]
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG memory 5 ECC double error address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg_m5dear](index.html) module"]
pub struct RAMCFG_M5DEAR_SPEC;
impl crate::RegisterSpec for RAMCFG_M5DEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramcfg_m5dear::R](R) reader structure"]
impl crate::Readable for RAMCFG_M5DEAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAMCFG_M5DEAR to value 0"]
impl crate::Resettable for RAMCFG_M5DEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
