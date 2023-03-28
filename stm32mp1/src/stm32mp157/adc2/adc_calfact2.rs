#[doc = "Register `ADC_CALFACT2` reader"]
pub struct R(crate::R<ADC_CALFACT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CALFACT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CALFACT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CALFACT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CALFACT2` writer"]
pub struct W(crate::W<ADC_CALFACT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CALFACT2_SPEC>;
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
impl From<crate::W<ADC_CALFACT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CALFACT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINCALFACT` reader - LINCALFACT"]
pub type LINCALFACT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LINCALFACT` writer - LINCALFACT"]
pub type LINCALFACT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_CALFACT2_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - LINCALFACT"]
    #[inline(always)]
    pub fn lincalfact(&self) -> LINCALFACT_R {
        LINCALFACT_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - LINCALFACT"]
    #[inline(always)]
    #[must_use]
    pub fn lincalfact(&mut self) -> LINCALFACT_W<0> {
        LINCALFACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration factor register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_calfact2](index.html) module"]
pub struct ADC_CALFACT2_SPEC;
impl crate::RegisterSpec for ADC_CALFACT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_calfact2::R](R) reader structure"]
impl crate::Readable for ADC_CALFACT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_calfact2::W](W) writer structure"]
impl crate::Writable for ADC_CALFACT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_CALFACT2 to value 0"]
impl crate::Resettable for ADC_CALFACT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
