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
#[doc = "Field `USBSW` reader - USB clock source selection"]
pub type USBSW_R = crate::BitReader<USBSW_A>;
#[doc = "USB clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSW_A {
    #[doc = "0: USB clock disabled"]
    Disabled = 0,
    #[doc = "1: PLL clock selected as USB clock source"]
    Pllclk = 1,
}
impl From<USBSW_A> for bool {
    #[inline(always)]
    fn from(variant: USBSW_A) -> Self {
        variant as u8 != 0
    }
}
impl USBSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSW_A {
        match self.bits {
            false => USBSW_A::Disabled,
            true => USBSW_A::Pllclk,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBSW_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Pllclk`"]
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        *self == USBSW_A::Pllclk
    }
}
#[doc = "Field `USBSW` writer - USB clock source selection"]
pub type USBSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, USBSW_A, O>;
impl<'a, const O: u8> USBSW_W<'a, O> {
    #[doc = "USB clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBSW_A::Disabled)
    }
    #[doc = "PLL clock selected as USB clock source"]
    #[inline(always)]
    pub fn pllclk(self) -> &'a mut W {
        self.variant(USBSW_A::Pllclk)
    }
}
#[doc = "Field `ADCSW` reader - ADCSW is deprecated. See ADC field in CFGR2 register."]
pub type ADCSW_R = crate::BitReader<bool>;
#[doc = "Field `ADCSW` writer - ADCSW is deprecated. See ADC field in CFGR2 register."]
pub type ADCSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, bool, O>;
#[doc = "Field `USART2SW` reader - USART2 clock source selection"]
pub use USART1SW_R as USART2SW_R;
#[doc = "Field `USART3SW` reader - USART3 clock source"]
pub use USART1SW_R as USART3SW_R;
#[doc = "Field `USART2SW` writer - USART2 clock source selection"]
pub use USART1SW_W as USART2SW_W;
#[doc = "Field `USART3SW` writer - USART3 clock source"]
pub use USART1SW_W as USART3SW_W;
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
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    pub fn usbsw(&self) -> USBSW_R {
        USBSW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    pub fn adcsw(&self) -> ADCSW_R {
        ADCSW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - USART3 clock source"]
    #[inline(always)]
    pub fn usart3sw(&self) -> USART3SW_R {
        USART3SW_R::new(((self.bits >> 18) & 3) as u8)
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
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbsw(&mut self) -> USBSW_W<7> {
        USBSW_W::new(self)
    }
    #[doc = "Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    #[must_use]
    pub fn adcsw(&mut self) -> ADCSW_W<8> {
        ADCSW_W::new(self)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sw(&mut self) -> USART2SW_W<16> {
        USART2SW_W::new(self)
    }
    #[doc = "Bits 18:19 - USART3 clock source"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sw(&mut self) -> USART3SW_W<18> {
        USART3SW_W::new(self)
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
