#[doc = "Register `SDMMC_DCNTR` reader"]
pub struct R(crate::R<SDMMC_DCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_DCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_DCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_DCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATACOUNT` reader - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
pub type DATACOUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "SDMMC data counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dcntr](index.html) module"]
pub struct SDMMC_DCNTR_SPEC;
impl crate::RegisterSpec for SDMMC_DCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_dcntr::R](R) reader structure"]
impl crate::Readable for SDMMC_DCNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDMMC_DCNTR to value 0"]
impl crate::Resettable for SDMMC_DCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
