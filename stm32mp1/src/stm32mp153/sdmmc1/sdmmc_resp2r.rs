#[doc = "Register `SDMMC_RESP2R` reader"]
pub struct R(crate::R<SDMMC_RESP2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_RESP2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_RESP2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_RESP2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUS2` reader - CARDSTATUS2"]
pub type CARDSTATUS2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS2"]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_resp2r](index.html) module"]
pub struct SDMMC_RESP2R_SPEC;
impl crate::RegisterSpec for SDMMC_RESP2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_resp2r::R](R) reader structure"]
impl crate::Readable for SDMMC_RESP2R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDMMC_RESP2R to value 0"]
impl crate::Resettable for SDMMC_RESP2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}