#[doc = "Register `ADC2_OR` reader"]
pub struct R(crate::R<ADC2_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2_OR` writer"]
pub struct W(crate::W<ADC2_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_OR_SPEC>;
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
impl From<crate::W<ADC2_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDCOREEN` reader - VDDCOREEN"]
pub type VDDCOREEN_R = crate::BitReader<bool>;
#[doc = "Field `VDDCOREEN` writer - VDDCOREEN"]
pub type VDDCOREEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_OR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - VDDCOREEN"]
    #[inline(always)]
    pub fn vddcoreen(&self) -> VDDCOREEN_R {
        VDDCOREEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VDDCOREEN"]
    #[inline(always)]
    #[must_use]
    pub fn vddcoreen(&mut self) -> VDDCOREEN_W<0> {
        VDDCOREEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2_or](index.html) module"]
pub struct ADC2_OR_SPEC;
impl crate::RegisterSpec for ADC2_OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2_or::R](R) reader structure"]
impl crate::Readable for ADC2_OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2_or::W](W) writer structure"]
impl crate::Writable for ADC2_OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC2_OR to value 0"]
impl crate::Resettable for ADC2_OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
