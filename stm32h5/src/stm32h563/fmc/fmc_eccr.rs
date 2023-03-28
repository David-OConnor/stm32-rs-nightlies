#[doc = "Register `FMC_ECCR` reader"]
pub struct R(crate::R<FMC_ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_ECCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC` reader - ECC result This field contains the value computed by the ECC computation logic. Table 99 describes the contents of these bitfields."]
pub type ECC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result This field contains the value computed by the ECC computation logic. Table 99 describes the contents of these bitfields."]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
#[doc = "ECC result registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_eccr](index.html) module"]
pub struct FMC_ECCR_SPEC;
impl crate::RegisterSpec for FMC_ECCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_eccr::R](R) reader structure"]
impl crate::Readable for FMC_ECCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_ECCR to value 0"]
impl crate::Resettable for FMC_ECCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
