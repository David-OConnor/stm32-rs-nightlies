#[doc = "Register `TIM15_CCER` reader"]
pub struct R(crate::R<TIM15_CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_CCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM15_CCER` writer"]
pub struct W(crate::W<TIM15_CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_CCER_SPEC>;
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
impl From<crate::W<TIM15_CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_CCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1E` reader - CC1E"]
pub type CC1E_R = crate::BitReader<bool>;
#[doc = "Field `CC1E` writer - CC1E"]
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CCER_SPEC, bool, O>;
#[doc = "Field `CC1P` reader - CC1P"]
pub type CC1P_R = crate::BitReader<bool>;
#[doc = "Field `CC1P` writer - CC1P"]
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CCER_SPEC, bool, O>;
#[doc = "Field `CC1NE` reader - CC1NE"]
pub type CC1NE_R = crate::BitReader<bool>;
#[doc = "Field `CC1NE` writer - CC1NE"]
pub type CC1NE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CCER_SPEC, bool, O>;
#[doc = "Field `CC1NP` reader - CC1NP"]
pub type CC1NP_R = crate::BitReader<bool>;
#[doc = "Field `CC1NP` writer - CC1NP"]
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CCER_SPEC, bool, O>;
#[doc = "Field `CC2E` reader - CC2E"]
pub type CC2E_R = crate::BitReader<bool>;
#[doc = "Field `CC2E` writer - CC2E"]
pub type CC2E_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CCER_SPEC, bool, O>;
#[doc = "Field `CC2P` reader - CC2P"]
pub type CC2P_R = crate::BitReader<bool>;
#[doc = "Field `CC2P` writer - CC2P"]
pub type CC2P_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CCER_SPEC, bool, O>;
#[doc = "Field `CC2NP` reader - CC2NP"]
pub type CC2NP_R = crate::BitReader<bool>;
#[doc = "Field `CC2NP` writer - CC2NP"]
pub type CC2NP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CCER_SPEC, bool, O>;
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
    #[doc = "Bit 2 - CC1NE"]
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC2E"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 2 - CC1NE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<2> {
        CC1NE_W::new(self)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    #[doc = "Bit 4 - CC2E"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<4> {
        CC2E_W::new(self)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<5> {
        CC2P_W::new(self)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<7> {
        CC2NP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_ccer](index.html) module"]
pub struct TIM15_CCER_SPEC;
impl crate::RegisterSpec for TIM15_CCER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim15_ccer::R](R) reader structure"]
impl crate::Readable for TIM15_CCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim15_ccer::W](W) writer structure"]
impl crate::Writable for TIM15_CCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM15_CCER to value 0"]
impl crate::Resettable for TIM15_CCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
