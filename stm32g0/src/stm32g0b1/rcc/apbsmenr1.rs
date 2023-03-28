#[doc = "Register `APBSMENR1` reader"]
pub struct R(crate::R<APBSMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBSMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBSMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBSMENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBSMENR1` writer"]
pub struct W(crate::W<APBSMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBSMENR1_SPEC>;
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
impl From<crate::W<APBSMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBSMENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clock enable during Sleep mode"]
pub type TIM2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clock enable during Sleep mode"]
pub type TIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode"]
pub type TIM3SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode"]
pub type TIM3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clock enable during Sleep mode"]
pub type TIM4SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clock enable during Sleep mode"]
pub type TIM4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode"]
pub type TIM6SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode"]
pub type TIM6SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode"]
pub type TIM7SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode"]
pub type TIM7SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `LPUART2SMEN` reader - LPUART2 clock enable"]
pub type LPUART2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART2SMEN` writer - LPUART2 clock enable"]
pub type LPUART2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `USART5SMEN` reader - USART5 clock enable"]
pub type USART5SMEN_R = crate::BitReader<bool>;
#[doc = "Field `USART5SMEN` writer - USART5 clock enable"]
pub type USART5SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `USART6SMEN` reader - USART6 clock enable"]
pub type USART6SMEN_R = crate::BitReader<bool>;
#[doc = "Field `USART6SMEN` writer - USART6 clock enable"]
pub type USART6SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode"]
pub type RTCAPBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode"]
pub type RTCAPBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode"]
pub type WWDGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode"]
pub type WWDGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `FDCANSMEN` reader - FDCAN clock enable during Sleep mode"]
pub type FDCANSMEN_R = crate::BitReader<bool>;
#[doc = "Field `FDCANSMEN` writer - FDCAN clock enable during Sleep mode"]
pub type FDCANSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `USBSMEN` reader - USB clock enable during Sleep mode"]
pub type USBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `USBSMEN` writer - USB clock enable during Sleep mode"]
pub type USBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode"]
pub type SPI2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode"]
pub type SPI2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `SPI3SMEN` reader - SPI3 clock enable during Sleep mode"]
pub type SPI3SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3SMEN` writer - SPI3 clock enable during Sleep mode"]
pub type SPI3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `CRSSSMEN` reader - CRSS clock enable during Sleep mode"]
pub type CRSSSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CRSSSMEN` writer - CRSS clock enable during Sleep mode"]
pub type CRSSSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `USART2SMEN` reader - USART2 clock enable during Sleep mode"]
pub type USART2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `USART2SMEN` writer - USART2 clock enable during Sleep mode"]
pub type USART2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `USART3SMEN` reader - USART3 clock enable during Sleep mode"]
pub type USART3SMEN_R = crate::BitReader<bool>;
#[doc = "Field `USART3SMEN` writer - USART3 clock enable during Sleep mode"]
pub type USART3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `USART4SMEN` reader - USART4 clock enable during Sleep mode"]
pub type USART4SMEN_R = crate::BitReader<bool>;
#[doc = "Field `USART4SMEN` writer - USART4 clock enable during Sleep mode"]
pub type USART4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `LPUART1SMEN` reader - LPUART1 clock enable during Sleep mode"]
pub type LPUART1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1SMEN` writer - LPUART1 clock enable during Sleep mode"]
pub type LPUART1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode"]
pub type I2C1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode"]
pub type I2C1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode"]
pub type I2C2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode"]
pub type I2C2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `I2C3SMEN` reader - I2C3 clock enable during Sleep mode"]
pub type I2C3SMEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3SMEN` writer - I2C3 clock enable during Sleep mode"]
pub type I2C3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `CECSMEN` reader - HDMI CEC clock enable during Sleep mode"]
pub type CECSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CECSMEN` writer - HDMI CEC clock enable during Sleep mode"]
pub type CECSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `UCPD1SMEN` reader - UCPD1 clock enable during Sleep mode"]
pub type UCPD1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `UCPD1SMEN` writer - UCPD1 clock enable during Sleep mode"]
pub type UCPD1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `UCPD2SMEN` reader - UCPD2 clock enable during Sleep mode"]
pub type UCPD2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `UCPD2SMEN` writer - UCPD2 clock enable during Sleep mode"]
pub type UCPD2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `DBGSMEN` reader - Debug support clock enable during Sleep mode"]
pub type DBGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `DBGSMEN` writer - Debug support clock enable during Sleep mode"]
pub type DBGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `PWRSMEN` reader - Power interface clock enable during Sleep mode"]
pub type PWRSMEN_R = crate::BitReader<bool>;
#[doc = "Field `PWRSMEN` writer - Power interface clock enable during Sleep mode"]
pub type PWRSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `DAC1SMEN` reader - DAC1 interface clock enable during Sleep mode"]
pub type DAC1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC1SMEN` writer - DAC1 interface clock enable during Sleep mode"]
pub type DAC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `LPTIM2SMEN` reader - Low Power Timer 2 clock enable during Sleep mode"]
pub type LPTIM2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2SMEN` writer - Low Power Timer 2 clock enable during Sleep mode"]
pub type LPTIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
#[doc = "Field `LPTIM1SMEN` reader - Low Power Timer 1 clock enable during Sleep mode"]
pub type LPTIM1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1SMEN` writer - Low Power Timer 1 clock enable during Sleep mode"]
pub type LPTIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPUART2 clock enable"]
    #[inline(always)]
    pub fn lpuart2smen(&self) -> LPUART2SMEN_R {
        LPUART2SMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5smen(&self) -> USART5SMEN_R {
        USART5SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6smen(&self) -> USART6SMEN_R {
        USART6SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FDCAN clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fdcansmen(&self) -> FDCANSMEN_R {
        FDCANSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CRSS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crsssmen(&self) -> CRSSSMEN_R {
        CRSSSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - HDMI CEC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn cecsmen(&self) -> CECSMEN_R {
        CECSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UCPD1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - UCPD2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ucpd2smen(&self) -> UCPD2SMEN_R {
        UCPD2SMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Power Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<0> {
        TIM2SMEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<1> {
        TIM3SMEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<2> {
        TIM4SMEN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<4> {
        TIM6SMEN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<5> {
        TIM7SMEN_W::new(self)
    }
    #[doc = "Bit 7 - LPUART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2smen(&mut self) -> LPUART2SMEN_W<7> {
        LPUART2SMEN_W::new(self)
    }
    #[doc = "Bit 8 - USART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart5smen(&mut self) -> USART5SMEN_W<8> {
        USART5SMEN_W::new(self)
    }
    #[doc = "Bit 9 - USART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6smen(&mut self) -> USART6SMEN_W<9> {
        USART6SMEN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<10> {
        RTCAPBSMEN_W::new(self)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<11> {
        WWDGSMEN_W::new(self)
    }
    #[doc = "Bit 12 - FDCAN clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansmen(&mut self) -> FDCANSMEN_W<12> {
        FDCANSMEN_W::new(self)
    }
    #[doc = "Bit 13 - USB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbsmen(&mut self) -> USBSMEN_W<13> {
        USBSMEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<14> {
        SPI2SMEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<15> {
        SPI3SMEN_W::new(self)
    }
    #[doc = "Bit 16 - CRSS clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crsssmen(&mut self) -> CRSSSMEN_W<16> {
        CRSSSMEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<17> {
        USART2SMEN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<18> {
        USART3SMEN_W::new(self)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart4smen(&mut self) -> USART4SMEN_W<19> {
        USART4SMEN_W::new(self)
    }
    #[doc = "Bit 20 - LPUART1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<20> {
        LPUART1SMEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<22> {
        I2C2SMEN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<23> {
        I2C3SMEN_W::new(self)
    }
    #[doc = "Bit 24 - HDMI CEC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn cecsmen(&mut self) -> CECSMEN_W<24> {
        CECSMEN_W::new(self)
    }
    #[doc = "Bit 25 - UCPD1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<25> {
        UCPD1SMEN_W::new(self)
    }
    #[doc = "Bit 26 - UCPD2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd2smen(&mut self) -> UCPD2SMEN_W<26> {
        UCPD2SMEN_W::new(self)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<27> {
        DBGSMEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<28> {
        PWRSMEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<29> {
        DAC1SMEN_W::new(self)
    }
    #[doc = "Bit 30 - Low Power Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<30> {
        LPTIM2SMEN_W::new(self)
    }
    #[doc = "Bit 31 - Low Power Timer 1 clock enable during Sleep mode"]
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
#[doc = "APB peripheral clock enable in Sleep mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbsmenr1](index.html) module"]
pub struct APBSMENR1_SPEC;
impl crate::RegisterSpec for APBSMENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbsmenr1::R](R) reader structure"]
impl crate::Readable for APBSMENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbsmenr1::W](W) writer structure"]
impl crate::Writable for APBSMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBSMENR1 to value 0xffff_ffb7"]
impl crate::Resettable for APBSMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffb7;
}
