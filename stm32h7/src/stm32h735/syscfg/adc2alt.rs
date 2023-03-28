#[doc = "Register `ADC2ALT` reader"]
pub struct R(crate::R<ADC2ALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2ALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2ALT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2ALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2ALT` writer"]
pub struct W(crate::W<ADC2ALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2ALT_SPEC>;
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
impl From<crate::W<ADC2ALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2ALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC2_ROUT0` reader - ADC2 V_INP16 alternate connection"]
pub type ADC2_ROUT0_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ROUT0` writer - ADC2 V_INP16 alternate connection"]
pub type ADC2_ROUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2ALT_SPEC, bool, O>;
#[doc = "Field `ADC2_ROUT1` reader - ADC2 V_INP17 alternate connection"]
pub type ADC2_ROUT1_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ROUT1` writer - ADC2 V_INP17 alternate connection"]
pub type ADC2_ROUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2ALT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC2 V_INP16 alternate connection"]
    #[inline(always)]
    pub fn adc2_rout0(&self) -> ADC2_ROUT0_R {
        ADC2_ROUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC2 V_INP17 alternate connection"]
    #[inline(always)]
    pub fn adc2_rout1(&self) -> ADC2_ROUT1_R {
        ADC2_ROUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC2 V_INP16 alternate connection"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_rout0(&mut self) -> ADC2_ROUT0_W<0> {
        ADC2_ROUT0_W::new(self)
    }
    #[doc = "Bit 1 - ADC2 V_INP17 alternate connection"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_rout1(&mut self) -> ADC2_ROUT1_W<1> {
        ADC2_ROUT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 internal input alternate connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2alt](index.html) module"]
pub struct ADC2ALT_SPEC;
impl crate::RegisterSpec for ADC2ALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2alt::R](R) reader structure"]
impl crate::Readable for ADC2ALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2alt::W](W) writer structure"]
impl crate::Writable for ADC2ALT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC2ALT to value 0"]
impl crate::Resettable for ADC2ALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
