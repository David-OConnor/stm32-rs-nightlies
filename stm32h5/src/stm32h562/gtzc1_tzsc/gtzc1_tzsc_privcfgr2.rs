#[doc = "Register `GTZC1_TZSC_PRIVCFGR2` reader"]
pub struct R(crate::R<GTZC1_TZSC_PRIVCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_PRIVCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_PRIVCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_PRIVCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZSC_PRIVCFGR2` writer"]
pub struct W(crate::W<GTZC1_TZSC_PRIVCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_PRIVCFGR2_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_PRIVCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_PRIVCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDCAN1PRIV` reader - privileged access mode for FDCAN1"]
pub type FDCAN1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN1PRIV` writer - privileged access mode for FDCAN1"]
pub type FDCAN1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `FDCAN2PRIV` reader - privileged access mode for FDCAN2"]
pub type FDCAN2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN2PRIV` writer - privileged access mode for FDCAN2"]
pub type FDCAN2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `UCPDPRIV` reader - privileged access mode for UCPD"]
pub type UCPDPRIV_R = crate::BitReader<bool>;
#[doc = "Field `UCPDPRIV` writer - privileged access mode for UCPD"]
pub type UCPDPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM1PRIV` reader - privileged access mode for TIM1"]
pub type TIM1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM1PRIV` writer - privileged access mode for TIM1"]
pub type TIM1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `SPI1PRIV` reader - privileged access mode for SPI1"]
pub type SPI1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI1PRIV` writer - privileged access mode for SPI1"]
pub type SPI1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM8PRIV` reader - privileged access mode for TIM8"]
pub type TIM8PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM8PRIV` writer - privileged access mode for TIM8"]
pub type TIM8PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `USART1PRIV` reader - privileged access mode for USART1"]
pub type USART1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART1PRIV` writer - privileged access mode for USART1"]
pub type USART1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM15PRIV` reader - privileged access mode for TIM15"]
pub type TIM15PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM15PRIV` writer - privileged access mode for TIM15"]
pub type TIM15PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM16PRIV` reader - privileged access mode for TIM16"]
pub type TIM16PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM16PRIV` writer - privileged access mode for TIM16"]
pub type TIM16PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `TIM17PRIV` reader - privileged access mode for TIM17"]
pub type TIM17PRIV_R = crate::BitReader<bool>;
#[doc = "Field `TIM17PRIV` writer - privileged access mode for TIM17"]
pub type TIM17PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `SPI4PRIV` reader - privileged access mode for SPI4"]
pub type SPI4PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI4PRIV` writer - privileged access mode for SPI4"]
pub type SPI4PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `SPI6PRIV` reader - privileged access mode for SPI6"]
pub type SPI6PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI6PRIV` writer - privileged access mode for SPI6"]
pub type SPI6PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `SAI1PRIV` reader - privileged access mode for SAI1"]
pub type SAI1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SAI1PRIV` writer - privileged access mode for SAI1"]
pub type SAI1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `SAI2PRIV` reader - privileged access mode for SAI2"]
pub type SAI2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SAI2PRIV` writer - privileged access mode for SAI2"]
pub type SAI2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `USBPRIV` reader - privileged access mode for USB"]
pub type USBPRIV_R = crate::BitReader<bool>;
#[doc = "Field `USBPRIV` writer - privileged access mode for USB"]
pub type USBPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `SPI5PRIV` reader - privileged access mode for SPI5"]
pub type SPI5PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SPI5PRIV` writer - privileged access mode for SPI5"]
pub type SPI5PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `LPUART1PRIV` reader - privileged access mode for LPUART"]
pub type LPUART1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1PRIV` writer - privileged access mode for LPUART"]
pub type LPUART1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `I2C3PRIV` reader - privileged access mode for I2C3"]
pub type I2C3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I2C3PRIV` writer - privileged access mode for I2C3"]
pub type I2C3PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `I2C4PRIV` reader - privileged access mode for I2C4"]
pub type I2C4PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I2C4PRIV` writer - privileged access mode for I2C4"]
pub type I2C4PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM3PRIV` reader - privileged access mode for LPTIM3"]
pub type LPTIM3PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3PRIV` writer - privileged access mode for LPTIM3"]
pub type LPTIM3PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM4PRIV` reader - privileged access mode for LPTIM4"]
pub type LPTIM4PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM4PRIV` writer - privileged access mode for LPTIM4"]
pub type LPTIM4PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM5PRIV` reader - privileged access mode for LPTIM5"]
pub type LPTIM5PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM5PRIV` writer - privileged access mode for LPTIM5"]
pub type LPTIM5PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - privileged access mode for FDCAN1"]
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for FDCAN2"]
    #[inline(always)]
    pub fn fdcan2priv(&self) -> FDCAN2PRIV_R {
        FDCAN2PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - privileged access mode for UCPD"]
    #[inline(always)]
    pub fn ucpdpriv(&self) -> UCPDPRIV_R {
        UCPDPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for TIM1"]
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for SPI1"]
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - privileged access mode for TIM8"]
    #[inline(always)]
    pub fn tim8priv(&self) -> TIM8PRIV_R {
        TIM8PRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for USART1"]
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for TIM15"]
    #[inline(always)]
    pub fn tim15priv(&self) -> TIM15PRIV_R {
        TIM15PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - privileged access mode for TIM16"]
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for TIM17"]
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - privileged access mode for SPI4"]
    #[inline(always)]
    pub fn spi4priv(&self) -> SPI4PRIV_R {
        SPI4PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - privileged access mode for SPI6"]
    #[inline(always)]
    pub fn spi6priv(&self) -> SPI6PRIV_R {
        SPI6PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for SAI1"]
    #[inline(always)]
    pub fn sai1priv(&self) -> SAI1PRIV_R {
        SAI1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for SAI2"]
    #[inline(always)]
    pub fn sai2priv(&self) -> SAI2PRIV_R {
        SAI2PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - privileged access mode for USB"]
    #[inline(always)]
    pub fn usbpriv(&self) -> USBPRIV_R {
        USBPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - privileged access mode for SPI5"]
    #[inline(always)]
    pub fn spi5priv(&self) -> SPI5PRIV_R {
        SPI5PRIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - privileged access mode for LPUART"]
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - privileged access mode for I2C3"]
    #[inline(always)]
    pub fn i2c3priv(&self) -> I2C3PRIV_R {
        I2C3PRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - privileged access mode for I2C4"]
    #[inline(always)]
    pub fn i2c4priv(&self) -> I2C4PRIV_R {
        I2C4PRIV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - privileged access mode for LPTIM1"]
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - privileged access mode for LPTIM3"]
    #[inline(always)]
    pub fn lptim3priv(&self) -> LPTIM3PRIV_R {
        LPTIM3PRIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - privileged access mode for LPTIM4"]
    #[inline(always)]
    pub fn lptim4priv(&self) -> LPTIM4PRIV_R {
        LPTIM4PRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - privileged access mode for LPTIM5"]
    #[inline(always)]
    pub fn lptim5priv(&self) -> LPTIM5PRIV_R {
        LPTIM5PRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W<0> {
        FDCAN1PRIV_W::new(self)
    }
    #[doc = "Bit 1 - privileged access mode for FDCAN2"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan2priv(&mut self) -> FDCAN2PRIV_W<1> {
        FDCAN2PRIV_W::new(self)
    }
    #[doc = "Bit 2 - privileged access mode for UCPD"]
    #[inline(always)]
    #[must_use]
    pub fn ucpdpriv(&mut self) -> UCPDPRIV_W<2> {
        UCPDPRIV_W::new(self)
    }
    #[doc = "Bit 8 - privileged access mode for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W<8> {
        TIM1PRIV_W::new(self)
    }
    #[doc = "Bit 9 - privileged access mode for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W<9> {
        SPI1PRIV_W::new(self)
    }
    #[doc = "Bit 10 - privileged access mode for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn tim8priv(&mut self) -> TIM8PRIV_W<10> {
        TIM8PRIV_W::new(self)
    }
    #[doc = "Bit 11 - privileged access mode for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<11> {
        USART1PRIV_W::new(self)
    }
    #[doc = "Bit 12 - privileged access mode for TIM15"]
    #[inline(always)]
    #[must_use]
    pub fn tim15priv(&mut self) -> TIM15PRIV_W<12> {
        TIM15PRIV_W::new(self)
    }
    #[doc = "Bit 13 - privileged access mode for TIM16"]
    #[inline(always)]
    #[must_use]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W<13> {
        TIM16PRIV_W::new(self)
    }
    #[doc = "Bit 14 - privileged access mode for TIM17"]
    #[inline(always)]
    #[must_use]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W<14> {
        TIM17PRIV_W::new(self)
    }
    #[doc = "Bit 15 - privileged access mode for SPI4"]
    #[inline(always)]
    #[must_use]
    pub fn spi4priv(&mut self) -> SPI4PRIV_W<15> {
        SPI4PRIV_W::new(self)
    }
    #[doc = "Bit 16 - privileged access mode for SPI6"]
    #[inline(always)]
    #[must_use]
    pub fn spi6priv(&mut self) -> SPI6PRIV_W<16> {
        SPI6PRIV_W::new(self)
    }
    #[doc = "Bit 17 - privileged access mode for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn sai1priv(&mut self) -> SAI1PRIV_W<17> {
        SAI1PRIV_W::new(self)
    }
    #[doc = "Bit 18 - privileged access mode for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn sai2priv(&mut self) -> SAI2PRIV_W<18> {
        SAI2PRIV_W::new(self)
    }
    #[doc = "Bit 19 - privileged access mode for USB"]
    #[inline(always)]
    #[must_use]
    pub fn usbpriv(&mut self) -> USBPRIV_W<19> {
        USBPRIV_W::new(self)
    }
    #[doc = "Bit 24 - privileged access mode for SPI5"]
    #[inline(always)]
    #[must_use]
    pub fn spi5priv(&mut self) -> SPI5PRIV_W<24> {
        SPI5PRIV_W::new(self)
    }
    #[doc = "Bit 25 - privileged access mode for LPUART"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<25> {
        LPUART1PRIV_W::new(self)
    }
    #[doc = "Bit 26 - privileged access mode for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3priv(&mut self) -> I2C3PRIV_W<26> {
        I2C3PRIV_W::new(self)
    }
    #[doc = "Bit 27 - privileged access mode for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4priv(&mut self) -> I2C4PRIV_W<27> {
        I2C4PRIV_W::new(self)
    }
    #[doc = "Bit 28 - privileged access mode for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<28> {
        LPTIM1PRIV_W::new(self)
    }
    #[doc = "Bit 29 - privileged access mode for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3priv(&mut self) -> LPTIM3PRIV_W<29> {
        LPTIM3PRIV_W::new(self)
    }
    #[doc = "Bit 30 - privileged access mode for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4priv(&mut self) -> LPTIM4PRIV_W<30> {
        LPTIM4PRIV_W::new(self)
    }
    #[doc = "Bit 31 - privileged access mode for LPTIM5"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5priv(&mut self) -> LPTIM5PRIV_W<31> {
        LPTIM5PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzsc_privcfgr2](index.html) module"]
pub struct GTZC1_TZSC_PRIVCFGR2_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_PRIVCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzsc_privcfgr2::R](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_PRIVCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzsc_privcfgr2::W](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_PRIVCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_PRIVCFGR2 to value 0"]
impl crate::Resettable for GTZC1_TZSC_PRIVCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
