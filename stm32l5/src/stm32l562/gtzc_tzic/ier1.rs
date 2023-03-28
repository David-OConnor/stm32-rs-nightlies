#[doc = "Register `IER1` reader"]
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER1` writer"]
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2IE` reader - TIM2IE"]
pub type TIM2IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM2IE` writer - TIM2IE"]
pub type TIM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TIM3IE` reader - TIM3IE"]
pub type TIM3IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM3IE` writer - TIM3IE"]
pub type TIM3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TIM4IE` reader - TIM4IE"]
pub type TIM4IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM4IE` writer - TIM4IE"]
pub type TIM4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TIM5IE` reader - TIM5IE"]
pub type TIM5IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM5IE` writer - TIM5IE"]
pub type TIM5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TIM6IE` reader - TIM6IE"]
pub type TIM6IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM6IE` writer - TIM6IE"]
pub type TIM6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TIM7IE` reader - TIM7IE"]
pub type TIM7IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM7IE` writer - TIM7IE"]
pub type TIM7IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `WWDGIE` reader - WWDGIE"]
pub type WWDGIE_R = crate::BitReader<bool>;
#[doc = "Field `WWDGIE` writer - WWDGIE"]
pub type WWDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `IWDGIE` reader - IWDGIE"]
pub type IWDGIE_R = crate::BitReader<bool>;
#[doc = "Field `IWDGIE` writer - IWDGIE"]
pub type IWDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `SPI2IE` reader - SPI2IE"]
pub type SPI2IE_R = crate::BitReader<bool>;
#[doc = "Field `SPI2IE` writer - SPI2IE"]
pub type SPI2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `SPI3IE` reader - SPI3IE"]
pub type SPI3IE_R = crate::BitReader<bool>;
#[doc = "Field `SPI3IE` writer - SPI3IE"]
pub type SPI3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `USART2IE` reader - USART2IE"]
pub type USART2IE_R = crate::BitReader<bool>;
#[doc = "Field `USART2IE` writer - USART2IE"]
pub type USART2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `USART3IE` reader - USART3IE"]
pub type USART3IE_R = crate::BitReader<bool>;
#[doc = "Field `USART3IE` writer - USART3IE"]
pub type USART3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `UART4IE` reader - UART4IE"]
pub type UART4IE_R = crate::BitReader<bool>;
#[doc = "Field `UART4IE` writer - UART4IE"]
pub type UART4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `UART5IE` reader - UART5IE"]
pub type UART5IE_R = crate::BitReader<bool>;
#[doc = "Field `UART5IE` writer - UART5IE"]
pub type UART5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `I2C1IE` reader - I2C1IE"]
pub type I2C1IE_R = crate::BitReader<bool>;
#[doc = "Field `I2C1IE` writer - I2C1IE"]
pub type I2C1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `I2C2IE` reader - I2C2IE"]
pub type I2C2IE_R = crate::BitReader<bool>;
#[doc = "Field `I2C2IE` writer - I2C2IE"]
pub type I2C2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `I2C3IE` reader - I2C3IE"]
pub type I2C3IE_R = crate::BitReader<bool>;
#[doc = "Field `I2C3IE` writer - I2C3IE"]
pub type I2C3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `CRSIE` reader - CRSIE"]
pub type CRSIE_R = crate::BitReader<bool>;
#[doc = "Field `CRSIE` writer - CRSIE"]
pub type CRSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `DACIE` reader - DACIE"]
pub type DACIE_R = crate::BitReader<bool>;
#[doc = "Field `DACIE` writer - DACIE"]
pub type DACIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `OPAMPIE` reader - OPAMPIE"]
pub type OPAMPIE_R = crate::BitReader<bool>;
#[doc = "Field `OPAMPIE` writer - OPAMPIE"]
pub type OPAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `LPTIM1IE` reader - LPTIM1IE"]
pub type LPTIM1IE_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1IE` writer - LPTIM1IE"]
pub type LPTIM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `LPUART1IE` reader - LPUART1IE"]
pub type LPUART1IE_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1IE` writer - LPUART1IE"]
pub type LPUART1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `I2C4IE` reader - I2C4IE"]
pub type I2C4IE_R = crate::BitReader<bool>;
#[doc = "Field `I2C4IE` writer - I2C4IE"]
pub type I2C4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `LPTIM2IE` reader - LPTIM2IE"]
pub type LPTIM2IE_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2IE` writer - LPTIM2IE"]
pub type LPTIM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `LPTIM3IE` reader - LPTIM3IE"]
pub type LPTIM3IE_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3IE` writer - LPTIM3IE"]
pub type LPTIM3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `FDCAN1IE` reader - FDCAN1IE"]
pub type FDCAN1IE_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN1IE` writer - FDCAN1IE"]
pub type FDCAN1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `USBFSIE` reader - USBFSIE"]
pub type USBFSIE_R = crate::BitReader<bool>;
#[doc = "Field `USBFSIE` writer - USBFSIE"]
pub type USBFSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `UCPD1IE` reader - UCPD1IE"]
pub type UCPD1IE_R = crate::BitReader<bool>;
#[doc = "Field `UCPD1IE` writer - UCPD1IE"]
pub type UCPD1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `VREFBUFIE` reader - VREFBUFIE"]
pub type VREFBUFIE_R = crate::BitReader<bool>;
#[doc = "Field `VREFBUFIE` writer - VREFBUFIE"]
pub type VREFBUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `COMPIE` reader - COMPIE"]
pub type COMPIE_R = crate::BitReader<bool>;
#[doc = "Field `COMPIE` writer - COMPIE"]
pub type COMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TIM1IE` reader - TIM1IE"]
pub type TIM1IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM1IE` writer - TIM1IE"]
pub type TIM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `SPI1IE` reader - SPI1IE"]
pub type SPI1IE_R = crate::BitReader<bool>;
#[doc = "Field `SPI1IE` writer - SPI1IE"]
pub type SPI1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2IE"]
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3IE"]
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4IE"]
    #[inline(always)]
    pub fn tim4ie(&self) -> TIM4IE_R {
        TIM4IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5IE"]
    #[inline(always)]
    pub fn tim5ie(&self) -> TIM5IE_R {
        TIM5IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6IE"]
    #[inline(always)]
    pub fn tim6ie(&self) -> TIM6IE_R {
        TIM6IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7IE"]
    #[inline(always)]
    pub fn tim7ie(&self) -> TIM7IE_R {
        TIM7IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDGIE"]
    #[inline(always)]
    pub fn wwdgie(&self) -> WWDGIE_R {
        WWDGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IWDGIE"]
    #[inline(always)]
    pub fn iwdgie(&self) -> IWDGIE_R {
        IWDGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI2IE"]
    #[inline(always)]
    pub fn spi2ie(&self) -> SPI2IE_R {
        SPI2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI3IE"]
    #[inline(always)]
    pub fn spi3ie(&self) -> SPI3IE_R {
        SPI3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USART2IE"]
    #[inline(always)]
    pub fn usart2ie(&self) -> USART2IE_R {
        USART2IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USART3IE"]
    #[inline(always)]
    pub fn usart3ie(&self) -> USART3IE_R {
        USART3IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART4IE"]
    #[inline(always)]
    pub fn uart4ie(&self) -> UART4IE_R {
        UART4IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UART5IE"]
    #[inline(always)]
    pub fn uart5ie(&self) -> UART5IE_R {
        UART5IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C1IE"]
    #[inline(always)]
    pub fn i2c1ie(&self) -> I2C1IE_R {
        I2C1IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C2IE"]
    #[inline(always)]
    pub fn i2c2ie(&self) -> I2C2IE_R {
        I2C2IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C3IE"]
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CRSIE"]
    #[inline(always)]
    pub fn crsie(&self) -> CRSIE_R {
        CRSIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DACIE"]
    #[inline(always)]
    pub fn dacie(&self) -> DACIE_R {
        DACIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPAMPIE"]
    #[inline(always)]
    pub fn opampie(&self) -> OPAMPIE_R {
        OPAMPIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPTIM1IE"]
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPUART1IE"]
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C4IE"]
    #[inline(always)]
    pub fn i2c4ie(&self) -> I2C4IE_R {
        I2C4IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LPTIM2IE"]
    #[inline(always)]
    pub fn lptim2ie(&self) -> LPTIM2IE_R {
        LPTIM2IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LPTIM3IE"]
    #[inline(always)]
    pub fn lptim3ie(&self) -> LPTIM3IE_R {
        LPTIM3IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN1IE"]
    #[inline(always)]
    pub fn fdcan1ie(&self) -> FDCAN1IE_R {
        FDCAN1IE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USBFSIE"]
    #[inline(always)]
    pub fn usbfsie(&self) -> USBFSIE_R {
        USBFSIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - UCPD1IE"]
    #[inline(always)]
    pub fn ucpd1ie(&self) -> UCPD1IE_R {
        UCPD1IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - VREFBUFIE"]
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - COMPIE"]
    #[inline(always)]
    pub fn compie(&self) -> COMPIE_R {
        COMPIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TIM1IE"]
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI1IE"]
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim2ie(&mut self) -> TIM2IE_W<0> {
        TIM2IE_W::new(self)
    }
    #[doc = "Bit 1 - TIM3IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim3ie(&mut self) -> TIM3IE_W<1> {
        TIM3IE_W::new(self)
    }
    #[doc = "Bit 2 - TIM4IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim4ie(&mut self) -> TIM4IE_W<2> {
        TIM4IE_W::new(self)
    }
    #[doc = "Bit 3 - TIM5IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim5ie(&mut self) -> TIM5IE_W<3> {
        TIM5IE_W::new(self)
    }
    #[doc = "Bit 4 - TIM6IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim6ie(&mut self) -> TIM6IE_W<4> {
        TIM6IE_W::new(self)
    }
    #[doc = "Bit 5 - TIM7IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim7ie(&mut self) -> TIM7IE_W<5> {
        TIM7IE_W::new(self)
    }
    #[doc = "Bit 6 - WWDGIE"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgie(&mut self) -> WWDGIE_W<6> {
        WWDGIE_W::new(self)
    }
    #[doc = "Bit 7 - IWDGIE"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgie(&mut self) -> IWDGIE_W<7> {
        IWDGIE_W::new(self)
    }
    #[doc = "Bit 8 - SPI2IE"]
    #[inline(always)]
    #[must_use]
    pub fn spi2ie(&mut self) -> SPI2IE_W<8> {
        SPI2IE_W::new(self)
    }
    #[doc = "Bit 9 - SPI3IE"]
    #[inline(always)]
    #[must_use]
    pub fn spi3ie(&mut self) -> SPI3IE_W<9> {
        SPI3IE_W::new(self)
    }
    #[doc = "Bit 10 - USART2IE"]
    #[inline(always)]
    #[must_use]
    pub fn usart2ie(&mut self) -> USART2IE_W<10> {
        USART2IE_W::new(self)
    }
    #[doc = "Bit 11 - USART3IE"]
    #[inline(always)]
    #[must_use]
    pub fn usart3ie(&mut self) -> USART3IE_W<11> {
        USART3IE_W::new(self)
    }
    #[doc = "Bit 12 - UART4IE"]
    #[inline(always)]
    #[must_use]
    pub fn uart4ie(&mut self) -> UART4IE_W<12> {
        UART4IE_W::new(self)
    }
    #[doc = "Bit 13 - UART5IE"]
    #[inline(always)]
    #[must_use]
    pub fn uart5ie(&mut self) -> UART5IE_W<13> {
        UART5IE_W::new(self)
    }
    #[doc = "Bit 14 - I2C1IE"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1ie(&mut self) -> I2C1IE_W<14> {
        I2C1IE_W::new(self)
    }
    #[doc = "Bit 15 - I2C2IE"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2ie(&mut self) -> I2C2IE_W<15> {
        I2C2IE_W::new(self)
    }
    #[doc = "Bit 16 - I2C3IE"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3ie(&mut self) -> I2C3IE_W<16> {
        I2C3IE_W::new(self)
    }
    #[doc = "Bit 17 - CRSIE"]
    #[inline(always)]
    #[must_use]
    pub fn crsie(&mut self) -> CRSIE_W<17> {
        CRSIE_W::new(self)
    }
    #[doc = "Bit 18 - DACIE"]
    #[inline(always)]
    #[must_use]
    pub fn dacie(&mut self) -> DACIE_W<18> {
        DACIE_W::new(self)
    }
    #[doc = "Bit 19 - OPAMPIE"]
    #[inline(always)]
    #[must_use]
    pub fn opampie(&mut self) -> OPAMPIE_W<19> {
        OPAMPIE_W::new(self)
    }
    #[doc = "Bit 20 - LPTIM1IE"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W<20> {
        LPTIM1IE_W::new(self)
    }
    #[doc = "Bit 21 - LPUART1IE"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W<21> {
        LPUART1IE_W::new(self)
    }
    #[doc = "Bit 22 - I2C4IE"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4ie(&mut self) -> I2C4IE_W<22> {
        I2C4IE_W::new(self)
    }
    #[doc = "Bit 23 - LPTIM2IE"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2ie(&mut self) -> LPTIM2IE_W<23> {
        LPTIM2IE_W::new(self)
    }
    #[doc = "Bit 24 - LPTIM3IE"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3ie(&mut self) -> LPTIM3IE_W<24> {
        LPTIM3IE_W::new(self)
    }
    #[doc = "Bit 25 - FDCAN1IE"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1ie(&mut self) -> FDCAN1IE_W<25> {
        FDCAN1IE_W::new(self)
    }
    #[doc = "Bit 26 - USBFSIE"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsie(&mut self) -> USBFSIE_W<26> {
        USBFSIE_W::new(self)
    }
    #[doc = "Bit 27 - UCPD1IE"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1ie(&mut self) -> UCPD1IE_W<27> {
        UCPD1IE_W::new(self)
    }
    #[doc = "Bit 28 - VREFBUFIE"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W<28> {
        VREFBUFIE_W::new(self)
    }
    #[doc = "Bit 29 - COMPIE"]
    #[inline(always)]
    #[must_use]
    pub fn compie(&mut self) -> COMPIE_W<29> {
        COMPIE_W::new(self)
    }
    #[doc = "Bit 30 - TIM1IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim1ie(&mut self) -> TIM1IE_W<30> {
        TIM1IE_W::new(self)
    }
    #[doc = "Bit 31 - SPI1IE"]
    #[inline(always)]
    #[must_use]
    pub fn spi1ie(&mut self) -> SPI1IE_W<31> {
        SPI1IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](index.html) module"]
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier1::R](R) reader structure"]
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier1::W](W) writer structure"]
impl crate::Writable for IER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
