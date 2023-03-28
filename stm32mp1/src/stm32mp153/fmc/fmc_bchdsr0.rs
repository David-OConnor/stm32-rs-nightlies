#[doc = "Register `FMC_BCHDSR0` reader"]
pub struct R(crate::R<FMC_BCHDSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHDSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHDSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHDSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUE` reader - DUE"]
pub type DUE_R = crate::BitReader<bool>;
#[doc = "Field `DEF` reader - DEF"]
pub type DEF_R = crate::BitReader<bool>;
#[doc = "Field `DEN` reader - DEN"]
pub type DEN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - DUE"]
    #[inline(always)]
    pub fn due(&self) -> DUE_R {
        DUE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DEF"]
    #[inline(always)]
    pub fn def(&self) -> DEF_R {
        DEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr0](index.html) module"]
pub struct FMC_BCHDSR0_SPEC;
impl crate::RegisterSpec for FMC_BCHDSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchdsr0::R](R) reader structure"]
impl crate::Readable for FMC_BCHDSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHDSR0 to value 0"]
impl crate::Resettable for FMC_BCHDSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
