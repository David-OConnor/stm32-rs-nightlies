#[doc = "Register `PRIVCFGR1` reader"]
pub struct R(crate::R<PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCFGR1` writer"]
pub struct W(crate::W<PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR1_SPEC>;
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
impl From<crate::W<PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2PRIV` reader - TIM2PRIV"]
pub type TIM2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM2PRIV` writer - TIM2PRIV"]
pub type TIM2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM3PRIV` reader - TIM3PRIV"]
pub type TIM3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM3PRIV` writer - TIM3PRIV"]
pub type TIM3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM4PRIV` reader - TIM4PRIV"]
pub type TIM4PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM4PRIV` writer - TIM4PRIV"]
pub type TIM4PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM5PRIV` reader - TIM5PRIV"]
pub type TIM5PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM5PRIV` writer - TIM5PRIV"]
pub type TIM5PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM6PRIV` reader - TIM6PRIV"]
pub type TIM6PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM6PRIV` writer - TIM6PRIV"]
pub type TIM6PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM7PRIV` reader - TIM7PRIV"]
pub type TIM7PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM7PRIV` writer - TIM7PRIV"]
pub type TIM7PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `WWDGPRIV` reader - WWDGPRIV"]
pub type WWDGPRIV_R = crate::BitReader<bool>;
#[doc = "Field `WWDGPRIV` writer - WWDGPRIV"]
pub type WWDGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `IWDGPRIV` reader - IWDGPRIV"]
pub type IWDGPRIV_R = crate::BitReader<bool>;
#[doc = "Field `IWDGPRIV` writer - IWDGPRIV"]
pub type IWDGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI2PRIV` reader - SPI2PRIV"]
pub type SPI2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI2PRIV` writer - SPI2PRIV"]
pub type SPI2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI3PRIV` reader - SPI3PRIV"]
pub type SPI3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI3PRIV` writer - SPI3PRIV"]
pub type SPI3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `USART2PRIV` reader - USART2PRIV"]
pub type USART2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART2PRIV` writer - USART2PRIV"]
pub type USART2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `USART3PRIV` reader - USART3PRIV"]
pub type USART3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART3PRIV` writer - USART3PRIV"]
pub type USART3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UART4PRIV` reader - UART4PRIV"]
pub type UART4PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UART4PRIV` writer - UART4PRIV"]
pub type UART4PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UART5PRIV` reader - UART5PRIV"]
pub type UART5PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UART5PRIV` writer - UART5PRIV"]
pub type UART5PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C1PRIV` reader - I2C1PRIV"]
pub type I2C1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I2C1PRIV` writer - I2C1PRIV"]
pub type I2C1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C2PRIV` reader - I2C2PRIV"]
pub type I2C2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I2C2PRIV` writer - I2C2PRIV"]
pub type I2C2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C3PRIV` reader - I2C3PRIV"]
pub type I2C3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I2C3PRIV` writer - I2C3PRIV"]
pub type I2C3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `CRSPRIV` reader - CRSPRIV"]
pub type CRSPRIV_R = crate::BitReader<bool>;
#[doc = "Field `CRSPRIV` writer - CRSPRIV"]
pub type CRSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `DACPRIV` reader - DACPRIV"]
pub type DACPRIV_R = crate::BitReader<bool>;
#[doc = "Field `DACPRIV` writer - DACPRIV"]
pub type DACPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `OPAMPPRIV` reader - OPAMPPRIV"]
pub type OPAMPPRIV_R = crate::BitReader<bool>;
#[doc = "Field `OPAMPPRIV` writer - OPAMPPRIV"]
pub type OPAMPPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `LPTIM1PRIV` reader - LPTIM1PRIV"]
pub type LPTIM1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1PRIV` writer - LPTIM1PRIV"]
pub type LPTIM1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `LPUART1PRIV` reader - LPUART1PRIV"]
pub type LPUART1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1PRIV` writer - LPUART1PRIV"]
pub type LPUART1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C4PRIV` reader - I2C4PRIV"]
pub type I2C4PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I2C4PRIV` writer - I2C4PRIV"]
pub type I2C4PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `LPTIM2PRIV` reader - LPTIM2PRIV"]
pub type LPTIM2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2PRIV` writer - LPTIM2PRIV"]
pub type LPTIM2PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `LPTIM3PRIV` reader - LPTIM3PRIV"]
pub type LPTIM3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3PRIV` writer - LPTIM3PRIV"]
pub type LPTIM3PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `FDCAN1PRIV` reader - FDCAN1PRIV"]
pub type FDCAN1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN1PRIV` writer - FDCAN1PRIV"]
pub type FDCAN1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `USBFSPRIV` reader - USBFSPRIV"]
pub type USBFSPRIV_R = crate::BitReader<bool>;
#[doc = "Field `USBFSPRIV` writer - USBFSPRIV"]
pub type USBFSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `UCPD1PRIV` reader - UCPD1PRIV"]
pub type UCPD1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `UCPD1PRIV` writer - UCPD1PRIV"]
pub type UCPD1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `VREFBUFPRIV` reader - VREFBUFPRIV"]
pub type VREFBUFPRIV_R = crate::BitReader<bool>;
#[doc = "Field `VREFBUFPRIV` writer - VREFBUFPRIV"]
pub type VREFBUFPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `COMPPRIV` reader - COMPPRIV"]
pub type COMPPRIV_R = crate::BitReader<bool>;
#[doc = "Field `COMPPRIV` writer - COMPPRIV"]
pub type COMPPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM1PRIV` reader - TIM1PRIV"]
pub type TIM1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM1PRIV` writer - TIM1PRIV"]
pub type TIM1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI1PRIV` reader - SPI1PRIV"]
pub type SPI1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI1PRIV` writer - SPI1PRIV"]
pub type SPI1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2PRIV"]
    #[inline(always)]
    pub fn tim2priv(&self) -> TIM2PRIV_R {
        TIM2PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3PRIV"]
    #[inline(always)]
    pub fn tim3priv(&self) -> TIM3PRIV_R {
        TIM3PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4PRIV"]
    #[inline(always)]
    pub fn tim4priv(&self) -> TIM4PRIV_R {
        TIM4PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5PRIV"]
    #[inline(always)]
    pub fn tim5priv(&self) -> TIM5PRIV_R {
        TIM5PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6PRIV"]
    #[inline(always)]
    pub fn tim6priv(&self) -> TIM6PRIV_R {
        TIM6PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7PRIV"]
    #[inline(always)]
    pub fn tim7priv(&self) -> TIM7PRIV_R {
        TIM7PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDGPRIV"]
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IWDGPRIV"]
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI2PRIV"]
    #[inline(always)]
    pub fn spi2priv(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI3PRIV"]
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USART2PRIV"]
    #[inline(always)]
    pub fn usart2priv(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USART3PRIV"]
    #[inline(always)]
    pub fn usart3priv(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART4PRIV"]
    #[inline(always)]
    pub fn uart4priv(&self) -> UART4PRIV_R {
        UART4PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UART5PRIV"]
    #[inline(always)]
    pub fn uart5priv(&self) -> UART5PRIV_R {
        UART5PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C1PRIV"]
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C2PRIV"]
    #[inline(always)]
    pub fn i2c2priv(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C3PRIV"]
    #[inline(always)]
    pub fn i2c3priv(&self) -> I2C3PRIV_R {
        I2C3PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CRSPRIV"]
    #[inline(always)]
    pub fn crspriv(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DACPRIV"]
    #[inline(always)]
    pub fn dacpriv(&self) -> DACPRIV_R {
        DACPRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPAMPPRIV"]
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPTIM1PRIV"]
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPUART1PRIV"]
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C4PRIV"]
    #[inline(always)]
    pub fn i2c4priv(&self) -> I2C4PRIV_R {
        I2C4PRIV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LPTIM2PRIV"]
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LPTIM3PRIV"]
    #[inline(always)]
    pub fn lptim3priv(&self) -> LPTIM3PRIV_R {
        LPTIM3PRIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN1PRIV"]
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USBFSPRIV"]
    #[inline(always)]
    pub fn usbfspriv(&self) -> USBFSPRIV_R {
        USBFSPRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - UCPD1PRIV"]
    #[inline(always)]
    pub fn ucpd1priv(&self) -> UCPD1PRIV_R {
        UCPD1PRIV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - VREFBUFPRIV"]
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - COMPPRIV"]
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TIM1PRIV"]
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI1PRIV"]
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W<0> {
        TIM2PRIV_W::new(self)
    }
    #[doc = "Bit 1 - TIM3PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W<1> {
        TIM3PRIV_W::new(self)
    }
    #[doc = "Bit 2 - TIM4PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim4priv(&mut self) -> TIM4PRIV_W<2> {
        TIM4PRIV_W::new(self)
    }
    #[doc = "Bit 3 - TIM5PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim5priv(&mut self) -> TIM5PRIV_W<3> {
        TIM5PRIV_W::new(self)
    }
    #[doc = "Bit 4 - TIM6PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim6priv(&mut self) -> TIM6PRIV_W<4> {
        TIM6PRIV_W::new(self)
    }
    #[doc = "Bit 5 - TIM7PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim7priv(&mut self) -> TIM7PRIV_W<5> {
        TIM7PRIV_W::new(self)
    }
    #[doc = "Bit 6 - WWDGPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W<6> {
        WWDGPRIV_W::new(self)
    }
    #[doc = "Bit 7 - IWDGPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W<7> {
        IWDGPRIV_W::new(self)
    }
    #[doc = "Bit 8 - SPI2PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn spi2priv(&mut self) -> SPI2PRIV_W<8> {
        SPI2PRIV_W::new(self)
    }
    #[doc = "Bit 9 - SPI3PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W<9> {
        SPI3PRIV_W::new(self)
    }
    #[doc = "Bit 10 - USART2PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn usart2priv(&mut self) -> USART2PRIV_W<10> {
        USART2PRIV_W::new(self)
    }
    #[doc = "Bit 11 - USART3PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn usart3priv(&mut self) -> USART3PRIV_W<11> {
        USART3PRIV_W::new(self)
    }
    #[doc = "Bit 12 - UART4PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn uart4priv(&mut self) -> UART4PRIV_W<12> {
        UART4PRIV_W::new(self)
    }
    #[doc = "Bit 13 - UART5PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn uart5priv(&mut self) -> UART5PRIV_W<13> {
        UART5PRIV_W::new(self)
    }
    #[doc = "Bit 14 - I2C1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W<14> {
        I2C1PRIV_W::new(self)
    }
    #[doc = "Bit 15 - I2C2PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2priv(&mut self) -> I2C2PRIV_W<15> {
        I2C2PRIV_W::new(self)
    }
    #[doc = "Bit 16 - I2C3PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3priv(&mut self) -> I2C3PRIV_W<16> {
        I2C3PRIV_W::new(self)
    }
    #[doc = "Bit 17 - CRSPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn crspriv(&mut self) -> CRSPRIV_W<17> {
        CRSPRIV_W::new(self)
    }
    #[doc = "Bit 18 - DACPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn dacpriv(&mut self) -> DACPRIV_W<18> {
        DACPRIV_W::new(self)
    }
    #[doc = "Bit 19 - OPAMPPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W<19> {
        OPAMPPRIV_W::new(self)
    }
    #[doc = "Bit 20 - LPTIM1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<20> {
        LPTIM1PRIV_W::new(self)
    }
    #[doc = "Bit 21 - LPUART1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<21> {
        LPUART1PRIV_W::new(self)
    }
    #[doc = "Bit 22 - I2C4PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4priv(&mut self) -> I2C4PRIV_W<22> {
        I2C4PRIV_W::new(self)
    }
    #[doc = "Bit 23 - LPTIM2PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W<23> {
        LPTIM2PRIV_W::new(self)
    }
    #[doc = "Bit 24 - LPTIM3PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3priv(&mut self) -> LPTIM3PRIV_W<24> {
        LPTIM3PRIV_W::new(self)
    }
    #[doc = "Bit 25 - FDCAN1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W<25> {
        FDCAN1PRIV_W::new(self)
    }
    #[doc = "Bit 26 - USBFSPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn usbfspriv(&mut self) -> USBFSPRIV_W<26> {
        USBFSPRIV_W::new(self)
    }
    #[doc = "Bit 27 - UCPD1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1priv(&mut self) -> UCPD1PRIV_W<27> {
        UCPD1PRIV_W::new(self)
    }
    #[doc = "Bit 28 - VREFBUFPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W<28> {
        VREFBUFPRIV_W::new(self)
    }
    #[doc = "Bit 29 - COMPPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn comppriv(&mut self) -> COMPPRIV_W<29> {
        COMPPRIV_W::new(self)
    }
    #[doc = "Bit 30 - TIM1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W<30> {
        TIM1PRIV_W::new(self)
    }
    #[doc = "Bit 31 - SPI1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W<31> {
        SPI1PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC privilege configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr1](index.html) module"]
pub struct PRIVCFGR1_SPEC;
impl crate::RegisterSpec for PRIVCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcfgr1::R](R) reader structure"]
impl crate::Readable for PRIVCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcfgr1::W](W) writer structure"]
impl crate::Writable for PRIVCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIVCFGR1 to value 0"]
impl crate::Resettable for PRIVCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
