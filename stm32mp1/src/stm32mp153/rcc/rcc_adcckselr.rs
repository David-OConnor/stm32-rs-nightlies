#[doc = "Register `RCC_ADCCKSELR` reader"]
pub struct R(crate::R<RCC_ADCCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ADCCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ADCCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ADCCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_ADCCKSELR` writer"]
pub struct W(crate::W<RCC_ADCCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ADCCKSELR_SPEC>;
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
impl From<crate::W<RCC_ADCCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ADCCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSRC` reader - ADCSRC"]
pub type ADCSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCSRC` writer - ADCSRC"]
pub type ADCSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ADCCKSELR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    pub fn adcsrc(&self) -> ADCSRC_R {
        ADCSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    #[must_use]
    pub fn adcsrc(&mut self) -> ADCSRC_W<0> {
        ADCSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the ADC block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_adcckselr](index.html) module"]
pub struct RCC_ADCCKSELR_SPEC;
impl crate::RegisterSpec for RCC_ADCCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_adcckselr::R](R) reader structure"]
impl crate::Readable for RCC_ADCCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_adcckselr::W](W) writer structure"]
impl crate::Writable for RCC_ADCCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_ADCCKSELR to value 0"]
impl crate::Resettable for RCC_ADCCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
