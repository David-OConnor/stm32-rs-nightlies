#[doc = "Register `LTDC_BCCR` reader"]
pub struct R(crate::R<LTDC_BCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_BCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_BCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_BCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_BCCR` writer"]
pub struct W(crate::W<LTDC_BCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_BCCR_SPEC>;
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
impl From<crate::W<LTDC_BCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_BCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCBLUE` reader - BCBLUE"]
pub type BCBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCBLUE` writer - BCBLUE"]
pub type BCBLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_BCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BCGREEN` reader - BCGREEN"]
pub type BCGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCGREEN` writer - BCGREEN"]
pub type BCGREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_BCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BCRED` reader - BCRED"]
pub type BCRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCRED` writer - BCRED"]
pub type BCRED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_BCCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - BCBLUE"]
    #[inline(always)]
    pub fn bcblue(&self) -> BCBLUE_R {
        BCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BCGREEN"]
    #[inline(always)]
    pub fn bcgreen(&self) -> BCGREEN_R {
        BCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BCRED"]
    #[inline(always)]
    pub fn bcred(&self) -> BCRED_R {
        BCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCBLUE"]
    #[inline(always)]
    #[must_use]
    pub fn bcblue(&mut self) -> BCBLUE_W<0> {
        BCBLUE_W::new(self)
    }
    #[doc = "Bits 8:15 - BCGREEN"]
    #[inline(always)]
    #[must_use]
    pub fn bcgreen(&mut self) -> BCGREEN_W<8> {
        BCGREEN_W::new(self)
    }
    #[doc = "Bits 16:23 - BCRED"]
    #[inline(always)]
    #[must_use]
    pub fn bcred(&mut self) -> BCRED_W<16> {
        BCRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the background color (RGB888).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_bccr](index.html) module"]
pub struct LTDC_BCCR_SPEC;
impl crate::RegisterSpec for LTDC_BCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_bccr::R](R) reader structure"]
impl crate::Readable for LTDC_BCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_bccr::W](W) writer structure"]
impl crate::Writable for LTDC_BCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_BCCR to value 0"]
impl crate::Resettable for LTDC_BCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
