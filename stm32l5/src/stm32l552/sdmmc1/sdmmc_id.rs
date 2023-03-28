#[doc = "Register `SDMMC_ID` reader"]
pub struct R(crate::R<SDMMC_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IP_ID` reader - SDMMC IP identification."]
pub type IP_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SDMMC IP identification."]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(self.bits)
    }
}
#[doc = "SDMMC IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_id](index.html) module"]
pub struct SDMMC_ID_SPEC;
impl crate::RegisterSpec for SDMMC_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_id::R](R) reader structure"]
impl crate::Readable for SDMMC_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDMMC_ID to value 0x0014_0022"]
impl crate::Resettable for SDMMC_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0014_0022;
}
