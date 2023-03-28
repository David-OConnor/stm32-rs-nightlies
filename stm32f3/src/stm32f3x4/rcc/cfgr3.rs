#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART1SW` reader - USART1 clock source selection"]
pub type USART1SW_R = crate::FieldReader<u8, USART1SW_A>;
#[doc = "USART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SW_A {
    #[doc = "0: PCLK selected as USART clock source"]
    Pclk = 0,
    #[doc = "1: SYSCLK selected as USART clock source"]
    Sysclk = 1,
    #[doc = "2: LSE selected as USART clock source"]
    Lse = 2,
    #[doc = "3: HSI selected as USART clock source"]
    Hsi = 3,
}
impl From<USART1SW_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SW_A) -> Self {
        variant as _
    }
}
impl USART1SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SW_A {
        match self.bits {
            0 => USART1SW_A::Pclk,
            1 => USART1SW_A::Sysclk,
            2 => USART1SW_A::Lse,
            3 => USART1SW_A::Hsi,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SW_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SW_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SW_A::Lse
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SW_A::Hsi
    }
}
#[doc = "Field `USART1SW` writer - USART1 clock source selection"]
pub type USART1SW_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR3_SPEC, u8, USART1SW_A, 2, O>;
impl<'a, const O: u8> USART1SW_W<'a, O> {
    #[doc = "PCLK selected as USART clock source"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART1SW_A::Pclk)
    }
    #[doc = "SYSCLK selected as USART clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SW_A::Sysclk)
    }
    #[doc = "LSE selected as USART clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SW_A::Lse)
    }
    #[doc = "HSI selected as USART clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART1SW_A::Hsi)
    }
}
#[doc = "Field `I2C1SW` reader - I2C1 clock source selection"]
pub type I2C1SW_R = crate::BitReader<I2C1SW_A>;
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SW_A {
    #[doc = "0: HSI clock selected as I2C clock source"]
    Hsi = 0,
    #[doc = "1: SYSCLK clock selected as I2C clock source"]
    Sysclk = 1,
}
impl From<I2C1SW_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1SW_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1SW_A {
        match self.bits {
            false => I2C1SW_A::Hsi,
            true => I2C1SW_A::Sysclk,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SW_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SW_A::Sysclk
    }
}
#[doc = "Field `I2C1SW` writer - I2C1 clock source selection"]
pub type I2C1SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, I2C1SW_A, O>;
impl<'a, const O: u8> I2C1SW_W<'a, O> {
    #[doc = "HSI clock selected as I2C clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SW_A::Hsi)
    }
    #[doc = "SYSCLK clock selected as I2C clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SW_A::Sysclk)
    }
}
#[doc = "Field `TIM1SW` reader - Timer1 clock source selection"]
pub type TIM1SW_R = crate::BitReader<TIM1SW_A>;
#[doc = "Timer1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SW_A {
    #[doc = "0: PCLK2 clock (doubled frequency when prescaled)"]
    Pclk2 = 0,
    #[doc = "1: PLL vco output (running up to 144 MHz)"]
    Pll = 1,
}
impl From<TIM1SW_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1SW_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1SW_A {
        match self.bits {
            false => TIM1SW_A::Pclk2,
            true => TIM1SW_A::Pll,
        }
    }
    #[doc = "Checks if the value of the field is `Pclk2`"]
    #[inline(always)]
    pub fn is_pclk2(&self) -> bool {
        *self == TIM1SW_A::Pclk2
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == TIM1SW_A::Pll
    }
}
#[doc = "Field `TIM1SW` writer - Timer1 clock source selection"]
pub type TIM1SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, TIM1SW_A, O>;
impl<'a, const O: u8> TIM1SW_W<'a, O> {
    #[doc = "PCLK2 clock (doubled frequency when prescaled)"]
    #[inline(always)]
    pub fn pclk2(self) -> &'a mut W {
        self.variant(TIM1SW_A::Pclk2)
    }
    #[doc = "PLL vco output (running up to 144 MHz)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(TIM1SW_A::Pll)
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&self) -> USART1SW_R {
        USART1SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2C1SW_R {
        I2C1SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer1 clock source selection"]
    #[inline(always)]
    pub fn tim1sw(&self) -> TIM1SW_R {
        TIM1SW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sw(&mut self) -> USART1SW_W<0> {
        USART1SW_W::new(self)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sw(&mut self) -> I2C1SW_W<4> {
        I2C1SW_W::new(self)
    }
    #[doc = "Bit 8 - Timer1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sw(&mut self) -> TIM1SW_W<8> {
        TIM1SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
