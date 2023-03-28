#[doc = "Register `TIM3_PSC` reader"]
pub struct R(crate::R<TIM3_PSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM3_PSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM3_PSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM3_PSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM3_PSC` writer"]
pub struct W(crate::W<TIM3_PSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM3_PSC_SPEC>;
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
impl From<crate::W<TIM3_PSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM3_PSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` reader - PSC"]
pub type PSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSC` writer - PSC"]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM3_PSC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<0> {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM3 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_psc](index.html) module"]
pub struct TIM3_PSC_SPEC;
impl crate::RegisterSpec for TIM3_PSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim3_psc::R](R) reader structure"]
impl crate::Readable for TIM3_PSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim3_psc::W](W) writer structure"]
impl crate::Writable for TIM3_PSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM3_PSC to value 0"]
impl crate::Resettable for TIM3_PSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
