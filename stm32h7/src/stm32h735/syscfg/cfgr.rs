#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDL` reader - Programmable voltage detector lockup bit"]
pub type PVDL_R = crate::BitReader<bool>;
#[doc = "Field `PVDL` writer - Programmable voltage detector lockup bit"]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `FLASHL` reader - FLASH double error lockup bit"]
pub type FLASHL_R = crate::BitReader<bool>;
#[doc = "Field `FLASHL` writer - FLASH double error lockup bit"]
pub type FLASHL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `CM7L` reader - CPU lockup bit"]
pub type CM7L_R = crate::BitReader<bool>;
#[doc = "Field `CM7L` writer - CPU lockup bit"]
pub type CM7L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `BKRAML` reader - Backup RAM Double error lockup bit"]
pub type BKRAML_R = crate::BitReader<bool>;
#[doc = "Field `BKRAML` writer - Backup RAM Double error lockup bit"]
pub type BKRAML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SRAM4L` reader - SRAM4 Double error lockup bit"]
pub type SRAM4L_R = crate::BitReader<bool>;
#[doc = "Field `SRAM4L` writer - SRAM4 Double error lockup bit"]
pub type SRAM4L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SRAM2L` reader - SRAM2 Double error lockup bit"]
pub type SRAM2L_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2L` writer - SRAM2 Double error lockup bit"]
pub type SRAM2L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SRAM1L` reader - SRAM1 Double error lockup bit"]
pub type SRAM1L_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1L` writer - SRAM1 Double error lockup bit"]
pub type SRAM1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `DTCML` reader - DTCM-RAM Double error lockup bit"]
pub type DTCML_R = crate::BitReader<bool>;
#[doc = "Field `DTCML` writer - DTCM-RAM Double error lockup bit"]
pub type DTCML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `ITCML` reader - ITCM-RAM Double error lockup bit"]
pub type ITCML_R = crate::BitReader<bool>;
#[doc = "Field `ITCML` writer - ITCM-RAM Double error lockup bit"]
pub type ITCML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `AXIRAML` reader - AXISRAM Double error lockup bit"]
pub type AXIRAML_R = crate::BitReader<bool>;
#[doc = "Field `AXIRAML` writer - AXISRAM Double error lockup bit"]
pub type AXIRAML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Programmable voltage detector lockup bit"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLASH double error lockup bit"]
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU lockup bit"]
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Backup RAM Double error lockup bit"]
    #[inline(always)]
    pub fn bkraml(&self) -> BKRAML_R {
        BKRAML_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM4 Double error lockup bit"]
    #[inline(always)]
    pub fn sram4l(&self) -> SRAM4L_R {
        SRAM4L_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM2 Double error lockup bit"]
    #[inline(always)]
    pub fn sram2l(&self) -> SRAM2L_R {
        SRAM2L_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM1 Double error lockup bit"]
    #[inline(always)]
    pub fn sram1l(&self) -> SRAM1L_R {
        SRAM1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DTCM-RAM Double error lockup bit"]
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ITCM-RAM Double error lockup bit"]
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AXISRAM Double error lockup bit"]
    #[inline(always)]
    pub fn axiraml(&self) -> AXIRAML_R {
        AXIRAML_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Programmable voltage detector lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    #[doc = "Bit 3 - FLASH double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn flashl(&mut self) -> FLASHL_W<3> {
        FLASHL_W::new(self)
    }
    #[doc = "Bit 6 - CPU lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn cm7l(&mut self) -> CM7L_W<6> {
        CM7L_W::new(self)
    }
    #[doc = "Bit 7 - Backup RAM Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn bkraml(&mut self) -> BKRAML_W<7> {
        BKRAML_W::new(self)
    }
    #[doc = "Bit 9 - SRAM4 Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram4l(&mut self) -> SRAM4L_W<9> {
        SRAM4L_W::new(self)
    }
    #[doc = "Bit 11 - SRAM2 Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram2l(&mut self) -> SRAM2L_W<11> {
        SRAM2L_W::new(self)
    }
    #[doc = "Bit 12 - SRAM1 Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram1l(&mut self) -> SRAM1L_W<12> {
        SRAM1L_W::new(self)
    }
    #[doc = "Bit 13 - DTCM-RAM Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtcml(&mut self) -> DTCML_W<13> {
        DTCML_W::new(self)
    }
    #[doc = "Bit 14 - ITCM-RAM Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn itcml(&mut self) -> ITCML_W<14> {
        ITCML_W::new(self)
    }
    #[doc = "Bit 15 - AXISRAM Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn axiraml(&mut self) -> AXIRAML_W<15> {
        AXIRAML_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer break lockup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
