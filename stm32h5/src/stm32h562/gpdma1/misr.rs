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
#[doc = "Field `MIS0` reader - masked interrupt status of channel x (x = 7 to 0)"]
pub type MIS0_R = crate::BitReader<bool>;
#[doc = "Field `MIS1` reader - masked interrupt status of channel x (x = 7 to 0)"]
pub type MIS1_R = crate::BitReader<bool>;
#[doc = "Field `MIS2` reader - masked interrupt status of channel x (x = 7 to 0)"]
pub type MIS2_R = crate::BitReader<bool>;
#[doc = "Field `MIS3` reader - masked interrupt status of channel x (x = 7 to 0)"]
pub type MIS3_R = crate::BitReader<bool>;
#[doc = "Field `MIS4` reader - masked interrupt status of channel x (x = 7 to 0)"]
pub type MIS4_R = crate::BitReader<bool>;
#[doc = "Field `MIS5` reader - masked interrupt status of channel x (x = 7 to 0)"]
pub type MIS5_R = crate::BitReader<bool>;
#[doc = "Field `MIS6` reader - masked interrupt status of channel x (x = 7 to 0)"]
pub type MIS6_R = crate::BitReader<bool>;
#[doc = "Field `MIS7` reader - masked interrupt status of channel x (x = 7 to 0)"]
pub type MIS7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - masked interrupt status of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis0(&self) -> MIS0_R {
        MIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - masked interrupt status of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis1(&self) -> MIS1_R {
        MIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - masked interrupt status of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis2(&self) -> MIS2_R {
        MIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - masked interrupt status of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis3(&self) -> MIS3_R {
        MIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - masked interrupt status of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis4(&self) -> MIS4_R {
        MIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - masked interrupt status of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis5(&self) -> MIS5_R {
        MIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - masked interrupt status of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis6(&self) -> MIS6_R {
        MIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - masked interrupt status of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis7(&self) -> MIS7_R {
        MIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "GPDMA non-secure masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
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
