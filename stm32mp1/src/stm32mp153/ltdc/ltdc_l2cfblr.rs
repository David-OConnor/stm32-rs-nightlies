#[doc = "Register `LTDC_L2CFBLR` reader"]
pub struct R(crate::R<LTDC_L2CFBLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2CFBLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2CFBLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2CFBLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2CFBLR` writer"]
pub struct W(crate::W<LTDC_L2CFBLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2CFBLR_SPEC>;
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
impl From<crate::W<LTDC_L2CFBLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2CFBLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBLL` reader - CFBLL"]
pub type CFBLL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFBLL` writer - CFBLL"]
pub type CFBLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L2CFBLR_SPEC, u16, u16, 14, O>;
#[doc = "Field `CFBP` reader - CFBP"]
pub type CFBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFBP` writer - CFBP"]
pub type CFBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L2CFBLR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - CFBLL"]
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - CFBP"]
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - CFBLL"]
    #[inline(always)]
    #[must_use]
    pub fn cfbll(&mut self) -> CFBLL_W<0> {
        CFBLL_W::new(self)
    }
    #[doc = "Bits 16:29 - CFBP"]
    #[inline(always)]
    #[must_use]
    pub fn cfbp(&mut self) -> CFBP_W<16> {
        CFBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the color frame buffer line length and pitch.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cfblr](index.html) module"]
pub struct LTDC_L2CFBLR_SPEC;
impl crate::RegisterSpec for LTDC_L2CFBLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2cfblr::R](R) reader structure"]
impl crate::Readable for LTDC_L2CFBLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cfblr::W](W) writer structure"]
impl crate::Writable for LTDC_L2CFBLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_L2CFBLR to value 0"]
impl crate::Resettable for LTDC_L2CFBLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
