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
#[doc = "Field `CM4L` reader - CM4L"]
pub type CM4L_R = crate::BitReader<bool>;
#[doc = "Field `CM4L` writer - CM4L"]
pub type CM4L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `PVDL` reader - PVDL"]
pub type PVDL_R = crate::BitReader<bool>;
#[doc = "Field `PVDL` writer - PVDL"]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `FLASHL` reader - FLASHL"]
pub type FLASHL_R = crate::BitReader<bool>;
#[doc = "Field `FLASHL` writer - FLASHL"]
pub type FLASHL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `CM7L` reader - CM7L"]
pub type CM7L_R = crate::BitReader<bool>;
#[doc = "Field `CM7L` writer - CM7L"]
pub type CM7L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `BKRAML` reader - BKRAML"]
pub type BKRAML_R = crate::BitReader<bool>;
#[doc = "Field `BKRAML` writer - BKRAML"]
pub type BKRAML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SRAM4L` reader - SRAM4L"]
pub type SRAM4L_R = crate::BitReader<bool>;
#[doc = "Field `SRAM4L` writer - SRAM4L"]
pub type SRAM4L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SRAM3L` reader - SRAM3L"]
pub type SRAM3L_R = crate::BitReader<bool>;
#[doc = "Field `SRAM3L` writer - SRAM3L"]
pub type SRAM3L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SRAM2L` reader - SRAM2L"]
pub type SRAM2L_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2L` writer - SRAM2L"]
pub type SRAM2L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SRAM1L` reader - SRAM1L"]
pub type SRAM1L_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1L` writer - SRAM1L"]
pub type SRAM1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `DTCML` reader - DTCML"]
pub type DTCML_R = crate::BitReader<bool>;
#[doc = "Field `DTCML` writer - DTCML"]
pub type DTCML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `ITCML` reader - ITCML"]
pub type ITCML_R = crate::BitReader<bool>;
#[doc = "Field `ITCML` writer - ITCML"]
pub type ITCML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `AXISRAML` reader - AXISRAML"]
pub type AXISRAML_R = crate::BitReader<bool>;
#[doc = "Field `AXISRAML` writer - AXISRAML"]
pub type AXISRAML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CM4L"]
    #[inline(always)]
    pub fn cm4l(&self) -> CM4L_R {
        CM4L_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PVDL"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLASHL"]
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CM7L"]
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BKRAML"]
    #[inline(always)]
    pub fn bkraml(&self) -> BKRAML_R {
        BKRAML_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM4L"]
    #[inline(always)]
    pub fn sram4l(&self) -> SRAM4L_R {
        SRAM4L_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM3L"]
    #[inline(always)]
    pub fn sram3l(&self) -> SRAM3L_R {
        SRAM3L_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM2L"]
    #[inline(always)]
    pub fn sram2l(&self) -> SRAM2L_R {
        SRAM2L_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM1L"]
    #[inline(always)]
    pub fn sram1l(&self) -> SRAM1L_R {
        SRAM1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DTCML"]
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ITCML"]
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AXISRAML"]
    #[inline(always)]
    pub fn axisraml(&self) -> AXISRAML_R {
        AXISRAML_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CM4L"]
    #[inline(always)]
    #[must_use]
    pub fn cm4l(&mut self) -> CM4L_W<0> {
        CM4L_W::new(self)
    }
    #[doc = "Bit 2 - PVDL"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    #[doc = "Bit 3 - FLASHL"]
    #[inline(always)]
    #[must_use]
    pub fn flashl(&mut self) -> FLASHL_W<3> {
        FLASHL_W::new(self)
    }
    #[doc = "Bit 6 - CM7L"]
    #[inline(always)]
    #[must_use]
    pub fn cm7l(&mut self) -> CM7L_W<6> {
        CM7L_W::new(self)
    }
    #[doc = "Bit 7 - BKRAML"]
    #[inline(always)]
    #[must_use]
    pub fn bkraml(&mut self) -> BKRAML_W<7> {
        BKRAML_W::new(self)
    }
    #[doc = "Bit 9 - SRAM4L"]
    #[inline(always)]
    #[must_use]
    pub fn sram4l(&mut self) -> SRAM4L_W<9> {
        SRAM4L_W::new(self)
    }
    #[doc = "Bit 10 - SRAM3L"]
    #[inline(always)]
    #[must_use]
    pub fn sram3l(&mut self) -> SRAM3L_W<10> {
        SRAM3L_W::new(self)
    }
    #[doc = "Bit 11 - SRAM2L"]
    #[inline(always)]
    #[must_use]
    pub fn sram2l(&mut self) -> SRAM2L_W<11> {
        SRAM2L_W::new(self)
    }
    #[doc = "Bit 12 - SRAM1L"]
    #[inline(always)]
    #[must_use]
    pub fn sram1l(&mut self) -> SRAM1L_W<12> {
        SRAM1L_W::new(self)
    }
    #[doc = "Bit 13 - DTCML"]
    #[inline(always)]
    #[must_use]
    pub fn dtcml(&mut self) -> DTCML_W<13> {
        DTCML_W::new(self)
    }
    #[doc = "Bit 14 - ITCML"]
    #[inline(always)]
    #[must_use]
    pub fn itcml(&mut self) -> ITCML_W<14> {
        ITCML_W::new(self)
    }
    #[doc = "Bit 15 - AXISRAML"]
    #[inline(always)]
    #[must_use]
    pub fn axisraml(&mut self) -> AXISRAML_W<15> {
        AXISRAML_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
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
