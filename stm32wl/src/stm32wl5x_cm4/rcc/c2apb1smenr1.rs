#[doc = "Register `C2APB1SMENR1` reader"]
pub struct R(crate::R<C2APB1SMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB1SMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB1SMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB1SMENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB1SMENR1` writer"]
pub struct W(crate::W<C2APB1SMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB1SMENR1_SPEC>;
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
impl From<crate::W<C2APB1SMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB1SMENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clock enable during CPU2 CSleep mode."]
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN_A>;
#[doc = "TIM2 timer clock enable during CPU2 CSleep mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<TIM2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2SMEN_A {
        match self.bits {
            false => TIM2SMEN_A::Disabled,
            true => TIM2SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN_A::Enabled
    }
}
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clock enable during CPU2 CSleep mode."]
pub type TIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB1SMENR1_SPEC, TIM2SMEN_A, O>;
impl<'a, const O: u8> TIM2SMEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::Enabled)
    }
}
#[doc = "Field `RTCAPBSMEN` reader - RTC bus clock enable during CPU2 CSleep mode."]
pub use TIM2SMEN_R as RTCAPBSMEN_R;
#[doc = "Field `SPI2S2SMEN` reader - SPI2S2 clock enable during CPU2 CSleep mode."]
pub use TIM2SMEN_R as SPI2S2SMEN_R;
#[doc = "Field `USART2SMEN` reader - USART2 clock enable during CPU2 CSleep mode."]
pub use TIM2SMEN_R as USART2SMEN_R;
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during CPU2 CSleep and CStop modes"]
pub use TIM2SMEN_R as I2C1SMEN_R;
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during CPU2 CSleep and CStop modes"]
pub use TIM2SMEN_R as I2C2SMEN_R;
#[doc = "Field `I2C3SMEN` reader - I2C3 clock enable during CPU2 CSleep and CStop modes"]
pub use TIM2SMEN_R as I2C3SMEN_R;
#[doc = "Field `DAC1SMEN` reader - DAC1 clock enable during CPU2 CSleep mode."]
pub use TIM2SMEN_R as DAC1SMEN_R;
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clock enable during CPU2 CSleep and CStop mode"]
pub use TIM2SMEN_R as LPTIM1SMEN_R;
#[doc = "Field `RTCAPBSMEN` writer - RTC bus clock enable during CPU2 CSleep mode."]
pub use TIM2SMEN_W as RTCAPBSMEN_W;
#[doc = "Field `SPI2S2SMEN` writer - SPI2S2 clock enable during CPU2 CSleep mode."]
pub use TIM2SMEN_W as SPI2S2SMEN_W;
#[doc = "Field `USART2SMEN` writer - USART2 clock enable during CPU2 CSleep mode."]
pub use TIM2SMEN_W as USART2SMEN_W;
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during CPU2 CSleep and CStop modes"]
pub use TIM2SMEN_W as I2C1SMEN_W;
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during CPU2 CSleep and CStop modes"]
pub use TIM2SMEN_W as I2C2SMEN_W;
#[doc = "Field `I2C3SMEN` writer - I2C3 clock enable during CPU2 CSleep and CStop modes"]
pub use TIM2SMEN_W as I2C3SMEN_W;
#[doc = "Field `DAC1SMEN` writer - DAC1 clock enable during CPU2 CSleep mode."]
pub use TIM2SMEN_W as DAC1SMEN_W;
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clock enable during CPU2 CSleep and CStop mode"]
pub use TIM2SMEN_W as LPTIM1SMEN_W;
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - RTC bus clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2S2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn spi2s2smen(&self) -> SPI2S2SMEN_R {
        SPI2S2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during CPU2 CSleep and CStop modes"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during CPU2 CSleep and CStop modes"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during CPU2 CSleep and CStop modes"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable during CPU2 CSleep and CStop mode"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<0> {
        TIM2SMEN_W::new(self)
    }
    #[doc = "Bit 10 - RTC bus clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<10> {
        RTCAPBSMEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2S2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi2s2smen(&mut self) -> SPI2S2SMEN_W<14> {
        SPI2S2SMEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<17> {
        USART2SMEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable during CPU2 CSleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable during CPU2 CSleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<22> {
        I2C2SMEN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clock enable during CPU2 CSleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<23> {
        I2C3SMEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<29> {
        DAC1SMEN_W::new(self)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable during CPU2 CSleep and CStop mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<31> {
        LPTIM1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1smenr1](index.html) module"]
pub struct C2APB1SMENR1_SPEC;
impl crate::RegisterSpec for C2APB1SMENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb1smenr1::R](R) reader structure"]
impl crate::Readable for C2APB1SMENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb1smenr1::W](W) writer structure"]
impl crate::Writable for C2APB1SMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2APB1SMENR1 to value 0xa0e2_4401"]
impl crate::Resettable for C2APB1SMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xa0e2_4401;
}
