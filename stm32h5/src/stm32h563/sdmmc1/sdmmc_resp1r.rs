#[doc = "Register `SDMMC_RESP1R` reader"]
pub struct R(crate::R<SDMMC_RESP1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_RESP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_RESP1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_RESP1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUSx` reader - Card status x See ."]
pub type CARDSTATUSX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Card status x See ."]
    #[inline(always)]
    pub fn cardstatusx(&self) -> CARDSTATUSX_R {
        CARDSTATUSX_R::new(self.bits)
    }
}
#[doc = "SDMMC response 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_resp1r](index.html) module"]
pub struct SDMMC_RESP1R_SPEC;
impl crate::RegisterSpec for SDMMC_RESP1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_resp1r::R](R) reader structure"]
impl crate::Readable for SDMMC_RESP1R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDMMC_RESP1R to value 0"]
impl crate::Resettable for SDMMC_RESP1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
