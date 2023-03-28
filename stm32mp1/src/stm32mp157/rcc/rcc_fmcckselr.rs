#[doc = "Register `RCC_FMCCKSELR` reader"]
pub struct R(crate::R<RCC_FMCCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_FMCCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_FMCCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_FMCCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_FMCCKSELR` writer"]
pub struct W(crate::W<RCC_FMCCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_FMCCKSELR_SPEC>;
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
impl From<crate::W<RCC_FMCCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_FMCCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCSRC` reader - FMCSRC"]
pub type FMCSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMCSRC` writer - FMCSRC"]
pub type FMCSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_FMCCKSELR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - FMCSRC"]
    #[inline(always)]
    pub fn fmcsrc(&self) -> FMCSRC_R {
        FMCSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMCSRC"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsrc(&mut self) -> FMCSRC_W<0> {
        FMCSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_fmcckselr](index.html) module"]
pub struct RCC_FMCCKSELR_SPEC;
impl crate::RegisterSpec for RCC_FMCCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_fmcckselr::R](R) reader structure"]
impl crate::Readable for RCC_FMCCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_fmcckselr::W](W) writer structure"]
impl crate::Writable for RCC_FMCCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_FMCCKSELR to value 0"]
impl crate::Resettable for RCC_FMCCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
