#[doc = "Register `MDMA_C26ESR` reader"]
pub struct R(crate::R<MDMA_C26ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C26ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C26ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C26ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEA` reader - TEA"]
pub type TEA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TED` reader - TED"]
pub type TED_R = crate::BitReader<bool>;
#[doc = "Field `TELD` reader - TELD"]
pub type TELD_R = crate::BitReader<bool>;
#[doc = "Field `TEMD` reader - TEMD"]
pub type TEMD_R = crate::BitReader<bool>;
#[doc = "Field `ASE` reader - ASE"]
pub type ASE_R = crate::BitReader<bool>;
#[doc = "Field `BSE` reader - BSE"]
pub type BSE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:6 - TEA"]
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - TED"]
    #[inline(always)]
    pub fn ted(&self) -> TED_R {
        TED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TELD"]
    #[inline(always)]
    pub fn teld(&self) -> TELD_R {
        TELD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEMD"]
    #[inline(always)]
    pub fn temd(&self) -> TEMD_R {
        TEMD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ASE"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BSE"]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "MDMA channel 26 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26esr](index.html) module"]
pub struct MDMA_C26ESR_SPEC;
impl crate::RegisterSpec for MDMA_C26ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c26esr::R](R) reader structure"]
impl crate::Readable for MDMA_C26ESR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDMA_C26ESR to value 0"]
impl crate::Resettable for MDMA_C26ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
