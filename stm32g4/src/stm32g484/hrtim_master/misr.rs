#[doc = "Register `MISR` reader"]
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MCMP1` reader - Master Compare 1 Interrupt Flag"]
pub type MCMP1_R = crate::BitReader<bool>;
#[doc = "Field `MCMP2` reader - Master Compare 2 Interrupt Flag"]
pub type MCMP2_R = crate::BitReader<bool>;
#[doc = "Field `MCMP3` reader - Master Compare 3 Interrupt Flag"]
pub type MCMP3_R = crate::BitReader<bool>;
#[doc = "Field `MCMP4` reader - Master Compare 4 Interrupt Flag"]
pub type MCMP4_R = crate::BitReader<bool>;
#[doc = "Field `MREP` reader - Master Repetition Interrupt Flag"]
pub type MREP_R = crate::BitReader<bool>;
#[doc = "Field `SYNC` reader - Sync Input Interrupt Flag"]
pub type SYNC_R = crate::BitReader<bool>;
#[doc = "Field `MUPD` reader - Master Update Interrupt Flag"]
pub type MUPD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Master Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync Input Interrupt Flag"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Update Interrupt Flag"]
    #[inline(always)]
    pub fn mupd(&self) -> MUPD_R {
        MUPD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Master Timer Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misr::R](R) reader structure"]
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
