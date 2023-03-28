#[doc = "Register `FMC_BCHISR` reader"]
pub struct R(crate::R<FMC_BCHISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUEF` reader - DUEF"]
pub type DUEF_R = crate::BitReader<bool>;
#[doc = "Field `DERF` reader - DERF"]
pub type DERF_R = crate::BitReader<bool>;
#[doc = "Field `DEFF` reader - DEFF"]
pub type DEFF_R = crate::BitReader<bool>;
#[doc = "Field `DSRF` reader - DSRF"]
pub type DSRF_R = crate::BitReader<bool>;
#[doc = "Field `EPBRF` reader - EPBRF"]
pub type EPBRF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DUEF"]
    #[inline(always)]
    pub fn duef(&self) -> DUEF_R {
        DUEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DERF"]
    #[inline(always)]
    pub fn derf(&self) -> DERF_R {
        DERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DEFF"]
    #[inline(always)]
    pub fn deff(&self) -> DEFF_R {
        DEFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSRF"]
    #[inline(always)]
    pub fn dsrf(&self) -> DSRF_R {
        DSRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EPBRF"]
    #[inline(always)]
    pub fn epbrf(&self) -> EPBRF_R {
        EPBRF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchisr](index.html) module"]
pub struct FMC_BCHISR_SPEC;
impl crate::RegisterSpec for FMC_BCHISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchisr::R](R) reader structure"]
impl crate::Readable for FMC_BCHISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHISR to value 0"]
impl crate::Resettable for FMC_BCHISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
