#[doc = "Register `LTDC_L2PFCR` reader"]
pub struct R(crate::R<LTDC_L2PFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2PFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2PFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2PFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2PFCR` writer"]
pub struct W(crate::W<LTDC_L2PFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2PFCR_SPEC>;
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
impl From<crate::W<LTDC_L2PFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2PFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF` reader - PF"]
pub type PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PF` writer - PF"]
pub type PF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L2PFCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - PF"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PF"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<0> {
        PF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2pfcr](index.html) module"]
pub struct LTDC_L2PFCR_SPEC;
impl crate::RegisterSpec for LTDC_L2PFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2pfcr::R](R) reader structure"]
impl crate::Readable for LTDC_L2PFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2pfcr::W](W) writer structure"]
impl crate::Writable for LTDC_L2PFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_L2PFCR to value 0"]
impl crate::Resettable for LTDC_L2PFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
