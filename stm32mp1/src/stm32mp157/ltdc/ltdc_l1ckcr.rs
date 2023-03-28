#[doc = "Register `LTDC_L1CKCR` reader"]
pub struct R(crate::R<LTDC_L1CKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L1CKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L1CKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L1CKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L1CKCR` writer"]
pub struct W(crate::W<LTDC_L1CKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L1CKCR_SPEC>;
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
impl From<crate::W<LTDC_L1CKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L1CKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKBLUE` reader - CKBLUE"]
pub type CKBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKBLUE` writer - CKBLUE"]
pub type CKBLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L1CKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKGREEN` reader - CKGREEN"]
pub type CKGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKGREEN` writer - CKGREEN"]
pub type CKGREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L1CKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKRED` reader - CKRED"]
pub type CKRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKRED` writer - CKRED"]
pub type CKRED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L1CKCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CKBLUE"]
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CKGREEN"]
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CKRED"]
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CKBLUE"]
    #[inline(always)]
    #[must_use]
    pub fn ckblue(&mut self) -> CKBLUE_W<0> {
        CKBLUE_W::new(self)
    }
    #[doc = "Bits 8:15 - CKGREEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckgreen(&mut self) -> CKGREEN_W<8> {
        CKGREEN_W::new(self)
    }
    #[doc = "Bits 16:23 - CKRED"]
    #[inline(always)]
    #[must_use]
    pub fn ckred(&mut self) -> CKRED_W<16> {
        CKRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the color key value (RGB), that is used by the color keying.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1ckcr](index.html) module"]
pub struct LTDC_L1CKCR_SPEC;
impl crate::RegisterSpec for LTDC_L1CKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l1ckcr::R](R) reader structure"]
impl crate::Readable for LTDC_L1CKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l1ckcr::W](W) writer structure"]
impl crate::Writable for LTDC_L1CKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_L1CKCR to value 0"]
impl crate::Resettable for LTDC_L1CKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
