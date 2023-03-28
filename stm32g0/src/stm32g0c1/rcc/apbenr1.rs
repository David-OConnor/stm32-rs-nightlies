#[doc = "Register `APBENR1` reader"]
pub struct R(crate::R<APBENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBENR1` writer"]
pub struct W(crate::W<APBENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBENR1_SPEC>;
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
impl From<crate::W<APBENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2EN` reader - TIM2 timer clock enable"]
pub type TIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2EN` writer - TIM2 timer clock enable"]
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable"]
pub type TIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable"]
pub type TIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `TIM4EN` reader - TIM4 timer clock enable"]
pub type TIM4EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4EN` writer - TIM4 timer clock enable"]
pub type TIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `TIM6EN` reader - TIM6 timer clock enable"]
pub type TIM6EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6EN` writer - TIM6 timer clock enable"]
pub type TIM6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `TIM7EN` reader - TIM7 timer clock enable"]
pub type TIM7EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7EN` writer - TIM7 timer clock enable"]
pub type TIM7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `LPUART2EN` reader - LPUART2 clock enable"]
pub type LPUART2EN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART2EN` writer - LPUART2 clock enable"]
pub type LPUART2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `USART5EN` reader - USART5EN"]
pub type USART5EN_R = crate::BitReader<bool>;
#[doc = "Field `USART5EN` writer - USART5EN"]
pub type USART5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `USART6EN` reader - USART6EN"]
pub type USART6EN_R = crate::BitReader<bool>;
#[doc = "Field `USART6EN` writer - USART6EN"]
pub type USART6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable"]
pub type RTCAPBEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable"]
pub type RTCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `WWDGEN` reader - WWDG clock enable"]
pub type WWDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGEN` writer - WWDG clock enable"]
pub type WWDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `FDCANEN` reader - USBEN"]
pub type FDCANEN_R = crate::BitReader<bool>;
#[doc = "Field `FDCANEN` writer - USBEN"]
pub type FDCANEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `USBEN` reader - USBEN"]
pub type USBEN_R = crate::BitReader<bool>;
#[doc = "Field `USBEN` writer - USBEN"]
pub type USBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type SPI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type SPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable"]
pub type SPI3EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable"]
pub type SPI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `CRSEN` reader - CRSEN"]
pub type CRSEN_R = crate::BitReader<bool>;
#[doc = "Field `CRSEN` writer - CRSEN"]
pub type CRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type USART2EN_R = crate::BitReader<bool>;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type USART2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `USART3EN` reader - USART3 clock enable"]
pub type USART3EN_R = crate::BitReader<bool>;
#[doc = "Field `USART3EN` writer - USART3 clock enable"]
pub type USART3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `USART4EN` reader - USART4 clock enable"]
pub type USART4EN_R = crate::BitReader<bool>;
#[doc = "Field `USART4EN` writer - USART4 clock enable"]
pub type USART4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable"]
pub type LPUART1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable"]
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub type I2C2EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub type I2C2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable"]
pub type I2C3EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable"]
pub type I2C3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `CECEN` reader - HDMI CEC clock enable"]
pub type CECEN_R = crate::BitReader<bool>;
#[doc = "Field `CECEN` writer - HDMI CEC clock enable"]
pub type CECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `UCPD1EN` reader - UCPD1 clock enable"]
pub type UCPD1EN_R = crate::BitReader<bool>;
#[doc = "Field `UCPD1EN` writer - UCPD1 clock enable"]
pub type UCPD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `UCPD2EN` reader - UCPD2 clock enable"]
pub type UCPD2EN_R = crate::BitReader<bool>;
#[doc = "Field `UCPD2EN` writer - UCPD2 clock enable"]
pub type UCPD2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `DBGEN` reader - Debug support clock enable"]
pub type DBGEN_R = crate::BitReader<bool>;
#[doc = "Field `DBGEN` writer - Debug support clock enable"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PWREN_R = crate::BitReader<bool>;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `DAC1EN` reader - DAC1 interface clock enable"]
pub type DAC1EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC1EN` writer - DAC1 interface clock enable"]
pub type DAC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `LPTIM2EN` reader - LPTIM2 clock enable"]
pub type LPTIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 clock enable"]
pub type LPTIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable"]
pub type LPTIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable"]
pub type LPTIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPUART2 clock enable"]
    #[inline(always)]
    pub fn lpuart2en(&self) -> LPUART2EN_R {
        LPUART2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USART5EN"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART6EN"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USBEN"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USBEN"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CRSEN"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - HDMI CEC clock enable"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UCPD1 clock enable"]
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - UCPD2 clock enable"]
    #[inline(always)]
    pub fn ucpd2en(&self) -> UCPD2EN_R {
        UCPD2EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable"]
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LPTIM2 clock enable"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<2> {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 7 - LPUART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2en(&mut self) -> LPUART2EN_W<7> {
        LPUART2EN_W::new(self)
    }
    #[doc = "Bit 8 - USART5EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart5en(&mut self) -> USART5EN_W<8> {
        USART5EN_W::new(self)
    }
    #[doc = "Bit 9 - USART6EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<9> {
        USART6EN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<10> {
        RTCAPBEN_W::new(self)
    }
    #[doc = "Bit 11 - WWDG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 12 - USBEN"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<12> {
        FDCANEN_W::new(self)
    }
    #[doc = "Bit 13 - USBEN"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<13> {
        USBEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<15> {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 16 - CRSEN"]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<16> {
        CRSEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<18> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart4en(&mut self) -> USART4EN_W<19> {
        USART4EN_W::new(self)
    }
    #[doc = "Bit 20 - LPUART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<20> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<23> {
        I2C3EN_W::new(self)
    }
    #[doc = "Bit 24 - HDMI CEC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<24> {
        CECEN_W::new(self)
    }
    #[doc = "Bit 25 - UCPD1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<25> {
        UCPD1EN_W::new(self)
    }
    #[doc = "Bit 26 - UCPD2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd2en(&mut self) -> UCPD2EN_W<26> {
        UCPD2EN_W::new(self)
    }
    #[doc = "Bit 27 - Debug support clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<27> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<29> {
        DAC1EN_W::new(self)
    }
    #[doc = "Bit 30 - LPTIM2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<30> {
        LPTIM2EN_W::new(self)
    }
    #[doc = "Bit 31 - LPTIM1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<31> {
        LPTIM1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral clock enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbenr1](index.html) module"]
pub struct APBENR1_SPEC;
impl crate::RegisterSpec for APBENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbenr1::R](R) reader structure"]
impl crate::Readable for APBENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbenr1::W](W) writer structure"]
impl crate::Writable for APBENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBENR1 to value 0"]
impl crate::Resettable for APBENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
