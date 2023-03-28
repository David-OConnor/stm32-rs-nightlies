#[doc = "Register `BDSR` reader"]
pub struct R(crate::R<BDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BRRDY` reader - backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
pub type BRRDY_R = crate::BitReader<bool>;
#[doc = "Field `VBATL` reader - V<sub>BAT</sub> level monitoring versus low threshold"]
pub type VBATL_R = crate::BitReader<bool>;
#[doc = "Field `VBATH` reader - V<sub>BAT</sub> level monitoring versus high threshold"]
pub type VBATH_R = crate::BitReader<bool>;
#[doc = "Field `TEMPL` reader - temperature level monitoring versus low threshold"]
pub type TEMPL_R = crate::BitReader<bool>;
#[doc = "Field `TEMPH` reader - temperature level monitoring versus high threshold"]
pub type TEMPH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 16 - backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - V<sub>BAT</sub> level monitoring versus low threshold"]
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - V<sub>BAT</sub> level monitoring versus high threshold"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - temperature level monitoring versus low threshold"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - temperature level monitoring versus high threshold"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "PWR Backup domain status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdsr](index.html) module"]
pub struct BDSR_SPEC;
impl crate::RegisterSpec for BDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdsr::R](R) reader structure"]
impl crate::Readable for BDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BDSR to value 0"]
impl crate::Resettable for BDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
