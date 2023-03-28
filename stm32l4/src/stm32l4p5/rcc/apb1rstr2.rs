#[doc = "Register `APB1RSTR2` reader"]
pub struct R(crate::R<APB1RSTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1RSTR2` writer"]
pub struct W(crate::W<APB1RSTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR2_SPEC>;
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
impl From<crate::W<APB1RSTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1RST` reader - Low-power UART 1 reset"]
pub type LPUART1RST_R = crate::BitReader<LPUART1RST_A>;
#[doc = "Low-power UART 1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset LPUART1"]
    Reset = 1,
}
impl From<LPUART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1RST_A {
        match self.bits {
            false => LPUART1RST_A::NoEffect,
            true => LPUART1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LPUART1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPUART1RST_A::Reset
    }
}
#[doc = "Field `LPUART1RST` writer - Low-power UART 1 reset"]
pub type LPUART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR2_SPEC, LPUART1RST_A, O>;
impl<'a, const O: u8> LPUART1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(LPUART1RST_A::NoEffect)
    }
    #[doc = "Reset LPUART1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPUART1RST_A::Reset)
    }
}
#[doc = "Field `I2C4RST` reader - I2C4 reset"]
pub type I2C4RST_R = crate::BitReader<I2C4RST_A>;
#[doc = "I2C4 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset I2C4"]
    Reset = 1,
}
impl From<I2C4RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C4RST_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C4RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C4RST_A {
        match self.bits {
            false => I2C4RST_A::NoEffect,
            true => I2C4RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == I2C4RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C4RST_A::Reset
    }
}
#[doc = "Field `I2C4RST` writer - I2C4 reset"]
pub type I2C4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR2_SPEC, I2C4RST_A, O>;
impl<'a, const O: u8> I2C4RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(I2C4RST_A::NoEffect)
    }
    #[doc = "Reset I2C4"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C4RST_A::Reset)
    }
}
#[doc = "Field `LPTIM2RST` reader - Low-power timer 2 reset"]
pub type LPTIM2RST_R = crate::BitReader<LPTIM2RST_A>;
#[doc = "Low-power timer 2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset LPTIM2"]
    Reset = 1,
}
impl From<LPTIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM2RST_A {
        match self.bits {
            false => LPTIM2RST_A::NoEffect,
            true => LPTIM2RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LPTIM2RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPTIM2RST_A::Reset
    }
}
#[doc = "Field `LPTIM2RST` writer - Low-power timer 2 reset"]
pub type LPTIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR2_SPEC, LPTIM2RST_A, O>;
impl<'a, const O: u8> LPTIM2RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(LPTIM2RST_A::NoEffect)
    }
    #[doc = "Reset LPTIM2"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPTIM2RST_A::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<0> {
        LPUART1RST_W::new(self)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<1> {
        I2C4RST_W::new(self)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<5> {
        LPTIM2RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral reset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr2](index.html) module"]
pub struct APB1RSTR2_SPEC;
impl crate::RegisterSpec for APB1RSTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1rstr2::R](R) reader structure"]
impl crate::Readable for APB1RSTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1rstr2::W](W) writer structure"]
impl crate::Writable for APB1RSTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1RSTR2 to value 0"]
impl crate::Resettable for APB1RSTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
