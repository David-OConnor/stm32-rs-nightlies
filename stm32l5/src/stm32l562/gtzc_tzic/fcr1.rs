#[doc = "Register `FCR1` writer"]
pub struct W(crate::W<FCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR1_SPEC>;
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
impl From<crate::W<FCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2FC` writer - TIM2FC"]
pub type TIM2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `TIM3FC` writer - TIM3FC"]
pub type TIM3FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `TIM4FC` writer - TIM4FC"]
pub type TIM4FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `TIM5FC` writer - TIM5FC"]
pub type TIM5FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `TIM6FC` writer - TIM6FC"]
pub type TIM6FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `TIM7FC` writer - TIM7FC"]
pub type TIM7FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `WWDGFC` writer - WWDGFC"]
pub type WWDGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `IWDGFC` writer - IWDGFC"]
pub type IWDGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `SPI2FC` writer - SPI2FC"]
pub type SPI2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `SPI3FC` writer - SPI3FC"]
pub type SPI3FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `USART2FC` writer - USART2FC"]
pub type USART2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `USART3FC` writer - USART3FC"]
pub type USART3FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `UART4FC` writer - UART4FC"]
pub type UART4FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `UART5FC` writer - UART5FC"]
pub type UART5FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `I2C1FC` writer - I2C1FC"]
pub type I2C1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `I2C2FC` writer - I2C2FC"]
pub type I2C2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `I2C3FC` writer - I2C3FC"]
pub type I2C3FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `CRSFC` writer - CRSFC"]
pub type CRSFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `DACFC` writer - DACFC"]
pub type DACFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `OPAMPFC` writer - OPAMPFC"]
pub type OPAMPFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `LPTIM1FC` writer - LPTIM1FC"]
pub type LPTIM1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `LPUART1FC` writer - LPUART1FC"]
pub type LPUART1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `I2C4FC` writer - I2C4FC"]
pub type I2C4FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `LPTIM2FC` writer - LPTIM2FC"]
pub type LPTIM2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `LPTIM3FC` writer - LPTIM3FC"]
pub type LPTIM3FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `FDCAN1FC` writer - FDCAN1FC"]
pub type FDCAN1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `USBFSFC` writer - USBFSFC"]
pub type USBFSFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `UCPD1FC` writer - UCPD1FC"]
pub type UCPD1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `VREFBUFFC` writer - VREFBUFFC"]
pub type VREFBUFFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `COMPFC` writer - COMPFC"]
pub type COMPFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `TIM1FC` writer - TIM1FC"]
pub type TIM1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
#[doc = "Field `SPI1FC` writer - SPI1FC"]
pub type SPI1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - TIM2FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim2fc(&mut self) -> TIM2FC_W<0> {
        TIM2FC_W::new(self)
    }
    #[doc = "Bit 1 - TIM3FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim3fc(&mut self) -> TIM3FC_W<1> {
        TIM3FC_W::new(self)
    }
    #[doc = "Bit 2 - TIM4FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim4fc(&mut self) -> TIM4FC_W<2> {
        TIM4FC_W::new(self)
    }
    #[doc = "Bit 3 - TIM5FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim5fc(&mut self) -> TIM5FC_W<3> {
        TIM5FC_W::new(self)
    }
    #[doc = "Bit 4 - TIM6FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim6fc(&mut self) -> TIM6FC_W<4> {
        TIM6FC_W::new(self)
    }
    #[doc = "Bit 5 - TIM7FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim7fc(&mut self) -> TIM7FC_W<5> {
        TIM7FC_W::new(self)
    }
    #[doc = "Bit 6 - WWDGFC"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgfc(&mut self) -> WWDGFC_W<6> {
        WWDGFC_W::new(self)
    }
    #[doc = "Bit 7 - IWDGFC"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgfc(&mut self) -> IWDGFC_W<7> {
        IWDGFC_W::new(self)
    }
    #[doc = "Bit 8 - SPI2FC"]
    #[inline(always)]
    #[must_use]
    pub fn spi2fc(&mut self) -> SPI2FC_W<8> {
        SPI2FC_W::new(self)
    }
    #[doc = "Bit 9 - SPI3FC"]
    #[inline(always)]
    #[must_use]
    pub fn spi3fc(&mut self) -> SPI3FC_W<9> {
        SPI3FC_W::new(self)
    }
    #[doc = "Bit 10 - USART2FC"]
    #[inline(always)]
    #[must_use]
    pub fn usart2fc(&mut self) -> USART2FC_W<10> {
        USART2FC_W::new(self)
    }
    #[doc = "Bit 11 - USART3FC"]
    #[inline(always)]
    #[must_use]
    pub fn usart3fc(&mut self) -> USART3FC_W<11> {
        USART3FC_W::new(self)
    }
    #[doc = "Bit 12 - UART4FC"]
    #[inline(always)]
    #[must_use]
    pub fn uart4fc(&mut self) -> UART4FC_W<12> {
        UART4FC_W::new(self)
    }
    #[doc = "Bit 13 - UART5FC"]
    #[inline(always)]
    #[must_use]
    pub fn uart5fc(&mut self) -> UART5FC_W<13> {
        UART5FC_W::new(self)
    }
    #[doc = "Bit 14 - I2C1FC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1fc(&mut self) -> I2C1FC_W<14> {
        I2C1FC_W::new(self)
    }
    #[doc = "Bit 15 - I2C2FC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2fc(&mut self) -> I2C2FC_W<15> {
        I2C2FC_W::new(self)
    }
    #[doc = "Bit 16 - I2C3FC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3fc(&mut self) -> I2C3FC_W<16> {
        I2C3FC_W::new(self)
    }
    #[doc = "Bit 17 - CRSFC"]
    #[inline(always)]
    #[must_use]
    pub fn crsfc(&mut self) -> CRSFC_W<17> {
        CRSFC_W::new(self)
    }
    #[doc = "Bit 18 - DACFC"]
    #[inline(always)]
    #[must_use]
    pub fn dacfc(&mut self) -> DACFC_W<18> {
        DACFC_W::new(self)
    }
    #[doc = "Bit 19 - OPAMPFC"]
    #[inline(always)]
    #[must_use]
    pub fn opampfc(&mut self) -> OPAMPFC_W<19> {
        OPAMPFC_W::new(self)
    }
    #[doc = "Bit 20 - LPTIM1FC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1fc(&mut self) -> LPTIM1FC_W<20> {
        LPTIM1FC_W::new(self)
    }
    #[doc = "Bit 21 - LPUART1FC"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1fc(&mut self) -> LPUART1FC_W<21> {
        LPUART1FC_W::new(self)
    }
    #[doc = "Bit 22 - I2C4FC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4fc(&mut self) -> I2C4FC_W<22> {
        I2C4FC_W::new(self)
    }
    #[doc = "Bit 23 - LPTIM2FC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2fc(&mut self) -> LPTIM2FC_W<23> {
        LPTIM2FC_W::new(self)
    }
    #[doc = "Bit 24 - LPTIM3FC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3fc(&mut self) -> LPTIM3FC_W<24> {
        LPTIM3FC_W::new(self)
    }
    #[doc = "Bit 25 - FDCAN1FC"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1fc(&mut self) -> FDCAN1FC_W<25> {
        FDCAN1FC_W::new(self)
    }
    #[doc = "Bit 26 - USBFSFC"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsfc(&mut self) -> USBFSFC_W<26> {
        USBFSFC_W::new(self)
    }
    #[doc = "Bit 27 - UCPD1FC"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1fc(&mut self) -> UCPD1FC_W<27> {
        UCPD1FC_W::new(self)
    }
    #[doc = "Bit 28 - VREFBUFFC"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbuffc(&mut self) -> VREFBUFFC_W<28> {
        VREFBUFFC_W::new(self)
    }
    #[doc = "Bit 29 - COMPFC"]
    #[inline(always)]
    #[must_use]
    pub fn compfc(&mut self) -> COMPFC_W<29> {
        COMPFC_W::new(self)
    }
    #[doc = "Bit 30 - TIM1FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim1fc(&mut self) -> TIM1FC_W<30> {
        TIM1FC_W::new(self)
    }
    #[doc = "Bit 31 - SPI1FC"]
    #[inline(always)]
    #[must_use]
    pub fn spi1fc(&mut self) -> SPI1FC_W<31> {
        SPI1FC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt clear register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr1](index.html) module"]
pub struct FCR1_SPEC;
impl crate::RegisterSpec for FCR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr1::W](W) writer structure"]
impl crate::Writable for FCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR1 to value 0"]
impl crate::Resettable for FCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
