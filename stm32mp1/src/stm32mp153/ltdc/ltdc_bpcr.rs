#[doc = "Register `LTDC_BPCR` reader"]
pub struct R(crate::R<LTDC_BPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_BPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_BPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_BPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_BPCR` writer"]
pub struct W(crate::W<LTDC_BPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_BPCR_SPEC>;
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
impl From<crate::W<LTDC_BPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_BPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVBP` reader - AVBP"]
pub type AVBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AVBP` writer - AVBP"]
pub type AVBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_BPCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `AHBP` reader - AHBP"]
pub type AHBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AHBP` writer - AHBP"]
pub type AHBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_BPCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - AVBP"]
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AVBP"]
    #[inline(always)]
    #[must_use]
    pub fn avbp(&mut self) -> AVBP_W<0> {
        AVBP_W::new(self)
    }
    #[doc = "Bits 16:27 - AHBP"]
    #[inline(always)]
    #[must_use]
    pub fn ahbp(&mut self) -> AHBP_W<16> {
        AHBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_bpcr](index.html) module"]
pub struct LTDC_BPCR_SPEC;
impl crate::RegisterSpec for LTDC_BPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_bpcr::R](R) reader structure"]
impl crate::Readable for LTDC_BPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_bpcr::W](W) writer structure"]
impl crate::Writable for LTDC_BPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_BPCR to value 0"]
impl crate::Resettable for LTDC_BPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
