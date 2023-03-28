#[doc = "Register `SPDIFRX_SR` reader"]
pub struct R(crate::R<SPDIFRX_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXNE` reader - RXNE"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `CSRNE` reader - CSRNE"]
pub type CSRNE_R = crate::BitReader<bool>;
#[doc = "Field `PERR` reader - PERR"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `OVR` reader - OVR"]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `SBD` reader - SBD"]
pub type SBD_R = crate::BitReader<bool>;
#[doc = "Field `SYNCD` reader - SYNCD"]
pub type SYNCD_R = crate::BitReader<bool>;
#[doc = "Field `FERR` reader - FERR"]
pub type FERR_R = crate::BitReader<bool>;
#[doc = "Field `SERR` reader - SERR"]
pub type SERR_R = crate::BitReader<bool>;
#[doc = "Field `TERR` reader - TERR"]
pub type TERR_R = crate::BitReader<bool>;
#[doc = "Field `WIDTH5` reader - WIDTH5"]
pub type WIDTH5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSRNE"]
    #[inline(always)]
    pub fn csrne(&self) -> CSRNE_R {
        CSRNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PERR"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SBD"]
    #[inline(always)]
    pub fn sbd(&self) -> SBD_R {
        SBD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNCD"]
    #[inline(always)]
    pub fn syncd(&self) -> SYNCD_R {
        SYNCD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FERR"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SERR"]
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TERR"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - WIDTH5"]
    #[inline(always)]
    pub fn width5(&self) -> WIDTH5_R {
        WIDTH5_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_sr](index.html) module"]
pub struct SPDIFRX_SR_SPEC;
impl crate::RegisterSpec for SPDIFRX_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_sr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPDIFRX_SR to value 0"]
impl crate::Resettable for SPDIFRX_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
