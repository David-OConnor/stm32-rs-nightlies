#[doc = "Register `CCIPR` reader"]
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR` writer"]
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type USART1SEL_R = crate::FieldReader<u8, USART1SEL_A>;
#[doc = "USART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
impl USART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SEL_A {
        match self.bits {
            0 => USART1SEL_A::Pclk,
            1 => USART1SEL_A::Sysclk,
            2 => USART1SEL_A::Hsi16,
            3 => USART1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART1SEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL_A::Lse
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type USART1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, USART1SEL_A, 2, O>;
impl<'a, const O: u8> USART1SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(USART1SEL_A::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SEL_A::Lse)
    }
}
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub use USART1SEL_R as USART2SEL_R;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub use USART1SEL_W as USART2SEL_W;
#[doc = "Field `SPI2S2SEL` reader - SPI2S2 I2S clock source selection"]
pub type SPI2S2SEL_R = crate::FieldReader<u8, SPI2S2SEL_A>;
#[doc = "SPI2S2 I2S clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI2S2SEL_A {
    #[doc = "1: PLLQ clock selected"]
    Pllq = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: External input I2S_CKIN selected"]
    I2s = 3,
}
impl From<SPI2S2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI2S2SEL_A) -> Self {
        variant as _
    }
}
impl SPI2S2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI2S2SEL_A> {
        match self.bits {
            1 => Some(SPI2S2SEL_A::Pllq),
            2 => Some(SPI2S2SEL_A::Hsi16),
            3 => Some(SPI2S2SEL_A::I2s),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pllq`"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == SPI2S2SEL_A::Pllq
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SPI2S2SEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `I2s`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == SPI2S2SEL_A::I2s
    }
}
#[doc = "Field `SPI2S2SEL` writer - SPI2S2 I2S clock source selection"]
pub type SPI2S2SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, SPI2S2SEL_A, 2, O>;
impl<'a, const O: u8> SPI2S2SEL_W<'a, O> {
    #[doc = "PLLQ clock selected"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(SPI2S2SEL_A::Pllq)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(SPI2S2SEL_A::Hsi16)
    }
    #[doc = "External input I2S_CKIN selected"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(SPI2S2SEL_A::I2s)
    }
}
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type LPUART1SEL_R = crate::FieldReader<u8, LPUART1SEL_A>;
#[doc = "LPUART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
}
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
impl LPUART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1SEL_A {
        match self.bits {
            0 => LPUART1SEL_A::Pclk,
            1 => LPUART1SEL_A::Sysclk,
            2 => LPUART1SEL_A::Hsi16,
            3 => LPUART1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPUART1SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == LPUART1SEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPUART1SEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL_A::Lse
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub type LPUART1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, LPUART1SEL_A, 2, O>;
impl<'a, const O: u8> LPUART1SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Lse)
    }
}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2C1SEL_R = crate::FieldReader<u8, I2C1SEL_A>;
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
impl I2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::Pclk),
            1 => Some(I2C1SEL_A::Sysclk),
            2 => Some(I2C1SEL_A::Hsi16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C1SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C1SEL_A::Hsi16
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, I2C1SEL_A, 2, O>;
impl<'a, const O: u8> I2C1SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Hsi16)
    }
}
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub use I2C1SEL_R as I2C2SEL_R;
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub use I2C1SEL_R as I2C3SEL_R;
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
pub use I2C1SEL_W as I2C2SEL_W;
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
pub use I2C1SEL_W as I2C3SEL_W;
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub type LPTIM1SEL_R = crate::FieldReader<u8, LPTIM1SEL_A>;
#[doc = "Low power timer 1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: LSI clock selected"]
    Lsi = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
impl LPTIM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::Pclk,
            1 => LPTIM1SEL_A::Lsi,
            2 => LPTIM1SEL_A::Hsi16,
            3 => LPTIM1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPTIM1SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::Lse
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection"]
pub type LPTIM1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, LPTIM1SEL_A, 2, O>;
impl<'a, const O: u8> LPTIM1SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Pclk)
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lsi)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lse)
    }
}
#[doc = "Field `LPTIM2SEL` reader - Low power timer 2 clock source selection"]
pub use LPTIM1SEL_R as LPTIM2SEL_R;
#[doc = "Field `LPTIM3SEL` reader - Low power timer 3 clock source selection"]
pub use LPTIM1SEL_R as LPTIM3SEL_R;
#[doc = "Field `LPTIM2SEL` writer - Low power timer 2 clock source selection"]
pub use LPTIM1SEL_W as LPTIM2SEL_W;
#[doc = "Field `LPTIM3SEL` writer - Low power timer 3 clock source selection"]
pub use LPTIM1SEL_W as LPTIM3SEL_W;
#[doc = "Field `ADCSEL` reader - ADC clock source selection"]
pub type ADCSEL_R = crate::FieldReader<u8, ADCSEL_A>;
#[doc = "ADC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL_A {
    #[doc = "0: No clock selected"]
    NoClock = 0,
    #[doc = "1: HSI16 clock selected"]
    Hsi16 = 1,
    #[doc = "2: PLLP clock selected"]
    Pllp = 2,
    #[doc = "3: SYSCLK clock selected"]
    Sysclk = 3,
}
impl From<ADCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSEL_A) -> Self {
        variant as _
    }
}
impl ADCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSEL_A {
        match self.bits {
            0 => ADCSEL_A::NoClock,
            1 => ADCSEL_A::Hsi16,
            2 => ADCSEL_A::Pllp,
            3 => ADCSEL_A::Sysclk,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoClock`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ADCSEL_A::NoClock
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == ADCSEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Pllp`"]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == ADCSEL_A::Pllp
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == ADCSEL_A::Sysclk
    }
}
#[doc = "Field `ADCSEL` writer - ADC clock source selection"]
pub type ADCSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, ADCSEL_A, 2, O>;
impl<'a, const O: u8> ADCSEL_W<'a, O> {
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(ADCSEL_A::NoClock)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(ADCSEL_A::Hsi16)
    }
    #[doc = "PLLP clock selected"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(ADCSEL_A::Pllp)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(ADCSEL_A::Sysclk)
    }
}
#[doc = "Field `RNGSEL` reader - RNG clock source selection"]
pub type RNGSEL_R = crate::FieldReader<u8, RNGSEL_A>;
#[doc = "RNG clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL_A {
    #[doc = "0: PLLQ clock selected"]
    Pllq = 0,
    #[doc = "1: LSI clock selected"]
    Lsi = 1,
    #[doc = "2: LSE clock selected"]
    Lse = 2,
    #[doc = "3: MSI clock selected"]
    Msi = 3,
}
impl From<RNGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RNGSEL_A) -> Self {
        variant as _
    }
}
impl RNGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGSEL_A {
        match self.bits {
            0 => RNGSEL_A::Pllq,
            1 => RNGSEL_A::Lsi,
            2 => RNGSEL_A::Lse,
            3 => RNGSEL_A::Msi,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pllq`"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == RNGSEL_A::Pllq
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL_A::Lse
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == RNGSEL_A::Msi
    }
}
#[doc = "Field `RNGSEL` writer - RNG clock source selection"]
pub type RNGSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, RNGSEL_A, 2, O>;
impl<'a, const O: u8> RNGSEL_W<'a, O> {
    #[doc = "PLLQ clock selected"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(RNGSEL_A::Pllq)
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RNGSEL_A::Lsi)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RNGSEL_A::Lse)
    }
    #[doc = "MSI clock selected"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(RNGSEL_A::Msi)
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SPI2S2 I2S clock source selection"]
    #[inline(always)]
    pub fn spi2s2sel(&self) -> SPI2S2SEL_R {
        SPI2S2SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Low power timer 3 clock source selection"]
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - RNG clock source selection"]
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<2> {
        USART2SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - SPI2S2 I2S clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi2s2sel(&mut self) -> SPI2S2SEL_W<8> {
        SPI2S2SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<10> {
        LPUART1SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<12> {
        I2C1SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<14> {
        I2C2SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<16> {
        I2C3SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<18> {
        LPTIM1SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<20> {
        LPTIM2SEL_W::new(self)
    }
    #[doc = "Bits 22:23 - Low power timer 3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<22> {
        LPTIM3SEL_W::new(self)
    }
    #[doc = "Bits 28:29 - ADC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<28> {
        ADCSEL_W::new(self)
    }
    #[doc = "Bits 30:31 - RNG clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<30> {
        RNGSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](index.html) module"]
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr::R](R) reader structure"]
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr::W](W) writer structure"]
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
