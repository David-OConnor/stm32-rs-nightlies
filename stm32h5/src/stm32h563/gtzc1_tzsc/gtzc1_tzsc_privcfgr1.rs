#[doc = "Register `GTZC1_TZSC_PRIVCFGR1` reader"]
pub struct R(crate::R<GTZC1_TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZSC_PRIVCFGR1` writer"]
pub struct W(crate::W<GTZC1_TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_PRIVCFGR1_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2PRIV` reader - privileged access mode for TIM2"]
pub type TIM2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM2PRIV` writer - privileged access mode for TIM2"]
pub type TIM2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM3PRIV` reader - privileged access mode for TIM3"]
pub type TIM3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM3PRIV` writer - privileged access mode for TIM3"]
pub type TIM3PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM4PRIV` reader - privileged access mode for TIM4"]
pub type TIM4PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM4PRIV` writer - privileged access mode for TIM4"]
pub type TIM4PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM5PRIV` reader - privileged access mode for TIM5"]
pub type TIM5PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM5PRIV` writer - privileged access mode for TIM5"]
pub type TIM5PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM6PRIV` reader - privileged access mode for TIM6"]
pub type TIM6PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM6PRIV` writer - privileged access mode for TIM6"]
pub type TIM6PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM7PRIV` reader - privileged access mode for TIM7"]
pub type TIM7PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM7PRIV` writer - privileged access mode for TIM7"]
pub type TIM7PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM12PRIV` reader - privileged access mode for TIM12"]
pub type TIM12PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM12PRIV` writer - privileged access mode for TIM12"]
pub type TIM12PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM13PRIV` reader - privileged access mode for TIM13"]
pub type TIM13PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM13PRIV` writer - privileged access mode for TIM13"]
pub type TIM13PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM14PRIV` reader - privileged access mode for TIM14"]
pub type TIM14PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM14PRIV` writer - privileged access mode for TIM14"]
pub type TIM14PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `WWDGPRIV` reader - privileged access mode for WWDG"]
pub type WWDGPRIV_R = crate::BitReader<bool>;
#[doc = "Field `WWDGPRIV` writer - privileged access mode for WWDG"]
pub type WWDGPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `IWDGPRIV` reader - privileged access mode for IWDG"]
pub type IWDGPRIV_R = crate::BitReader<bool>;
#[doc = "Field `IWDGPRIV` writer - privileged access mode for IWDG"]
pub type IWDGPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI2PRIV` reader - privileged access mode for SPI2"]
pub type SPI2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI2PRIV` writer - privileged access mode for SPI2"]
pub type SPI2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI3PRIV` reader - privileged access mode for SPI3"]
pub type SPI3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI3PRIV` writer - privileged access mode for SPI3"]
pub type SPI3PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `USART2PRIV` reader - privileged access mode for USART2"]
pub type USART2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART2PRIV` writer - privileged access mode for USART2"]
pub type USART2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `USART3PRIV` reader - privileged access mode for USART3"]
pub type USART3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART3PRIV` writer - privileged access mode for USART3"]
pub type USART3PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UART4PRIV` reader - privileged access mode for UART4"]
pub type UART4PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UART4PRIV` writer - privileged access mode for UART4"]
pub type UART4PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UART5PRIV` reader - privileged access mode for UART5"]
pub type UART5PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UART5PRIV` writer - privileged access mode for UART5"]
pub type UART5PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C1PRIV` reader - privileged access mode for I2C1"]
pub type I2C1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I2C1PRIV` writer - privileged access mode for I2C1"]
pub type I2C1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C2PRIV` reader - privileged access mode for I2C2"]
pub type I2C2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I2C2PRIV` writer - privileged access mode for I2C2"]
pub type I2C2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `I3C1PRIV` reader - privileged access mode for I3C1"]
pub type I3C1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I3C1PRIV` writer - privileged access mode for I3C1"]
pub type I3C1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `CRSPRIV` reader - privileged access mode for CRS"]
pub type CRSPRIV_R = crate::BitReader<bool>;
#[doc = "Field `CRSPRIV` writer - privileged access mode for CRS"]
pub type CRSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `USART6PRIV` reader - privileged access mode for USART6"]
pub type USART6PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART6PRIV` writer - privileged access mode for USART6"]
pub type USART6PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `USART10PRIV` reader - privileged access mode for USART10"]
pub type USART10PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART10PRIV` writer - privileged access mode for USART10"]
pub type USART10PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `USART11PRIV` reader - privileged access mode for USART11"]
pub type USART11PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART11PRIV` writer - privileged access mode for USART11"]
pub type USART11PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `HDMICECPRIV` reader - privileged access mode for HDMICEC"]
pub type HDMICECPRIV_R = crate::BitReader<bool>;
#[doc = "Field `HDMICECPRIV` writer - privileged access mode for HDMICEC"]
pub type HDMICECPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `DAC1PRIV` reader - privileged access mode for DAC1"]
pub type DAC1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `DAC1PRIV` writer - privileged access mode for DAC1"]
pub type DAC1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UART7PRIV` reader - privileged access mode for UART7"]
pub type UART7PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UART7PRIV` writer - privileged access mode for UART7"]
pub type UART7PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UART8PRIV` reader - privileged access mode for UART8"]
pub type UART8PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UART8PRIV` writer - privileged access mode for UART8"]
pub type UART8PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UART9PRIV` reader - privileged access mode for UART9"]
pub type UART9PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UART9PRIV` writer - privileged access mode for UART9"]
pub type UART9PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UART12PRIV` reader - privileged access mode for UART12"]
pub type UART12PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UART12PRIV` writer - privileged access mode for UART12"]
pub type UART12PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `DTSPRIV` reader - privileged access mode for DTS"]
pub type DTSPRIV_R = crate::BitReader<bool>;
#[doc = "Field `DTSPRIV` writer - privileged access mode for DTS"]
pub type DTSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `LPTIM2PRIV` reader - privileged access mode for LPTIM2"]
pub type LPTIM2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2PRIV` writer - privileged access mode for LPTIM2"]
pub type LPTIM2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - privileged access mode for TIM2"]
    #[inline(always)]
    pub fn tim2priv(&self) -> TIM2PRIV_R {
        TIM2PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for TIM3"]
    #[inline(always)]
    pub fn tim3priv(&self) -> TIM3PRIV_R {
        TIM3PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - privileged access mode for TIM4"]
    #[inline(always)]
    pub fn tim4priv(&self) -> TIM4PRIV_R {
        TIM4PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - privileged access mode for TIM5"]
    #[inline(always)]
    pub fn tim5priv(&self) -> TIM5PRIV_R {
        TIM5PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for TIM6"]
    #[inline(always)]
    pub fn tim6priv(&self) -> TIM6PRIV_R {
        TIM6PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - privileged access mode for TIM7"]
    #[inline(always)]
    pub fn tim7priv(&self) -> TIM7PRIV_R {
        TIM7PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - privileged access mode for TIM12"]
    #[inline(always)]
    pub fn tim12priv(&self) -> TIM12PRIV_R {
        TIM12PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - privileged access mode for TIM13"]
    #[inline(always)]
    pub fn tim13priv(&self) -> TIM13PRIV_R {
        TIM13PRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for TIM14"]
    #[inline(always)]
    pub fn tim14priv(&self) -> TIM14PRIV_R {
        TIM14PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for WWDG"]
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - privileged access mode for IWDG"]
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for SPI2"]
    #[inline(always)]
    pub fn spi2priv(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for SPI3"]
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - privileged access mode for USART2"]
    #[inline(always)]
    pub fn usart2priv(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for USART3"]
    #[inline(always)]
    pub fn usart3priv(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - privileged access mode for UART4"]
    #[inline(always)]
    pub fn uart4priv(&self) -> UART4PRIV_R {
        UART4PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - privileged access mode for UART5"]
    #[inline(always)]
    pub fn uart5priv(&self) -> UART5PRIV_R {
        UART5PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for I2C1"]
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for I2C2"]
    #[inline(always)]
    pub fn i2c2priv(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - privileged access mode for I3C1"]
    #[inline(always)]
    pub fn i3c1priv(&self) -> I3C1PRIV_R {
        I3C1PRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - privileged access mode for CRS"]
    #[inline(always)]
    pub fn crspriv(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - privileged access mode for USART6"]
    #[inline(always)]
    pub fn usart6priv(&self) -> USART6PRIV_R {
        USART6PRIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - privileged access mode for USART10"]
    #[inline(always)]
    pub fn usart10priv(&self) -> USART10PRIV_R {
        USART10PRIV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - privileged access mode for USART11"]
    #[inline(always)]
    pub fn usart11priv(&self) -> USART11PRIV_R {
        USART11PRIV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - privileged access mode for HDMICEC"]
    #[inline(always)]
    pub fn hdmicecpriv(&self) -> HDMICECPRIV_R {
        HDMICECPRIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - privileged access mode for DAC1"]
    #[inline(always)]
    pub fn dac1priv(&self) -> DAC1PRIV_R {
        DAC1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - privileged access mode for UART7"]
    #[inline(always)]
    pub fn uart7priv(&self) -> UART7PRIV_R {
        UART7PRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - privileged access mode for UART8"]
    #[inline(always)]
    pub fn uart8priv(&self) -> UART8PRIV_R {
        UART8PRIV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - privileged access mode for UART9"]
    #[inline(always)]
    pub fn uart9priv(&self) -> UART9PRIV_R {
        UART9PRIV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - privileged access mode for UART12"]
    #[inline(always)]
    pub fn uart12priv(&self) -> UART12PRIV_R {
        UART12PRIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - privileged access mode for DTS"]
    #[inline(always)]
    pub fn dtspriv(&self) -> DTSPRIV_R {
        DTSPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - privileged access mode for LPTIM2"]
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for TIM2"]
    #[inline(always)]
    #[must_use]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W<0> {
        TIM2PRIV_W::new(self)
    }
    #[doc = "Bit 1 - privileged access mode for TIM3"]
    #[inline(always)]
    #[must_use]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W<1> {
        TIM3PRIV_W::new(self)
    }
    #[doc = "Bit 2 - privileged access mode for TIM4"]
    #[inline(always)]
    #[must_use]
    pub fn tim4priv(&mut self) -> TIM4PRIV_W<2> {
        TIM4PRIV_W::new(self)
    }
    #[doc = "Bit 3 - privileged access mode for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn tim5priv(&mut self) -> TIM5PRIV_W<3> {
        TIM5PRIV_W::new(self)
    }
    #[doc = "Bit 4 - privileged access mode for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn tim6priv(&mut self) -> TIM6PRIV_W<4> {
        TIM6PRIV_W::new(self)
    }
    #[doc = "Bit 5 - privileged access mode for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn tim7priv(&mut self) -> TIM7PRIV_W<5> {
        TIM7PRIV_W::new(self)
    }
    #[doc = "Bit 6 - privileged access mode for TIM12"]
    #[inline(always)]
    #[must_use]
    pub fn tim12priv(&mut self) -> TIM12PRIV_W<6> {
        TIM12PRIV_W::new(self)
    }
    #[doc = "Bit 7 - privileged access mode for TIM13"]
    #[inline(always)]
    #[must_use]
    pub fn tim13priv(&mut self) -> TIM13PRIV_W<7> {
        TIM13PRIV_W::new(self)
    }
    #[doc = "Bit 8 - privileged access mode for TIM14"]
    #[inline(always)]
    #[must_use]
    pub fn tim14priv(&mut self) -> TIM14PRIV_W<8> {
        TIM14PRIV_W::new(self)
    }
    #[doc = "Bit 9 - privileged access mode for WWDG"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W<9> {
        WWDGPRIV_W::new(self)
    }
    #[doc = "Bit 10 - privileged access mode for IWDG"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W<10> {
        IWDGPRIV_W::new(self)
    }
    #[doc = "Bit 11 - privileged access mode for SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn spi2priv(&mut self) -> SPI2PRIV_W<11> {
        SPI2PRIV_W::new(self)
    }
    #[doc = "Bit 12 - privileged access mode for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W<12> {
        SPI3PRIV_W::new(self)
    }
    #[doc = "Bit 13 - privileged access mode for USART2"]
    #[inline(always)]
    #[must_use]
    pub fn usart2priv(&mut self) -> USART2PRIV_W<13> {
        USART2PRIV_W::new(self)
    }
    #[doc = "Bit 14 - privileged access mode for USART3"]
    #[inline(always)]
    #[must_use]
    pub fn usart3priv(&mut self) -> USART3PRIV_W<14> {
        USART3PRIV_W::new(self)
    }
    #[doc = "Bit 15 - privileged access mode for UART4"]
    #[inline(always)]
    #[must_use]
    pub fn uart4priv(&mut self) -> UART4PRIV_W<15> {
        UART4PRIV_W::new(self)
    }
    #[doc = "Bit 16 - privileged access mode for UART5"]
    #[inline(always)]
    #[must_use]
    pub fn uart5priv(&mut self) -> UART5PRIV_W<16> {
        UART5PRIV_W::new(self)
    }
    #[doc = "Bit 17 - privileged access mode for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W<17> {
        I2C1PRIV_W::new(self)
    }
    #[doc = "Bit 18 - privileged access mode for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2priv(&mut self) -> I2C2PRIV_W<18> {
        I2C2PRIV_W::new(self)
    }
    #[doc = "Bit 19 - privileged access mode for I3C1"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1priv(&mut self) -> I3C1PRIV_W<19> {
        I3C1PRIV_W::new(self)
    }
    #[doc = "Bit 20 - privileged access mode for CRS"]
    #[inline(always)]
    #[must_use]
    pub fn crspriv(&mut self) -> CRSPRIV_W<20> {
        CRSPRIV_W::new(self)
    }
    #[doc = "Bit 21 - privileged access mode for USART6"]
    #[inline(always)]
    #[must_use]
    pub fn usart6priv(&mut self) -> USART6PRIV_W<21> {
        USART6PRIV_W::new(self)
    }
    #[doc = "Bit 22 - privileged access mode for USART10"]
    #[inline(always)]
    #[must_use]
    pub fn usart10priv(&mut self) -> USART10PRIV_W<22> {
        USART10PRIV_W::new(self)
    }
    #[doc = "Bit 23 - privileged access mode for USART11"]
    #[inline(always)]
    #[must_use]
    pub fn usart11priv(&mut self) -> USART11PRIV_W<23> {
        USART11PRIV_W::new(self)
    }
    #[doc = "Bit 24 - privileged access mode for HDMICEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmicecpriv(&mut self) -> HDMICECPRIV_W<24> {
        HDMICECPRIV_W::new(self)
    }
    #[doc = "Bit 25 - privileged access mode for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn dac1priv(&mut self) -> DAC1PRIV_W<25> {
        DAC1PRIV_W::new(self)
    }
    #[doc = "Bit 26 - privileged access mode for UART7"]
    #[inline(always)]
    #[must_use]
    pub fn uart7priv(&mut self) -> UART7PRIV_W<26> {
        UART7PRIV_W::new(self)
    }
    #[doc = "Bit 27 - privileged access mode for UART8"]
    #[inline(always)]
    #[must_use]
    pub fn uart8priv(&mut self) -> UART8PRIV_W<27> {
        UART8PRIV_W::new(self)
    }
    #[doc = "Bit 28 - privileged access mode for UART9"]
    #[inline(always)]
    #[must_use]
    pub fn uart9priv(&mut self) -> UART9PRIV_W<28> {
        UART9PRIV_W::new(self)
    }
    #[doc = "Bit 29 - privileged access mode for UART12"]
    #[inline(always)]
    #[must_use]
    pub fn uart12priv(&mut self) -> UART12PRIV_W<29> {
        UART12PRIV_W::new(self)
    }
    #[doc = "Bit 30 - privileged access mode for DTS"]
    #[inline(always)]
    #[must_use]
    pub fn dtspriv(&mut self) -> DTSPRIV_W<30> {
        DTSPRIV_W::new(self)
    }
    #[doc = "Bit 31 - privileged access mode for LPTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W<31> {
        LPTIM2PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzsc_privcfgr1](index.html) module"]
pub struct GTZC1_TZSC_PRIVCFGR1_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_PRIVCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzsc_privcfgr1::R](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_PRIVCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzsc_privcfgr1::W](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_PRIVCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_PRIVCFGR1 to value 0"]
impl crate::Resettable for GTZC1_TZSC_PRIVCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
