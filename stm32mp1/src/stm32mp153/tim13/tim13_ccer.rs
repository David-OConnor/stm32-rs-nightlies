#[doc = "Register `TIM13_CCER` reader"]
pub struct R(crate::R<TIM13_CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM13_CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM13_CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM13_CCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM13_CCER` writer"]
pub struct W(crate::W<TIM13_CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM13_CCER_SPEC>;
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
impl From<crate::W<TIM13_CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM13_CCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1E` reader - CC1E"]
pub type CC1E_R = crate::BitReader<bool>;
#[doc = "Field `CC1E` writer - CC1E"]
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM13_CCER_SPEC, bool, O>;
#[doc = "Field `CC1P` reader - CC1P"]
pub type CC1P_R = crate::BitReader<bool>;
#[doc = "Field `CC1P` writer - CC1P"]
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM13_CCER_SPEC, bool, O>;
#[doc = "Field `CC1NP` reader - CC1NP"]
pub type CC1NP_R = crate::BitReader<bool>;
#[doc = "Field `CC1NP` writer - CC1NP"]
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM13_CCER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<0> {
        CC1E_W::new(self)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<1> {
        CC1P_W::new(self)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM13 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_ccer](index.html) module"]
pub struct TIM13_CCER_SPEC;
impl crate::RegisterSpec for TIM13_CCER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim13_ccer::R](R) reader structure"]
impl crate::Readable for TIM13_CCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim13_ccer::W](W) writer structure"]
impl crate::Writable for TIM13_CCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM13_CCER to value 0"]
impl crate::Resettable for TIM13_CCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
