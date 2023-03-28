#[doc = "Register `APB2FZR` reader"]
pub struct R(crate::R<APB2FZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2FZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2FZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2FZR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2FZR` writer"]
pub struct W(crate::W<APB2FZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2FZR_SPEC>;
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
impl From<crate::W<APB2FZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2FZR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 stop in debug"]
pub type DBG_TIM1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 stop in debug"]
pub type DBG_TIM1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2FZR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<11> {
        DBG_TIM1_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB2 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fzr](index.html) module"]
pub struct APB2FZR_SPEC;
impl crate::RegisterSpec for APB2FZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2fzr::R](R) reader structure"]
impl crate::Readable for APB2FZR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2fzr::W](W) writer structure"]
impl crate::Writable for APB2FZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2FZR to value 0"]
impl crate::Resettable for APB2FZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
