#[doc = "Register `RCC_CPERCKSELR` reader"]
pub struct R(crate::R<RCC_CPERCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CPERCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CPERCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CPERCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CPERCKSELR` writer"]
pub struct W(crate::W<RCC_CPERCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CPERCKSELR_SPEC>;
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
impl From<crate::W<RCC_CPERCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CPERCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKPERSRC` reader - CKPERSRC"]
pub type CKPERSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKPERSRC` writer - CKPERSRC"]
pub type CKPERSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_CPERCKSELR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    pub fn ckpersrc(&self) -> CKPERSRC_R {
        CKPERSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ckpersrc(&mut self) -> CKPERSRC_W<0> {
        CKPERSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cperckselr](index.html) module"]
pub struct RCC_CPERCKSELR_SPEC;
impl crate::RegisterSpec for RCC_CPERCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cperckselr::R](R) reader structure"]
impl crate::Readable for RCC_CPERCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cperckselr::W](W) writer structure"]
impl crate::Writable for RCC_CPERCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CPERCKSELR to value 0"]
impl crate::Resettable for RCC_CPERCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
