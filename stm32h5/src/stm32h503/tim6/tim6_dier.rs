#[doc = "Register `TIM6_DIER` reader"]
pub struct R(crate::R<TIM6_DIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_DIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_DIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_DIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM6_DIER` writer"]
pub struct W(crate::W<TIM6_DIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_DIER_SPEC>;
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
impl From<crate::W<TIM6_DIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_DIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader<bool>;
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_DIER_SPEC, bool, O>;
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader<bool>;
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_DIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<0> {
        UIE_W::new(self)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UDE_W<8> {
        UDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM6 DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_dier](index.html) module"]
pub struct TIM6_DIER_SPEC;
impl crate::RegisterSpec for TIM6_DIER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim6_dier::R](R) reader structure"]
impl crate::Readable for TIM6_DIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim6_dier::W](W) writer structure"]
impl crate::Writable for TIM6_DIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM6_DIER to value 0"]
impl crate::Resettable for TIM6_DIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
