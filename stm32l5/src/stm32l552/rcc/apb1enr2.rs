#[doc = "Register `APB1ENR2` reader"]
pub struct R(crate::R<APB1ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR2` writer"]
pub struct W(crate::W<APB1ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR2_SPEC>;
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
impl From<crate::W<APB1ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1EN` reader - Low power UART 1 clock enable"]
pub type LPUART1EN_R = crate::BitReader<LPUART1EN_A>;
#[doc = "Low power UART 1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<LPUART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1EN_A {
        match self.bits {
            false => LPUART1EN_A::Disabled,
            true => LPUART1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1EN_A::Enabled
    }
}
#[doc = "Field `LPUART1EN` writer - Low power UART 1 clock enable"]
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR2_SPEC, LPUART1EN_A, O>;
impl<'a, const O: u8> LPUART1EN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::Enabled)
    }
}
#[doc = "Field `I2C4EN` reader - I2C4 clock enable"]
pub use LPUART1EN_R as I2C4EN_R;
#[doc = "Field `LPTIM2EN` reader - LPTIM2EN"]
pub use LPUART1EN_R as LPTIM2EN_R;
#[doc = "Field `LPTIM3EN` reader - LPTIM3EN"]
pub use LPUART1EN_R as LPTIM3EN_R;
#[doc = "Field `FDCAN1EN` reader - FDCAN1EN"]
pub use LPUART1EN_R as FDCAN1EN_R;
#[doc = "Field `USBFSEN` reader - USBFSEN"]
pub use LPUART1EN_R as USBFSEN_R;
#[doc = "Field `UCPD1EN` reader - UCPD1EN"]
pub use LPUART1EN_R as UCPD1EN_R;
#[doc = "Field `I2C4EN` writer - I2C4 clock enable"]
pub use LPUART1EN_W as I2C4EN_W;
#[doc = "Field `LPTIM2EN` writer - LPTIM2EN"]
pub use LPUART1EN_W as LPTIM2EN_W;
#[doc = "Field `LPTIM3EN` writer - LPTIM3EN"]
pub use LPUART1EN_W as LPTIM3EN_W;
#[doc = "Field `FDCAN1EN` writer - FDCAN1EN"]
pub use LPUART1EN_W as FDCAN1EN_W;
#[doc = "Field `USBFSEN` writer - USBFSEN"]
pub use LPUART1EN_W as USBFSEN_W;
#[doc = "Field `UCPD1EN` writer - UCPD1EN"]
pub use LPUART1EN_W as UCPD1EN_W;
impl R {
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1EN"]
    #[inline(always)]
    pub fn fdcan1en(&self) -> FDCAN1EN_R {
        FDCAN1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 21 - USBFSEN"]
    #[inline(always)]
    pub fn usbfsen(&self) -> USBFSEN_R {
        USBFSEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - UCPD1EN"]
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<0> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<1> {
        I2C4EN_W::new(self)
    }
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<5> {
        LPTIM2EN_W::new(self)
    }
    #[doc = "Bit 6 - LPTIM3EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<6> {
        LPTIM3EN_W::new(self)
    }
    #[doc = "Bit 9 - FDCAN1EN"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1en(&mut self) -> FDCAN1EN_W<9> {
        FDCAN1EN_W::new(self)
    }
    #[doc = "Bit 21 - USBFSEN"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsen(&mut self) -> USBFSEN_W<21> {
        USBFSEN_W::new(self)
    }
    #[doc = "Bit 23 - UCPD1EN"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<23> {
        UCPD1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr2](index.html) module"]
pub struct APB1ENR2_SPEC;
impl crate::RegisterSpec for APB1ENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr2::R](R) reader structure"]
impl crate::Readable for APB1ENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr2::W](W) writer structure"]
impl crate::Writable for APB1ENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1ENR2 to value 0"]
impl crate::Resettable for APB1ENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
