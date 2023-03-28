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
#[doc = "Field `FRAME_IE` reader - FRAME_IE"]
pub type FRAME_IE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_IE` writer - FRAME_IE"]
pub type FRAME_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
#[doc = "Field `OVR_IE` reader - OVR_IE"]
pub type OVR_IE_R = crate::BitReader<bool>;
#[doc = "Field `OVR_IE` writer - OVR_IE"]
pub type OVR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
#[doc = "Field `ERR_IE` reader - ERR_IE"]
pub type ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `ERR_IE` writer - ERR_IE"]
pub type ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
#[doc = "Field `VSYNC_IE` reader - VSYNC_IE"]
pub type VSYNC_IE_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC_IE` writer - VSYNC_IE"]
pub type VSYNC_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
#[doc = "Field `LINE_IE` reader - LINE_IE"]
pub type LINE_IE_R = crate::BitReader<bool>;
#[doc = "Field `LINE_IE` writer - LINE_IE"]
pub type LINE_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FRAME_IE"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVR_IE"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERR_IE"]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC_IE"]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LINE_IE"]
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRAME_IE"]
    #[inline(always)]
    #[must_use]
    pub fn frame_ie(&mut self) -> FRAME_IE_W<0> {
        FRAME_IE_W::new(self)
    }
    #[doc = "Bit 1 - OVR_IE"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<1> {
        OVR_IE_W::new(self)
    }
    #[doc = "Bit 2 - ERR_IE"]
    #[inline(always)]
    #[must_use]
    pub fn err_ie(&mut self) -> ERR_IE_W<2> {
        ERR_IE_W::new(self)
    }
    #[doc = "Bit 3 - VSYNC_IE"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W<3> {
        VSYNC_IE_W::new(self)
    }
    #[doc = "Bit 4 - LINE_IE"]
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
#[doc = "The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_ier](index.html) module"]
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
