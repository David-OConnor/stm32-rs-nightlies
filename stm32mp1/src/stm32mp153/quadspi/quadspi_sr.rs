#[doc = "Register `QUADSPI_SR` reader"]
pub struct R(crate::R<QUADSPI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEF` reader - TEF"]
pub type TEF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` reader - TCF"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `FTF` reader - FTF"]
pub type FTF_R = crate::BitReader<bool>;
#[doc = "Field `SMF` reader - SMF"]
pub type SMF_R = crate::BitReader<bool>;
#[doc = "Field `TOF` reader - TOF"]
pub type TOF_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `FLEVEL` reader - FLEVEL"]
pub type FLEVEL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - TEF"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FTF"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMF"]
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TOF"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - FLEVEL"]
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "QUADSPI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_sr](index.html) module"]
pub struct QUADSPI_SR_SPEC;
impl crate::RegisterSpec for QUADSPI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_sr::R](R) reader structure"]
impl crate::Readable for QUADSPI_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUADSPI_SR to value 0"]
impl crate::Resettable for QUADSPI_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
