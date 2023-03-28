#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear wakeup flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1W_AW {
    #[doc = "1: Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0."]
    Clear = 1,
}
impl From<CWUF1W_AW> for bool {
    #[inline(always)]
    fn from(variant: CWUF1W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1"]
pub type CWUF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CWUF1W_AW, O>;
impl<'a, const O: u8> CWUF1_W<'a, O> {
    #[doc = "Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF1W_AW::Clear)
    }
}
#[doc = "Clear wakeup flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF2W_AW {
    #[doc = "1: Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0."]
    Clear = 1,
}
impl From<CWUF2W_AW> for bool {
    #[inline(always)]
    fn from(variant: CWUF2W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2"]
pub type CWUF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CWUF2W_AW, O>;
impl<'a, const O: u8> CWUF2_W<'a, O> {
    #[doc = "Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF2W_AW::Clear)
    }
}
#[doc = "Clear wakeup flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF3W_AW {
    #[doc = "1: Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0."]
    Clear = 1,
}
impl From<CWUF3W_AW> for bool {
    #[inline(always)]
    fn from(variant: CWUF3W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3"]
pub type CWUF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CWUF3W_AW, O>;
impl<'a, const O: u8> CWUF3_W<'a, O> {
    #[doc = "Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF3W_AW::Clear)
    }
}
#[doc = "Clear wakeup PVD interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWPVDFW_AW {
    #[doc = "1: Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0."]
    Clear = 1,
}
impl From<CWPVDFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CWPVDFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWPVDF` writer - Clear wakeup PVD interrupt flag"]
pub type CWPVDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CWPVDFW_AW, O>;
impl<'a, const O: u8> CWPVDF_W<'a, O> {
    #[doc = "Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWPVDFW_AW::Clear)
    }
}
#[doc = "Clear wakeup Radio BUSY flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWRFBUSYFW_AW {
    #[doc = "1: Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0."]
    Clear = 1,
}
impl From<CWRFBUSYFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CWRFBUSYFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWRFBUSYF` writer - Clear wakeup Radio BUSY flag"]
pub type CWRFBUSYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CWRFBUSYFW_AW, O>;
impl<'a, const O: u8> CWRFBUSYF_W<'a, O> {
    #[doc = "Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWRFBUSYFW_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<0> {
        CWUF1_W::new(self)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<1> {
        CWUF2_W::new(self)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<2> {
        CWUF3_W::new(self)
    }
    #[doc = "Bit 8 - Clear wakeup PVD interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwpvdf(&mut self) -> CWPVDF_W<8> {
        CWPVDF_W::new(self)
    }
    #[doc = "Bit 11 - Clear wakeup Radio BUSY flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwrfbusyf(&mut self) -> CWRFBUSYF_W<11> {
        CWRFBUSYF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
