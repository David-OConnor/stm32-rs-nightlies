#[doc = "Register `FMC_BCHDSR3` reader"]
pub struct R(crate::R<FMC_BCHDSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHDSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHDSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHDSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EBP5` reader - EBP5"]
pub type EBP5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EBP6` reader - EBP6"]
pub type EBP6_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - EBP5"]
    #[inline(always)]
    pub fn ebp5(&self) -> EBP5_R {
        EBP5_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP6"]
    #[inline(always)]
    pub fn ebp6(&self) -> EBP6_R {
        EBP6_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr3](index.html) module"]
pub struct FMC_BCHDSR3_SPEC;
impl crate::RegisterSpec for FMC_BCHDSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchdsr3::R](R) reader structure"]
impl crate::Readable for FMC_BCHDSR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHDSR3 to value 0"]
impl crate::Resettable for FMC_BCHDSR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
