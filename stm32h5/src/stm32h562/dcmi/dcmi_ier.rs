#[doc = "Register `DCMI_IER` reader"]
pub struct R(crate::R<DCMI_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCMI_IER` writer"]
pub struct W(crate::W<DCMI_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_IER_SPEC>;
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
impl From<crate::W<DCMI_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_IE` reader - Capture complete interrupt enable"]
pub type FRAME_IE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_IE` writer - Capture complete interrupt enable"]
pub type FRAME_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
#[doc = "Field `OVR_IE` reader - Overrun interrupt enable"]
pub type OVR_IE_R = crate::BitReader<bool>;
#[doc = "Field `OVR_IE` writer - Overrun interrupt enable"]
pub type OVR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
#[doc = "Field `ERR_IE` reader - Synchronization error interrupt enable Note: This bit is available only in embedded synchronization mode."]
pub type ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `ERR_IE` writer - Synchronization error interrupt enable Note: This bit is available only in embedded synchronization mode."]
pub type ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
#[doc = "Field `VSYNC_IE` reader - DCMI_VSYNC interrupt enable The active state of the DCMI_VSYNC signal is defined by the VSPOL bit."]
pub type VSYNC_IE_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC_IE` writer - DCMI_VSYNC interrupt enable The active state of the DCMI_VSYNC signal is defined by the VSPOL bit."]
pub type VSYNC_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
#[doc = "Field `LINE_IE` reader - Line interrupt enable"]
pub type LINE_IE_R = crate::BitReader<bool>;
#[doc = "Field `LINE_IE` writer - Line interrupt enable"]
pub type LINE_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable Note: This bit is available only in embedded synchronization mode."]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DCMI_VSYNC interrupt enable The active state of the DCMI_VSYNC signal is defined by the VSPOL bit."]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame_ie(&mut self) -> FRAME_IE_W<0> {
        FRAME_IE_W::new(self)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<1> {
        OVR_IE_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable Note: This bit is available only in embedded synchronization mode."]
    #[inline(always)]
    #[must_use]
    pub fn err_ie(&mut self) -> ERR_IE_W<2> {
        ERR_IE_W::new(self)
    }
    #[doc = "Bit 3 - DCMI_VSYNC interrupt enable The active state of the DCMI_VSYNC signal is defined by the VSPOL bit."]
    #[inline(always)]
    #[must_use]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W<3> {
        VSYNC_IE_W::new(self)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn line_ie(&mut self) -> LINE_IE_W<4> {
        LINE_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCMI interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_ier](index.html) module"]
pub struct DCMI_IER_SPEC;
impl crate::RegisterSpec for DCMI_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_ier::R](R) reader structure"]
impl crate::Readable for DCMI_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcmi_ier::W](W) writer structure"]
impl crate::Writable for DCMI_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCMI_IER to value 0"]
impl crate::Resettable for DCMI_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
