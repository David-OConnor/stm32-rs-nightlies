#[doc = "Register `GTZC1_TZSC_SECCFGR1` reader"]
pub struct R(crate::R<GTZC1_TZSC_SECCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_SECCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_SECCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_SECCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZSC_SECCFGR1` writer"]
pub struct W(crate::W<GTZC1_TZSC_SECCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_SECCFGR1_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_SECCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_SECCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SEC` reader - secure access mode for TIM2"]
pub type TIM2SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM2SEC` writer - secure access mode for TIM2"]
pub type TIM2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM3SEC` reader - secure access mode for TIM3"]
pub type TIM3SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM3SEC` writer - secure access mode for TIM3"]
pub type TIM3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM4SEC` reader - secure access mode for TIM4"]
pub type TIM4SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM4SEC` writer - secure access mode for TIM4"]
pub type TIM4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM5SEC` reader - secure access mode for TIM5"]
pub type TIM5SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM5SEC` writer - secure access mode for TIM5"]
pub type TIM5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM6SEC` reader - secure access mode for TIM6"]
pub type TIM6SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM6SEC` writer - secure access mode for TIM6"]
pub type TIM6SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM7SEC` reader - secure access mode for TIM7"]
pub type TIM7SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM7SEC` writer - secure access mode for TIM7"]
pub type TIM7SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM12SEC` reader - secure access mode for TIM12"]
pub type TIM12SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM12SEC` writer - secure access mode for TIM12"]
pub type TIM12SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM13SEC` reader - secure access mode for TIM13"]
pub type TIM13SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM13SEC` writer - secure access mode for TIM13"]
pub type TIM13SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM14SEC` reader - secure access mode for TIM14"]
pub type TIM14SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM14SEC` writer - secure access mode for TIM14"]
pub type TIM14SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `WWDGSEC` reader - secure access mode for WWDG"]
pub type WWDGSEC_R = crate::BitReader<bool>;
#[doc = "Field `WWDGSEC` writer - secure access mode for WWDG"]
pub type WWDGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `IWDGSEC` reader - secure access mode for IWDG"]
pub type IWDGSEC_R = crate::BitReader<bool>;
#[doc = "Field `IWDGSEC` writer - secure access mode for IWDG"]
pub type IWDGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI2SEC` reader - secure access mode for SPI2"]
pub type SPI2SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI2SEC` writer - secure access mode for SPI2"]
pub type SPI2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI3SEC` reader - secure access mode for SPI3"]
pub type SPI3SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI3SEC` writer - secure access mode for SPI3"]
pub type SPI3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `USART2SEC` reader - secure access mode for USART2"]
pub type USART2SEC_R = crate::BitReader<bool>;
#[doc = "Field `USART2SEC` writer - secure access mode for USART2"]
pub type USART2SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `USART3SEC` reader - secure access mode for USART3"]
pub type USART3SEC_R = crate::BitReader<bool>;
#[doc = "Field `USART3SEC` writer - secure access mode for USART3"]
pub type USART3SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UART4SEC` reader - secure access mode for UART4"]
pub type UART4SEC_R = crate::BitReader<bool>;
#[doc = "Field `UART4SEC` writer - secure access mode for UART4"]
pub type UART4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UART5SEC` reader - secure access mode for UART5"]
pub type UART5SEC_R = crate::BitReader<bool>;
#[doc = "Field `UART5SEC` writer - secure access mode for UART5"]
pub type UART5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C1SEC` reader - secure access mode for I2C1"]
pub type I2C1SEC_R = crate::BitReader<bool>;
#[doc = "Field `I2C1SEC` writer - secure access mode for I2C1"]
pub type I2C1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C2SEC` reader - secure access mode for I2C2"]
pub type I2C2SEC_R = crate::BitReader<bool>;
#[doc = "Field `I2C2SEC` writer - secure access mode for I2C2"]
pub type I2C2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `I3C1SEC` reader - secure access mode for I3C1"]
pub type I3C1SEC_R = crate::BitReader<bool>;
#[doc = "Field `I3C1SEC` writer - secure access mode for I3C1"]
pub type I3C1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `CRSSEC` reader - secure access mode for CRS"]
pub type CRSSEC_R = crate::BitReader<bool>;
#[doc = "Field `CRSSEC` writer - secure access mode for CRS"]
pub type CRSSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `USART6SEC` reader - secure access mode for USART6"]
pub type USART6SEC_R = crate::BitReader<bool>;
#[doc = "Field `USART6SEC` writer - secure access mode for USART6"]
pub type USART6SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `USART10SEC` reader - secure access mode for USART10"]
pub type USART10SEC_R = crate::BitReader<bool>;
#[doc = "Field `USART10SEC` writer - secure access mode for USART10"]
pub type USART10SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `USART11SEC` reader - secure access mode for USART11"]
pub type USART11SEC_R = crate::BitReader<bool>;
#[doc = "Field `USART11SEC` writer - secure access mode for USART11"]
pub type USART11SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `HDMICECSEC` reader - secure access mode for HDMICEC"]
pub type HDMICECSEC_R = crate::BitReader<bool>;
#[doc = "Field `HDMICECSEC` writer - secure access mode for HDMICEC"]
pub type HDMICECSEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `DAC1SEC` reader - secure access mode for DAC1"]
pub type DAC1SEC_R = crate::BitReader<bool>;
#[doc = "Field `DAC1SEC` writer - secure access mode for DAC1"]
pub type DAC1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UART7SEC` reader - secure access mode for UART7"]
pub type UART7SEC_R = crate::BitReader<bool>;
#[doc = "Field `UART7SEC` writer - secure access mode for UART7"]
pub type UART7SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UART8SEC` reader - secure access mode for UART8"]
pub type UART8SEC_R = crate::BitReader<bool>;
#[doc = "Field `UART8SEC` writer - secure access mode for UART8"]
pub type UART8SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UART9SEC` reader - secure access mode for UART9"]
pub type UART9SEC_R = crate::BitReader<bool>;
#[doc = "Field `UART9SEC` writer - secure access mode for UART9"]
pub type UART9SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UART12SEC` reader - secure access mode for UART12"]
pub type UART12SEC_R = crate::BitReader<bool>;
#[doc = "Field `UART12SEC` writer - secure access mode for UART12"]
pub type UART12SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `DTSSEC` reader - secure access mode for DTS"]
pub type DTSSEC_R = crate::BitReader<bool>;
#[doc = "Field `DTSSEC` writer - secure access mode for DTS"]
pub type DTSSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
#[doc = "Field `LPTIM2SEC` reader - secure access mode for LPTIM2"]
pub type LPTIM2SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2SEC` writer - secure access mode for LPTIM2"]
pub type LPTIM2SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - secure access mode for TIM2"]
    #[inline(always)]
    pub fn tim2sec(&self) -> TIM2SEC_R {
        TIM2SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure access mode for TIM3"]
    #[inline(always)]
    pub fn tim3sec(&self) -> TIM3SEC_R {
        TIM3SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - secure access mode for TIM4"]
    #[inline(always)]
    pub fn tim4sec(&self) -> TIM4SEC_R {
        TIM4SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - secure access mode for TIM5"]
    #[inline(always)]
    pub fn tim5sec(&self) -> TIM5SEC_R {
        TIM5SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - secure access mode for TIM6"]
    #[inline(always)]
    pub fn tim6sec(&self) -> TIM6SEC_R {
        TIM6SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - secure access mode for TIM7"]
    #[inline(always)]
    pub fn tim7sec(&self) -> TIM7SEC_R {
        TIM7SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - secure access mode for TIM12"]
    #[inline(always)]
    pub fn tim12sec(&self) -> TIM12SEC_R {
        TIM12SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - secure access mode for TIM13"]
    #[inline(always)]
    pub fn tim13sec(&self) -> TIM13SEC_R {
        TIM13SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - secure access mode for TIM14"]
    #[inline(always)]
    pub fn tim14sec(&self) -> TIM14SEC_R {
        TIM14SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - secure access mode for WWDG"]
    #[inline(always)]
    pub fn wwdgsec(&self) -> WWDGSEC_R {
        WWDGSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - secure access mode for IWDG"]
    #[inline(always)]
    pub fn iwdgsec(&self) -> IWDGSEC_R {
        IWDGSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - secure access mode for SPI2"]
    #[inline(always)]
    pub fn spi2sec(&self) -> SPI2SEC_R {
        SPI2SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - secure access mode for SPI3"]
    #[inline(always)]
    pub fn spi3sec(&self) -> SPI3SEC_R {
        SPI3SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - secure access mode for USART2"]
    #[inline(always)]
    pub fn usart2sec(&self) -> USART2SEC_R {
        USART2SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - secure access mode for USART3"]
    #[inline(always)]
    pub fn usart3sec(&self) -> USART3SEC_R {
        USART3SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - secure access mode for UART4"]
    #[inline(always)]
    pub fn uart4sec(&self) -> UART4SEC_R {
        UART4SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - secure access mode for UART5"]
    #[inline(always)]
    pub fn uart5sec(&self) -> UART5SEC_R {
        UART5SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - secure access mode for I2C1"]
    #[inline(always)]
    pub fn i2c1sec(&self) -> I2C1SEC_R {
        I2C1SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - secure access mode for I2C2"]
    #[inline(always)]
    pub fn i2c2sec(&self) -> I2C2SEC_R {
        I2C2SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - secure access mode for I3C1"]
    #[inline(always)]
    pub fn i3c1sec(&self) -> I3C1SEC_R {
        I3C1SEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - secure access mode for CRS"]
    #[inline(always)]
    pub fn crssec(&self) -> CRSSEC_R {
        CRSSEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - secure access mode for USART6"]
    #[inline(always)]
    pub fn usart6sec(&self) -> USART6SEC_R {
        USART6SEC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - secure access mode for USART10"]
    #[inline(always)]
    pub fn usart10sec(&self) -> USART10SEC_R {
        USART10SEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - secure access mode for USART11"]
    #[inline(always)]
    pub fn usart11sec(&self) -> USART11SEC_R {
        USART11SEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - secure access mode for HDMICEC"]
    #[inline(always)]
    pub fn hdmicecsec(&self) -> HDMICECSEC_R {
        HDMICECSEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - secure access mode for DAC1"]
    #[inline(always)]
    pub fn dac1sec(&self) -> DAC1SEC_R {
        DAC1SEC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - secure access mode for UART7"]
    #[inline(always)]
    pub fn uart7sec(&self) -> UART7SEC_R {
        UART7SEC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - secure access mode for UART8"]
    #[inline(always)]
    pub fn uart8sec(&self) -> UART8SEC_R {
        UART8SEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - secure access mode for UART9"]
    #[inline(always)]
    pub fn uart9sec(&self) -> UART9SEC_R {
        UART9SEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - secure access mode for UART12"]
    #[inline(always)]
    pub fn uart12sec(&self) -> UART12SEC_R {
        UART12SEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - secure access mode for DTS"]
    #[inline(always)]
    pub fn dtssec(&self) -> DTSSEC_R {
        DTSSEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - secure access mode for LPTIM2"]
    #[inline(always)]
    pub fn lptim2sec(&self) -> LPTIM2SEC_R {
        LPTIM2SEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure access mode for TIM2"]
    #[inline(always)]
    #[must_use]
    pub fn tim2sec(&mut self) -> TIM2SEC_W<0> {
        TIM2SEC_W::new(self)
    }
    #[doc = "Bit 1 - secure access mode for TIM3"]
    #[inline(always)]
    #[must_use]
    pub fn tim3sec(&mut self) -> TIM3SEC_W<1> {
        TIM3SEC_W::new(self)
    }
    #[doc = "Bit 2 - secure access mode for TIM4"]
    #[inline(always)]
    #[must_use]
    pub fn tim4sec(&mut self) -> TIM4SEC_W<2> {
        TIM4SEC_W::new(self)
    }
    #[doc = "Bit 3 - secure access mode for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn tim5sec(&mut self) -> TIM5SEC_W<3> {
        TIM5SEC_W::new(self)
    }
    #[doc = "Bit 4 - secure access mode for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn tim6sec(&mut self) -> TIM6SEC_W<4> {
        TIM6SEC_W::new(self)
    }
    #[doc = "Bit 5 - secure access mode for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn tim7sec(&mut self) -> TIM7SEC_W<5> {
        TIM7SEC_W::new(self)
    }
    #[doc = "Bit 6 - secure access mode for TIM12"]
    #[inline(always)]
    #[must_use]
    pub fn tim12sec(&mut self) -> TIM12SEC_W<6> {
        TIM12SEC_W::new(self)
    }
    #[doc = "Bit 7 - secure access mode for TIM13"]
    #[inline(always)]
    #[must_use]
    pub fn tim13sec(&mut self) -> TIM13SEC_W<7> {
        TIM13SEC_W::new(self)
    }
    #[doc = "Bit 8 - secure access mode for TIM14"]
    #[inline(always)]
    #[must_use]
    pub fn tim14sec(&mut self) -> TIM14SEC_W<8> {
        TIM14SEC_W::new(self)
    }
    #[doc = "Bit 9 - secure access mode for WWDG"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsec(&mut self) -> WWDGSEC_W<9> {
        WWDGSEC_W::new(self)
    }
    #[doc = "Bit 10 - secure access mode for IWDG"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgsec(&mut self) -> IWDGSEC_W<10> {
        IWDGSEC_W::new(self)
    }
    #[doc = "Bit 11 - secure access mode for SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn spi2sec(&mut self) -> SPI2SEC_W<11> {
        SPI2SEC_W::new(self)
    }
    #[doc = "Bit 12 - secure access mode for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn spi3sec(&mut self) -> SPI3SEC_W<12> {
        SPI3SEC_W::new(self)
    }
    #[doc = "Bit 13 - secure access mode for USART2"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sec(&mut self) -> USART2SEC_W<13> {
        USART2SEC_W::new(self)
    }
    #[doc = "Bit 14 - secure access mode for USART3"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sec(&mut self) -> USART3SEC_W<14> {
        USART3SEC_W::new(self)
    }
    #[doc = "Bit 15 - secure access mode for UART4"]
    #[inline(always)]
    #[must_use]
    pub fn uart4sec(&mut self) -> UART4SEC_W<15> {
        UART4SEC_W::new(self)
    }
    #[doc = "Bit 16 - secure access mode for UART5"]
    #[inline(always)]
    #[must_use]
    pub fn uart5sec(&mut self) -> UART5SEC_W<16> {
        UART5SEC_W::new(self)
    }
    #[doc = "Bit 17 - secure access mode for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sec(&mut self) -> I2C1SEC_W<17> {
        I2C1SEC_W::new(self)
    }
    #[doc = "Bit 18 - secure access mode for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sec(&mut self) -> I2C2SEC_W<18> {
        I2C2SEC_W::new(self)
    }
    #[doc = "Bit 19 - secure access mode for I3C1"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1sec(&mut self) -> I3C1SEC_W<19> {
        I3C1SEC_W::new(self)
    }
    #[doc = "Bit 20 - secure access mode for CRS"]
    #[inline(always)]
    #[must_use]
    pub fn crssec(&mut self) -> CRSSEC_W<20> {
        CRSSEC_W::new(self)
    }
    #[doc = "Bit 21 - secure access mode for USART6"]
    #[inline(always)]
    #[must_use]
    pub fn usart6sec(&mut self) -> USART6SEC_W<21> {
        USART6SEC_W::new(self)
    }
    #[doc = "Bit 22 - secure access mode for USART10"]
    #[inline(always)]
    #[must_use]
    pub fn usart10sec(&mut self) -> USART10SEC_W<22> {
        USART10SEC_W::new(self)
    }
    #[doc = "Bit 23 - secure access mode for USART11"]
    #[inline(always)]
    #[must_use]
    pub fn usart11sec(&mut self) -> USART11SEC_W<23> {
        USART11SEC_W::new(self)
    }
    #[doc = "Bit 24 - secure access mode for HDMICEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmicecsec(&mut self) -> HDMICECSEC_W<24> {
        HDMICECSEC_W::new(self)
    }
    #[doc = "Bit 25 - secure access mode for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn dac1sec(&mut self) -> DAC1SEC_W<25> {
        DAC1SEC_W::new(self)
    }
    #[doc = "Bit 26 - secure access mode for UART7"]
    #[inline(always)]
    #[must_use]
    pub fn uart7sec(&mut self) -> UART7SEC_W<26> {
        UART7SEC_W::new(self)
    }
    #[doc = "Bit 27 - secure access mode for UART8"]
    #[inline(always)]
    #[must_use]
    pub fn uart8sec(&mut self) -> UART8SEC_W<27> {
        UART8SEC_W::new(self)
    }
    #[doc = "Bit 28 - secure access mode for UART9"]
    #[inline(always)]
    #[must_use]
    pub fn uart9sec(&mut self) -> UART9SEC_W<28> {
        UART9SEC_W::new(self)
    }
    #[doc = "Bit 29 - secure access mode for UART12"]
    #[inline(always)]
    #[must_use]
    pub fn uart12sec(&mut self) -> UART12SEC_W<29> {
        UART12SEC_W::new(self)
    }
    #[doc = "Bit 30 - secure access mode for DTS"]
    #[inline(always)]
    #[must_use]
    pub fn dtssec(&mut self) -> DTSSEC_W<30> {
        DTSSEC_W::new(self)
    }
    #[doc = "Bit 31 - secure access mode for LPTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sec(&mut self) -> LPTIM2SEC_W<31> {
        LPTIM2SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZSC secure configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzsc_seccfgr1](index.html) module"]
pub struct GTZC1_TZSC_SECCFGR1_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_SECCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzsc_seccfgr1::R](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_SECCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzsc_seccfgr1::W](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_SECCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_SECCFGR1 to value 0"]
impl crate::Resettable for GTZC1_TZSC_SECCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
