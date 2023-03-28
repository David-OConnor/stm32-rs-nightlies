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
    #[doc = "1: Reset the selected module"]
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
    pub fn variant(&self) -> Option<LPUART1RST_A> {
        match self.bits {
            true => Some(LPUART1RST_A::Reset),
            _ => None,
        }
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
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPUART1RST_A::Reset)
    }
}
#[doc = "Field `I2C4RST` reader - I2C4 reset"]
pub use LPUART1RST_R as I2C4RST_R;
#[doc = "Field `UCPD1RST` reader - UCPD1 reset"]
pub use LPUART1RST_R as UCPD1RST_R;
#[doc = "Field `I2C4RST` writer - I2C4 reset"]
pub use LPUART1RST_W as I2C4RST_W;
#[doc = "Field `UCPD1RST` writer - UCPD1 reset"]
pub use LPUART1RST_W as UCPD1RST_W;
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
    #[doc = "Bit 8 - UCPD1 reset"]
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bit 8 - UCPD1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<8> {
        UCPD1RST_W::new(self)
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
