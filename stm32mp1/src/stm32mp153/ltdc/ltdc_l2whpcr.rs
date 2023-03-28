#[doc = "Register `LTDC_L2WHPCR` reader"]
pub struct R(crate::R<LTDC_L2WHPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2WHPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2WHPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2WHPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2WHPCR` writer"]
pub struct W(crate::W<LTDC_L2WHPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2WHPCR_SPEC>;
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
impl From<crate::W<LTDC_L2WHPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2WHPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WHSTPOS` reader - WHSTPOS"]
pub type WHSTPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WHSTPOS` writer - WHSTPOS"]
pub type WHSTPOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LTDC_L2WHPCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `WHSPPOS` reader - WHSPPOS"]
pub type WHSPPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WHSPPOS` writer - WHSPPOS"]
pub type WHSPPOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LTDC_L2WHPCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - WHSTPOS"]
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - WHSPPOS"]
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WHSTPOS"]
    #[inline(always)]
    #[must_use]
    pub fn whstpos(&mut self) -> WHSTPOS_W<0> {
        WHSTPOS_W::new(self)
    }
    #[doc = "Bits 16:27 - WHSPPOS"]
    #[inline(always)]
    #[must_use]
    pub fn whsppos(&mut self) -> WHSPPOS_W<16> {
        WHSPPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\\[11:0\\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\\[11:0\\]
bits in the LTDC_AWCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2whpcr](index.html) module"]
pub struct LTDC_L2WHPCR_SPEC;
impl crate::RegisterSpec for LTDC_L2WHPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2whpcr::R](R) reader structure"]
impl crate::Readable for LTDC_L2WHPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2whpcr::W](W) writer structure"]
impl crate::Writable for LTDC_L2WHPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_L2WHPCR to value 0"]
impl crate::Resettable for LTDC_L2WHPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
