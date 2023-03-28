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
#[doc = "Field `OPAMPPRIV` reader - privileged access mode for OPAMP"]
pub type OPAMPPRIV_R = crate::BitReader<bool>;
#[doc = "Field `OPAMPPRIV` writer - privileged access mode for OPAMP"]
pub type OPAMPPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `COMPPRIV` reader - privileged access mode for COMP"]
pub type COMPPRIV_R = crate::BitReader<bool>;
#[doc = "Field `COMPPRIV` writer - privileged access mode for COMP"]
pub type COMPPRIV_W<'a, const O: u8> =
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
#[doc = "Field `USART1PRIV` reader - privileged access mode for USART1"]
pub type USART1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `USART1PRIV` writer - privileged access mode for USART1"]
pub type USART1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `USBFSPRIV` reader - privileged access mode for USBSF"]
pub type USBFSPRIV_R = crate::BitReader<bool>;
#[doc = "Field `USBFSPRIV` writer - privileged access mode for USBSF"]
pub type USBFSPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `LPUART1PRIV` reader - privileged access mode for LPUART"]
pub type LPUART1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1PRIV` writer - privileged access mode for LPUART"]
pub type LPUART1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - privileged access mode for FDCAN1"]
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - privileged access mode for OPAMP"]
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for COMP"]
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 11 - privileged access mode for USART1"]
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 19 - privileged access mode for USBSF"]
    #[inline(always)]
    pub fn usbfspriv(&self) -> USBFSPRIV_R {
        USBFSPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - privileged access mode for LPUART"]
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - privileged access mode for LPTIM1"]
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W<0> {
        FDCAN1PRIV_W::new(self)
    }
    #[doc = "Bit 3 - privileged access mode for OPAMP"]
    #[inline(always)]
    #[must_use]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W<3> {
        OPAMPPRIV_W::new(self)
    }
    #[doc = "Bit 4 - privileged access mode for COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comppriv(&mut self) -> COMPPRIV_W<4> {
        COMPPRIV_W::new(self)
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
    #[doc = "Bit 11 - privileged access mode for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<11> {
        USART1PRIV_W::new(self)
    }
    #[doc = "Bit 19 - privileged access mode for USBSF"]
    #[inline(always)]
    #[must_use]
    pub fn usbfspriv(&mut self) -> USBFSPRIV_W<19> {
        USBFSPRIV_W::new(self)
    }
    #[doc = "Bit 25 - privileged access mode for LPUART"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<25> {
        LPUART1PRIV_W::new(self)
    }
    #[doc = "Bit 28 - privileged access mode for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<28> {
        LPTIM1PRIV_W::new(self)
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
