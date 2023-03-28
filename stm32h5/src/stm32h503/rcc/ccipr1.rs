#[doc = "Register `CCIPR1` reader"]
pub struct R(crate::R<CCIPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR1` writer"]
pub struct W(crate::W<CCIPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR1_SPEC>;
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
impl From<crate::W<CCIPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART1SEL` reader - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART1SEL` writer - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `USART2SEL` reader - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART2SEL` writer - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `USART3SEL` reader - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART3SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART3SEL` writer - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIMICSEL` reader - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
pub type TIMICSEL_R = crate::BitReader<bool>;
#[doc = "Field `TIMICSEL` writer - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
pub type TIMICSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
    #[inline(always)]
    pub fn timicsel(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    #[doc = "Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<3> {
        USART2SEL_W::new(self)
    }
    #[doc = "Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<6> {
        USART3SEL_W::new(self)
    }
    #[doc = "Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn timicsel(&mut self) -> TIMICSEL_W<31> {
        TIMICSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr1](index.html) module"]
pub struct CCIPR1_SPEC;
impl crate::RegisterSpec for CCIPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr1::R](R) reader structure"]
impl crate::Readable for CCIPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr1::W](W) writer structure"]
impl crate::Writable for CCIPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR1 to value 0"]
impl crate::Resettable for CCIPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
