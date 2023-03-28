#[doc = "Register `ADCPS2` reader"]
pub struct R(crate::R<ADCPS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCPS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCPS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCPS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCPS2` writer"]
pub struct W(crate::W<ADCPS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCPS2_SPEC>;
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
impl From<crate::W<ADCPS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCPS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC6PSC` reader - ADC6PSC"]
pub type ADC6PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC6PSC` writer - ADC6PSC"]
pub type ADC6PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCPS2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ADC7PSC` reader - ADC7PSC"]
pub type ADC7PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC7PSC` writer - ADC7PSC"]
pub type ADC7PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCPS2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ADC8PSC` reader - ADC8PSC"]
pub type ADC8PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC8PSC` writer - ADC8PSC"]
pub type ADC8PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCPS2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ADC9PSC` reader - ADC9PSC"]
pub type ADC9PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC9PSC` writer - ADC9PSC"]
pub type ADC9PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCPS2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ADC10PSC` reader - ADC10PSC"]
pub type ADC10PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC10PSC` writer - ADC10PSC"]
pub type ADC10PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCPS2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADC6PSC"]
    #[inline(always)]
    pub fn adc6psc(&self) -> ADC6PSC_R {
        ADC6PSC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC7PSC"]
    #[inline(always)]
    pub fn adc7psc(&self) -> ADC7PSC_R {
        ADC7PSC_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC8PSC"]
    #[inline(always)]
    pub fn adc8psc(&self) -> ADC8PSC_R {
        ADC8PSC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC9PSC"]
    #[inline(always)]
    pub fn adc9psc(&self) -> ADC9PSC_R {
        ADC9PSC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADC10PSC"]
    #[inline(always)]
    pub fn adc10psc(&self) -> ADC10PSC_R {
        ADC10PSC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC6PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc6psc(&mut self) -> ADC6PSC_W<0> {
        ADC6PSC_W::new(self)
    }
    #[doc = "Bits 6:10 - ADC7PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc7psc(&mut self) -> ADC7PSC_W<6> {
        ADC7PSC_W::new(self)
    }
    #[doc = "Bits 12:16 - ADC8PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc8psc(&mut self) -> ADC8PSC_W<12> {
        ADC8PSC_W::new(self)
    }
    #[doc = "Bits 18:22 - ADC9PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc9psc(&mut self) -> ADC9PSC_W<18> {
        ADC9PSC_W::new(self)
    }
    #[doc = "Bits 24:28 - ADC10PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc10psc(&mut self) -> ADC10PSC_W<24> {
        ADC10PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM ADC Post Scaler Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcps2](index.html) module"]
pub struct ADCPS2_SPEC;
impl crate::RegisterSpec for ADCPS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcps2::R](R) reader structure"]
impl crate::Readable for ADCPS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcps2::W](W) writer structure"]
impl crate::Writable for ADCPS2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCPS2 to value 0"]
impl crate::Resettable for ADCPS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
