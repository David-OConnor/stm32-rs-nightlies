#[doc = "Register `ADC_AWD2CR` reader"]
pub struct R(crate::R<ADC_AWD2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_AWD2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_AWD2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_AWD2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_AWD2CR` writer"]
pub struct W(crate::W<ADC_AWD2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_AWD2CR_SPEC>;
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
impl From<crate::W<ADC_AWD2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_AWD2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWD2CH` reader - AWD2CH"]
pub type AWD2CH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AWD2CH` writer - AWD2CH"]
pub type AWD2CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_AWD2CR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch(&self) -> AWD2CH_R {
        AWD2CH_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch(&mut self) -> AWD2CH_W<0> {
        AWD2CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC analog watchdog 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_awd2cr](index.html) module"]
pub struct ADC_AWD2CR_SPEC;
impl crate::RegisterSpec for ADC_AWD2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_awd2cr::R](R) reader structure"]
impl crate::Readable for ADC_AWD2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_awd2cr::W](W) writer structure"]
impl crate::Writable for ADC_AWD2CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_AWD2CR to value 0"]
impl crate::Resettable for ADC_AWD2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
