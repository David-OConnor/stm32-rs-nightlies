#[doc = "Register `GTZC1_TZSC_SECCFGR2` reader"]
pub struct R(crate::R<GTZC1_TZSC_SECCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_SECCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_SECCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_SECCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZSC_SECCFGR2` writer"]
pub struct W(crate::W<GTZC1_TZSC_SECCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_SECCFGR2_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_SECCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_SECCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDCAN1SEC` reader - secure access mode for FDCAN1"]
pub type FDCAN1SEC_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN1SEC` writer - secure access mode for FDCAN1"]
pub type FDCAN1SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `FDCAN2SEC` reader - secure access mode for FDCAN2"]
pub type FDCAN2SEC_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN2SEC` writer - secure access mode for FDCAN2"]
pub type FDCAN2SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `UCPDSEC` reader - secure access mode for UCPD"]
pub type UCPDSEC_R = crate::BitReader<bool>;
#[doc = "Field `UCPDSEC` writer - secure access mode for UCPD"]
pub type UCPDSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM1SEC` reader - secure access mode for TIM1"]
pub type TIM1SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM1SEC` writer - secure access mode for TIM1"]
pub type TIM1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SPI1SEC` reader - secure access mode for SPI1"]
pub type SPI1SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI1SEC` writer - secure access mode for SPI1"]
pub type SPI1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM8SEC` reader - secure access mode for TIM8"]
pub type TIM8SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM8SEC` writer - secure access mode for TIM8"]
pub type TIM8SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `USART1SEC` reader - secure access mode for USART1"]
pub type USART1SEC_R = crate::BitReader<bool>;
#[doc = "Field `USART1SEC` writer - secure access mode for USART1"]
pub type USART1SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM15SEC` reader - secure access mode for TIM15"]
pub type TIM15SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM15SEC` writer - secure access mode for TIM15"]
pub type TIM15SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM16SEC` reader - secure access mode for TIM16"]
pub type TIM16SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM16SEC` writer - secure access mode for TIM16"]
pub type TIM16SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM17SEC` reader - secure access mode for TIM17"]
pub type TIM17SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM17SEC` writer - secure access mode for TIM17"]
pub type TIM17SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SPI4SEC` reader - secure access mode for SPI4"]
pub type SPI4SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI4SEC` writer - secure access mode for SPI4"]
pub type SPI4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SPI6SEC` reader - secure access mode for SPI6"]
pub type SPI6SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI6SEC` writer - secure access mode for SPI6"]
pub type SPI6SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SAI1SEC` reader - secure access mode for SAI1"]
pub type SAI1SEC_R = crate::BitReader<bool>;
#[doc = "Field `SAI1SEC` writer - secure access mode for SAI1"]
pub type SAI1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SAI2SEC` reader - secure access mode for SAI2"]
pub type SAI2SEC_R = crate::BitReader<bool>;
#[doc = "Field `SAI2SEC` writer - secure access mode for SAI2"]
pub type SAI2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `USBSEC` reader - secure access mode for USB"]
pub type USBSEC_R = crate::BitReader<bool>;
#[doc = "Field `USBSEC` writer - secure access mode for USB"]
pub type USBSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SPI5SEC` reader - secure access mode for SPI5"]
pub type SPI5SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI5SEC` writer - secure access mode for SPI5"]
pub type SPI5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `LPUART1SEC` reader - secure access mode for LPUART"]
pub type LPUART1SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1SEC` writer - secure access mode for LPUART"]
pub type LPUART1SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `I2C3SEC` reader - secure access mode for I2C3"]
pub type I2C3SEC_R = crate::BitReader<bool>;
#[doc = "Field `I2C3SEC` writer - secure access mode for I2C3"]
pub type I2C3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `I2C4SEC` reader - secure access mode for I2C4"]
pub type I2C4SEC_R = crate::BitReader<bool>;
#[doc = "Field `I2C4SEC` writer - secure access mode for I2C4"]
pub type I2C4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM1SEC` reader - secure access mode for LPTIM1"]
pub type LPTIM1SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1SEC` writer - secure access mode for LPTIM1"]
pub type LPTIM1SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM3SEC` reader - secure access mode for LPTIM3"]
pub type LPTIM3SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3SEC` writer - secure access mode for LPTIM3"]
pub type LPTIM3SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM4SEC` reader - secure access mode for LPTIM4"]
pub type LPTIM4SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM4SEC` writer - secure access mode for LPTIM4"]
pub type LPTIM4SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM5SEC` reader - secure access mode for LPTIM5"]
pub type LPTIM5SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM5SEC` writer - secure access mode for LPTIM5"]
pub type LPTIM5SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - secure access mode for FDCAN1"]
    #[inline(always)]
    pub fn fdcan1sec(&self) -> FDCAN1SEC_R {
        FDCAN1SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure access mode for FDCAN2"]
    #[inline(always)]
    pub fn fdcan2sec(&self) -> FDCAN2SEC_R {
        FDCAN2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - secure access mode for UCPD"]
    #[inline(always)]
    pub fn ucpdsec(&self) -> UCPDSEC_R {
        UCPDSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - secure access mode for TIM1"]
    #[inline(always)]
    pub fn tim1sec(&self) -> TIM1SEC_R {
        TIM1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - secure access mode for SPI1"]
    #[inline(always)]
    pub fn spi1sec(&self) -> SPI1SEC_R {
        SPI1SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - secure access mode for TIM8"]
    #[inline(always)]
    pub fn tim8sec(&self) -> TIM8SEC_R {
        TIM8SEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - secure access mode for USART1"]
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - secure access mode for TIM15"]
    #[inline(always)]
    pub fn tim15sec(&self) -> TIM15SEC_R {
        TIM15SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - secure access mode for TIM16"]
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - secure access mode for TIM17"]
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - secure access mode for SPI4"]
    #[inline(always)]
    pub fn spi4sec(&self) -> SPI4SEC_R {
        SPI4SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - secure access mode for SPI6"]
    #[inline(always)]
    pub fn spi6sec(&self) -> SPI6SEC_R {
        SPI6SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - secure access mode for SAI1"]
    #[inline(always)]
    pub fn sai1sec(&self) -> SAI1SEC_R {
        SAI1SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - secure access mode for SAI2"]
    #[inline(always)]
    pub fn sai2sec(&self) -> SAI2SEC_R {
        SAI2SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - secure access mode for USB"]
    #[inline(always)]
    pub fn usbsec(&self) -> USBSEC_R {
        USBSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - secure access mode for SPI5"]
    #[inline(always)]
    pub fn spi5sec(&self) -> SPI5SEC_R {
        SPI5SEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - secure access mode for LPUART"]
    #[inline(always)]
    pub fn lpuart1sec(&self) -> LPUART1SEC_R {
        LPUART1SEC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - secure access mode for I2C3"]
    #[inline(always)]
    pub fn i2c3sec(&self) -> I2C3SEC_R {
        I2C3SEC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - secure access mode for I2C4"]
    #[inline(always)]
    pub fn i2c4sec(&self) -> I2C4SEC_R {
        I2C4SEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - secure access mode for LPTIM1"]
    #[inline(always)]
    pub fn lptim1sec(&self) -> LPTIM1SEC_R {
        LPTIM1SEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - secure access mode for LPTIM3"]
    #[inline(always)]
    pub fn lptim3sec(&self) -> LPTIM3SEC_R {
        LPTIM3SEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - secure access mode for LPTIM4"]
    #[inline(always)]
    pub fn lptim4sec(&self) -> LPTIM4SEC_R {
        LPTIM4SEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - secure access mode for LPTIM5"]
    #[inline(always)]
    pub fn lptim5sec(&self) -> LPTIM5SEC_R {
        LPTIM5SEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure access mode for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1sec(&mut self) -> FDCAN1SEC_W<0> {
        FDCAN1SEC_W::new(self)
    }
    #[doc = "Bit 1 - secure access mode for FDCAN2"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan2sec(&mut self) -> FDCAN2SEC_W<1> {
        FDCAN2SEC_W::new(self)
    }
    #[doc = "Bit 2 - secure access mode for UCPD"]
    #[inline(always)]
    #[must_use]
    pub fn ucpdsec(&mut self) -> UCPDSEC_W<2> {
        UCPDSEC_W::new(self)
    }
    #[doc = "Bit 8 - secure access mode for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sec(&mut self) -> TIM1SEC_W<8> {
        TIM1SEC_W::new(self)
    }
    #[doc = "Bit 9 - secure access mode for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1sec(&mut self) -> SPI1SEC_W<9> {
        SPI1SEC_W::new(self)
    }
    #[doc = "Bit 10 - secure access mode for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn tim8sec(&mut self) -> TIM8SEC_W<10> {
        TIM8SEC_W::new(self)
    }
    #[doc = "Bit 11 - secure access mode for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sec(&mut self) -> USART1SEC_W<11> {
        USART1SEC_W::new(self)
    }
    #[doc = "Bit 12 - secure access mode for TIM15"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sec(&mut self) -> TIM15SEC_W<12> {
        TIM15SEC_W::new(self)
    }
    #[doc = "Bit 13 - secure access mode for TIM16"]
    #[inline(always)]
    #[must_use]
    pub fn tim16sec(&mut self) -> TIM16SEC_W<13> {
        TIM16SEC_W::new(self)
    }
    #[doc = "Bit 14 - secure access mode for TIM17"]
    #[inline(always)]
    #[must_use]
    pub fn tim17sec(&mut self) -> TIM17SEC_W<14> {
        TIM17SEC_W::new(self)
    }
    #[doc = "Bit 15 - secure access mode for SPI4"]
    #[inline(always)]
    #[must_use]
    pub fn spi4sec(&mut self) -> SPI4SEC_W<15> {
        SPI4SEC_W::new(self)
    }
    #[doc = "Bit 16 - secure access mode for SPI6"]
    #[inline(always)]
    #[must_use]
    pub fn spi6sec(&mut self) -> SPI6SEC_W<16> {
        SPI6SEC_W::new(self)
    }
    #[doc = "Bit 17 - secure access mode for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sec(&mut self) -> SAI1SEC_W<17> {
        SAI1SEC_W::new(self)
    }
    #[doc = "Bit 18 - secure access mode for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sec(&mut self) -> SAI2SEC_W<18> {
        SAI2SEC_W::new(self)
    }
    #[doc = "Bit 19 - secure access mode for USB"]
    #[inline(always)]
    #[must_use]
    pub fn usbsec(&mut self) -> USBSEC_W<19> {
        USBSEC_W::new(self)
    }
    #[doc = "Bit 24 - secure access mode for SPI5"]
    #[inline(always)]
    #[must_use]
    pub fn spi5sec(&mut self) -> SPI5SEC_W<24> {
        SPI5SEC_W::new(self)
    }
    #[doc = "Bit 25 - secure access mode for LPUART"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sec(&mut self) -> LPUART1SEC_W<25> {
        LPUART1SEC_W::new(self)
    }
    #[doc = "Bit 26 - secure access mode for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sec(&mut self) -> I2C3SEC_W<26> {
        I2C3SEC_W::new(self)
    }
    #[doc = "Bit 27 - secure access mode for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sec(&mut self) -> I2C4SEC_W<27> {
        I2C4SEC_W::new(self)
    }
    #[doc = "Bit 28 - secure access mode for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sec(&mut self) -> LPTIM1SEC_W<28> {
        LPTIM1SEC_W::new(self)
    }
    #[doc = "Bit 29 - secure access mode for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sec(&mut self) -> LPTIM3SEC_W<29> {
        LPTIM3SEC_W::new(self)
    }
    #[doc = "Bit 30 - secure access mode for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4sec(&mut self) -> LPTIM4SEC_W<30> {
        LPTIM4SEC_W::new(self)
    }
    #[doc = "Bit 31 - secure access mode for LPTIM5"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5sec(&mut self) -> LPTIM5SEC_W<31> {
        LPTIM5SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZSC secure configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzsc_seccfgr2](index.html) module"]
pub struct GTZC1_TZSC_SECCFGR2_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_SECCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzsc_seccfgr2::R](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_SECCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzsc_seccfgr2::W](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_SECCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_SECCFGR2 to value 0"]
impl crate::Resettable for GTZC1_TZSC_SECCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
