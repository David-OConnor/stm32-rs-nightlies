#[doc = "Register `APB1SMENR2` reader"]
pub struct R(crate::R<APB1SMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1SMENR2` writer"]
pub struct W(crate::W<APB1SMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR2_SPEC>;
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
impl From<crate::W<APB1SMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
pub type LPUART1SMEN_R = crate::BitReader<LPUART1SMEN_A>;
#[doc = "Low power UART 1 clock enable during CPU1 Csleep and CStop modes.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1SMEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<LPUART1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1SMEN_A {
        match self.bits {
            false => LPUART1SMEN_A::Disabled,
            true => LPUART1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1SMEN_A::Enabled
    }
}
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
pub type LPUART1SMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1SMENR2_SPEC, LPUART1SMEN_A, O>;
impl<'a, const O: u8> LPUART1SMEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1SMEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1SMEN_A::Enabled)
    }
}
#[doc = "Field `LPTIM2SMEN` reader - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
pub use LPUART1SMEN_R as LPTIM2SMEN_R;
#[doc = "Field `LPTIM3SMEN` reader - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
pub use LPUART1SMEN_R as LPTIM3SMEN_R;
#[doc = "Field `LPTIM2SMEN` writer - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
pub use LPUART1SMEN_W as LPTIM2SMEN_W;
#[doc = "Field `LPTIM3SMEN` writer - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
pub use LPUART1SMEN_W as LPTIM3SMEN_W;
impl R {
    #[doc = "Bit 0 - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<0> {
        LPUART1SMEN_W::new(self)
    }
    #[doc = "Bit 5 - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<5> {
        LPTIM2SMEN_W::new(self)
    }
    #[doc = "Bit 6 - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W<6> {
        LPTIM3SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep mode register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr2](index.html) module"]
pub struct APB1SMENR2_SPEC;
impl crate::RegisterSpec for APB1SMENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1smenr2::R](R) reader structure"]
impl crate::Readable for APB1SMENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1smenr2::W](W) writer structure"]
impl crate::Writable for APB1SMENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1SMENR2 to value 0x61"]
impl crate::Resettable for APB1SMENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x61;
}
