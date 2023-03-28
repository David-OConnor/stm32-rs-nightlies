#[doc = "Register `LTDC_L2WVPCR` reader"]
pub struct R(crate::R<LTDC_L2WVPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2WVPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2WVPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2WVPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2WVPCR` writer"]
pub struct W(crate::W<LTDC_L2WVPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2WVPCR_SPEC>;
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
impl From<crate::W<LTDC_L2WVPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2WVPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVSTPOS` reader - WVSTPOS"]
pub type WVSTPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVSTPOS` writer - WVSTPOS"]
pub type WVSTPOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LTDC_L2WVPCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `WVSPPOS` reader - WVSPPOS"]
pub type WVSPPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVSPPOS` writer - WVSPPOS"]
pub type WVSPPOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LTDC_L2WVPCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - WVSTPOS"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - WVSPPOS"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WVSTPOS"]
    #[inline(always)]
    #[must_use]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<0> {
        WVSTPOS_W::new(self)
    }
    #[doc = "Bits 16:27 - WVSPPOS"]
    #[inline(always)]
    #[must_use]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<16> {
        WVSPPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2wvpcr](index.html) module"]
pub struct LTDC_L2WVPCR_SPEC;
impl crate::RegisterSpec for LTDC_L2WVPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2wvpcr::R](R) reader structure"]
impl crate::Readable for LTDC_L2WVPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2wvpcr::W](W) writer structure"]
impl crate::Writable for LTDC_L2WVPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_L2WVPCR to value 0"]
impl crate::Resettable for LTDC_L2WVPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
