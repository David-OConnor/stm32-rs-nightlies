#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture complete interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_ISC_AW {
    #[doc = "1: Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<FRAME_ISC_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAME_ISC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME_ISC` writer - Capture complete interrupt status clear"]
pub type FRAME_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, FRAME_ISC_AW, O>;
impl<'a, const O: u8> FRAME_ISC_W<'a, O> {
    #[doc = "Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRAME_ISC_AW::Clear)
    }
}
#[doc = "Overrun interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_ISC_AW {
    #[doc = "1: Setting this bit clears the OVR_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<OVR_ISC_AW> for bool {
    #[inline(always)]
    fn from(variant: OVR_ISC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_ISC` writer - Overrun interrupt status clear"]
pub type OVR_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, OVR_ISC_AW, O>;
impl<'a, const O: u8> OVR_ISC_W<'a, O> {
    #[doc = "Setting this bit clears the OVR_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVR_ISC_AW::Clear)
    }
}
#[doc = "Synchronization error interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_ISC_AW {
    #[doc = "1: Setting this bit clears the ERR_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<ERR_ISC_AW> for bool {
    #[inline(always)]
    fn from(variant: ERR_ISC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_ISC` writer - Synchronization error interrupt status clear"]
pub type ERR_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ERR_ISC_AW, O>;
impl<'a, const O: u8> ERR_ISC_W<'a, O> {
    #[doc = "Setting this bit clears the ERR_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERR_ISC_AW::Clear)
    }
}
#[doc = "Vertical synch interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_ISC_AW {
    #[doc = "1: Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<VSYNC_ISC_AW> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_ISC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNC_ISC` writer - Vertical synch interrupt status clear"]
pub type VSYNC_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, VSYNC_ISC_AW, O>;
impl<'a, const O: u8> VSYNC_ISC_W<'a, O> {
    #[doc = "Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(VSYNC_ISC_AW::Clear)
    }
}
#[doc = "line interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_ISC_AW {
    #[doc = "1: Setting this bit clears the LINE_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<LINE_ISC_AW> for bool {
    #[inline(always)]
    fn from(variant: LINE_ISC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE_ISC` writer - line interrupt status clear"]
pub type LINE_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, LINE_ISC_AW, O>;
impl<'a, const O: u8> LINE_ISC_W<'a, O> {
    #[doc = "Setting this bit clears the LINE_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LINE_ISC_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Capture complete interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W<0> {
        FRAME_ISC_W::new(self)
    }
    #[doc = "Bit 1 - Overrun interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<1> {
        OVR_ISC_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization error interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_isc(&mut self) -> ERR_ISC_W<2> {
        ERR_ISC_W::new(self)
    }
    #[doc = "Bit 3 - Vertical synch interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W<3> {
        VSYNC_ISC_W::new(self)
    }
    #[doc = "Bit 4 - line interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn line_isc(&mut self) -> LINE_ISC_W<4> {
        LINE_ISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
