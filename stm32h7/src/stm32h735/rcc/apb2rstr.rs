#[doc = "Register `APB2RSTR` reader"]
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RSTR` writer"]
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 block reset"]
pub type TIM1RST_R = crate::BitReader<TIM1RST_A>;
#[doc = "TIM1 block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<TIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM1RST_A> {
        match self.bits {
            true => Some(TIM1RST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST_A::Reset
    }
}
#[doc = "Field `TIM1RST` writer - TIM1 block reset"]
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, TIM1RST_A, O>;
impl<'a, const O: u8> TIM1RST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::Reset)
    }
}
#[doc = "Field `TIM8RST` reader - TIM8 block reset"]
pub use TIM1RST_R as TIM8RST_R;
#[doc = "Field `USART1RST` reader - USART1 block reset"]
pub use TIM1RST_R as USART1RST_R;
#[doc = "Field `USART6RST` reader - USART6 block reset"]
pub use TIM1RST_R as USART6RST_R;
#[doc = "Field `UART9RST` reader - UART9 block reset"]
pub use TIM1RST_R as UART9RST_R;
#[doc = "Field `USART10RST` reader - USART10 block reset"]
pub use TIM1RST_R as USART10RST_R;
#[doc = "Field `SPI1RST` reader - SPI1 block reset"]
pub use TIM1RST_R as SPI1RST_R;
#[doc = "Field `SPI4RST` reader - SPI4 block reset"]
pub use TIM1RST_R as SPI4RST_R;
#[doc = "Field `TIM15RST` reader - TIM15 block reset"]
pub use TIM1RST_R as TIM15RST_R;
#[doc = "Field `TIM16RST` reader - TIM16 block reset"]
pub use TIM1RST_R as TIM16RST_R;
#[doc = "Field `TIM17RST` reader - TIM17 block reset"]
pub use TIM1RST_R as TIM17RST_R;
#[doc = "Field `SPI5RST` reader - SPI5 block reset"]
pub use TIM1RST_R as SPI5RST_R;
#[doc = "Field `SAI1RST` reader - SAI1 block reset"]
pub use TIM1RST_R as SAI1RST_R;
#[doc = "Field `SAI2RST` reader - SAI2 block reset"]
pub use TIM1RST_R as SAI2RST_R;
#[doc = "Field `SAI3RST` reader - SAI3 block reset"]
pub use TIM1RST_R as SAI3RST_R;
#[doc = "Field `DFSDM1RST` reader - DFSDM1 block reset"]
pub use TIM1RST_R as DFSDM1RST_R;
#[doc = "Field `TIM8RST` writer - TIM8 block reset"]
pub use TIM1RST_W as TIM8RST_W;
#[doc = "Field `USART1RST` writer - USART1 block reset"]
pub use TIM1RST_W as USART1RST_W;
#[doc = "Field `USART6RST` writer - USART6 block reset"]
pub use TIM1RST_W as USART6RST_W;
#[doc = "Field `UART9RST` writer - UART9 block reset"]
pub use TIM1RST_W as UART9RST_W;
#[doc = "Field `USART10RST` writer - USART10 block reset"]
pub use TIM1RST_W as USART10RST_W;
#[doc = "Field `SPI1RST` writer - SPI1 block reset"]
pub use TIM1RST_W as SPI1RST_W;
#[doc = "Field `SPI4RST` writer - SPI4 block reset"]
pub use TIM1RST_W as SPI4RST_W;
#[doc = "Field `TIM15RST` writer - TIM15 block reset"]
pub use TIM1RST_W as TIM15RST_W;
#[doc = "Field `TIM16RST` writer - TIM16 block reset"]
pub use TIM1RST_W as TIM16RST_W;
#[doc = "Field `TIM17RST` writer - TIM17 block reset"]
pub use TIM1RST_W as TIM17RST_W;
#[doc = "Field `SPI5RST` writer - SPI5 block reset"]
pub use TIM1RST_W as SPI5RST_W;
#[doc = "Field `SAI1RST` writer - SAI1 block reset"]
pub use TIM1RST_W as SAI1RST_W;
#[doc = "Field `SAI2RST` writer - SAI2 block reset"]
pub use TIM1RST_W as SAI2RST_W;
#[doc = "Field `SAI3RST` writer - SAI3 block reset"]
pub use TIM1RST_W as SAI3RST_W;
#[doc = "Field `DFSDM1RST` writer - DFSDM1 block reset"]
pub use TIM1RST_W as DFSDM1RST_W;
impl R {
    #[doc = "Bit 0 - TIM1 block reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 block reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 block reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 block reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART9 block reset"]
    #[inline(always)]
    pub fn uart9rst(&self) -> UART9RST_R {
        UART9RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART10 block reset"]
    #[inline(always)]
    pub fn usart10rst(&self) -> USART10RST_R {
        USART10RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 block reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 block reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 block reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 block reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 block reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 block reset"]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 block reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 block reset"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 block reset"]
    #[inline(always)]
    pub fn sai3rst(&self) -> SAI3RST_R {
        SAI3RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 block reset"]
    #[inline(always)]
    pub fn dfsdm1rst(&self) -> DFSDM1RST_R {
        DFSDM1RST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<0> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<1> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 4 - USART1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<4> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 5 - USART6 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<5> {
        USART6RST_W::new(self)
    }
    #[doc = "Bit 6 - UART9 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart9rst(&mut self) -> UART9RST_W<6> {
        UART9RST_W::new(self)
    }
    #[doc = "Bit 7 - USART10 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart10rst(&mut self) -> USART10RST_W<7> {
        USART10RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> SPI4RST_W<13> {
        SPI4RST_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 20 - SPI5 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi5rst(&mut self) -> SPI5RST_W<20> {
        SPI5RST_W::new(self)
    }
    #[doc = "Bit 22 - SAI1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<22> {
        SAI1RST_W::new(self)
    }
    #[doc = "Bit 23 - SAI2 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<23> {
        SAI2RST_W::new(self)
    }
    #[doc = "Bit 24 - SAI3 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai3rst(&mut self) -> SAI3RST_W<24> {
        SAI3RST_W::new(self)
    }
    #[doc = "Bit 28 - DFSDM1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1rst(&mut self) -> DFSDM1RST_W<28> {
        DFSDM1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB2 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rstr::R](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
