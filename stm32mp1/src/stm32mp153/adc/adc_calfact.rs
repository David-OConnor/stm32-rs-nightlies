#[doc = "Register `ADC_CALFACT` reader"]
pub struct R(crate::R<ADC_CALFACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CALFACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CALFACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CALFACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CALFACT` writer"]
pub struct W(crate::W<ADC_CALFACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CALFACT_SPEC>;
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
impl From<crate::W<ADC_CALFACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CALFACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALFACT_S` reader - CALFACT_S"]
pub type CALFACT_S_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALFACT_S` writer - CALFACT_S"]
pub type CALFACT_S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_CALFACT_SPEC, u16, u16, 11, O>;
#[doc = "Field `CALFACT_D` reader - CALFACT_D"]
pub type CALFACT_D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALFACT_D` writer - CALFACT_D"]
pub type CALFACT_D_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_CALFACT_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - CALFACT_S"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - CALFACT_D"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CALFACT_S"]
    #[inline(always)]
    #[must_use]
    pub fn calfact_s(&mut self) -> CALFACT_S_W<0> {
        CALFACT_S_W::new(self)
    }
    #[doc = "Bits 16:26 - CALFACT_D"]
    #[inline(always)]
    #[must_use]
    pub fn calfact_d(&mut self) -> CALFACT_D_W<16> {
        CALFACT_D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration factors register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_calfact](index.html) module"]
pub struct ADC_CALFACT_SPEC;
impl crate::RegisterSpec for ADC_CALFACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_calfact::R](R) reader structure"]
impl crate::Readable for ADC_CALFACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_calfact::W](W) writer structure"]
impl crate::Writable for ADC_CALFACT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_CALFACT to value 0"]
impl crate::Resettable for ADC_CALFACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
