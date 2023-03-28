#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - System clock switch"]
pub type SW_R = crate::FieldReader<u8, SW_A>;
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "0: MSI oscillator used as system clock"]
    Hsi = 0,
    #[doc = "1: HSI oscillator used as system clock"]
    Msi = 1,
    #[doc = "2: HSE oscillator used as system clock"]
    Hse = 2,
    #[doc = "3: PLL used as system clock"]
    Pll = 3,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            0 => SW_A::Hsi,
            1 => SW_A::Msi,
            2 => SW_A::Hse,
            3 => SW_A::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SW_A::Msi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW_A::Pll
    }
}
#[doc = "Field `SW` writer - System clock switch"]
pub type SW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, SW_A, 2, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    #[doc = "MSI oscillator used as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::Hsi)
    }
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(SW_A::Msi)
    }
    #[doc = "HSE oscillator used as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::Hse)
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SW_A::Pll)
    }
}
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SWS_R = crate::FieldReader<u8, SWSR_A>;
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR_A {
    #[doc = "0: MSI oscillator used as system clock"]
    Hsi = 0,
    #[doc = "1: HSI oscillator used as system clock"]
    Msi = 1,
    #[doc = "2: HSE oscillator used as system clock"]
    Hse = 2,
    #[doc = "3: PLL used as system clock"]
    Pll = 3,
}
impl From<SWSR_A> for u8 {
    #[inline(always)]
    fn from(variant: SWSR_A) -> Self {
        variant as _
    }
}
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSR_A {
        match self.bits {
            0 => SWSR_A::Hsi,
            1 => SWSR_A::Msi,
            2 => SWSR_A::Hse,
            3 => SWSR_A::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SWSR_A::Msi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWSR_A::Pll
    }
}
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader<u8, HPRE_A>;
#[doc = "AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "0: system clock not divided"]
    Div1 = 0,
    #[doc = "8: system clock divided by 2"]
    Div2 = 8,
    #[doc = "9: system clock divided by 4"]
    Div4 = 9,
    #[doc = "10: system clock divided by 8"]
    Div8 = 10,
    #[doc = "11: system clock divided by 16"]
    Div16 = 11,
    #[doc = "12: system clock divided by 64"]
    Div64 = 12,
    #[doc = "13: system clock divided by 128"]
    Div128 = 13,
    #[doc = "14: system clock divided by 256"]
    Div256 = 14,
    #[doc = "15: system clock divided by 512"]
    Div512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::Div1),
            8 => Some(HPRE_A::Div2),
            9 => Some(HPRE_A::Div4),
            10 => Some(HPRE_A::Div8),
            11 => Some(HPRE_A::Div16),
            12 => Some(HPRE_A::Div64),
            13 => Some(HPRE_A::Div128),
            14 => Some(HPRE_A::Div256),
            15 => Some(HPRE_A::Div512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE_A::Div128
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE_A::Div256
    }
    #[doc = "Checks if the value of the field is `Div512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE_A::Div512
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, HPRE_A, 4, O>;
impl<'a, const O: u8> HPRE_W<'a, O> {
    #[doc = "system clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::Div1)
    }
    #[doc = "system clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::Div2)
    }
    #[doc = "system clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::Div4)
    }
    #[doc = "system clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::Div8)
    }
    #[doc = "system clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::Div16)
    }
    #[doc = "system clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::Div64)
    }
    #[doc = "system clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::Div128)
    }
    #[doc = "system clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::Div256)
    }
    #[doc = "system clock divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::Div512)
    }
}
#[doc = "Field `PPRE1` reader - APB low-speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<u8, PPRE1_A>;
#[doc = "APB low-speed prescaler (APB1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1_A {
    #[doc = "0: HCLK not divided"]
    Div1 = 0,
    #[doc = "4: HCLK divided by 2"]
    Div2 = 4,
    #[doc = "5: HCLK divided by 4"]
    Div4 = 5,
    #[doc = "6: HCLK divided by 8"]
    Div8 = 6,
    #[doc = "7: HCLK divided by 16"]
    Div16 = 7,
}
impl From<PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1_A) -> Self {
        variant as _
    }
}
impl PPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE1_A> {
        match self.bits {
            0 => Some(PPRE1_A::Div1),
            4 => Some(PPRE1_A::Div2),
            5 => Some(PPRE1_A::Div4),
            6 => Some(PPRE1_A::Div8),
            7 => Some(PPRE1_A::Div16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE1_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1_A::Div16
    }
}
#[doc = "Field `PPRE1` writer - APB low-speed prescaler (APB1)"]
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PPRE1_A, 3, O>;
impl<'a, const O: u8> PPRE1_W<'a, O> {
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1_A::Div1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1_A::Div2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1_A::Div4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1_A::Div8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1_A::Div16)
    }
}
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2)"]
pub use PPRE1_R as PPRE2_R;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2)"]
pub use PPRE1_W as PPRE2_W;
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PLLSRC_R = crate::BitReader<PLLSRC_A>;
#[doc = "PLL entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC_A {
    #[doc = "0: HSI selected as PLL input clock"]
    Hsi = 0,
    #[doc = "1: HSE selected as PLL input clock"]
    Hse = 1,
}
impl From<PLLSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            false => PLLSRC_A::Hsi,
            true => PLLSRC_A::Hse,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRC_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC_A::Hse
    }
}
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, PLLSRC_A, O>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    #[doc = "HSI selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hsi)
    }
    #[doc = "HSE selected as PLL input clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hse)
    }
}
#[doc = "Field `PLLMUL` reader - PLL multiplication factor"]
pub type PLLMUL_R = crate::FieldReader<u8, PLLMUL_A>;
#[doc = "PLL multiplication factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL_A {
    #[doc = "0: PLL clock entry x 3"]
    Mul3 = 0,
    #[doc = "1: PLL clock entry x 4"]
    Mul4 = 1,
    #[doc = "2: PLL clock entry x 6"]
    Mul6 = 2,
    #[doc = "3: PLL clock entry x 8"]
    Mul8 = 3,
    #[doc = "4: PLL clock entry x 12"]
    Mul12 = 4,
    #[doc = "5: PLL clock entry x 16"]
    Mul16 = 5,
    #[doc = "6: PLL clock entry x 24"]
    Mul24 = 6,
    #[doc = "7: PLL clock entry x 32"]
    Mul32 = 7,
    #[doc = "8: PLL clock entry x 48"]
    Mul48 = 8,
}
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL_A) -> Self {
        variant as _
    }
}
impl PLLMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLMUL_A> {
        match self.bits {
            0 => Some(PLLMUL_A::Mul3),
            1 => Some(PLLMUL_A::Mul4),
            2 => Some(PLLMUL_A::Mul6),
            3 => Some(PLLMUL_A::Mul8),
            4 => Some(PLLMUL_A::Mul12),
            5 => Some(PLLMUL_A::Mul16),
            6 => Some(PLLMUL_A::Mul24),
            7 => Some(PLLMUL_A::Mul32),
            8 => Some(PLLMUL_A::Mul48),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Mul3`"]
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMUL_A::Mul3
    }
    #[doc = "Checks if the value of the field is `Mul4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMUL_A::Mul4
    }
    #[doc = "Checks if the value of the field is `Mul6`"]
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMUL_A::Mul6
    }
    #[doc = "Checks if the value of the field is `Mul8`"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMUL_A::Mul8
    }
    #[doc = "Checks if the value of the field is `Mul12`"]
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMUL_A::Mul12
    }
    #[doc = "Checks if the value of the field is `Mul16`"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMUL_A::Mul16
    }
    #[doc = "Checks if the value of the field is `Mul24`"]
    #[inline(always)]
    pub fn is_mul24(&self) -> bool {
        *self == PLLMUL_A::Mul24
    }
    #[doc = "Checks if the value of the field is `Mul32`"]
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        *self == PLLMUL_A::Mul32
    }
    #[doc = "Checks if the value of the field is `Mul48`"]
    #[inline(always)]
    pub fn is_mul48(&self) -> bool {
        *self == PLLMUL_A::Mul48
    }
}
#[doc = "Field `PLLMUL` writer - PLL multiplication factor"]
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PLLMUL_A, 4, O>;
impl<'a, const O: u8> PLLMUL_W<'a, O> {
    #[doc = "PLL clock entry x 3"]
    #[inline(always)]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul3)
    }
    #[doc = "PLL clock entry x 4"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul4)
    }
    #[doc = "PLL clock entry x 6"]
    #[inline(always)]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul6)
    }
    #[doc = "PLL clock entry x 8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul8)
    }
    #[doc = "PLL clock entry x 12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul12)
    }
    #[doc = "PLL clock entry x 16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul16)
    }
    #[doc = "PLL clock entry x 24"]
    #[inline(always)]
    pub fn mul24(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul24)
    }
    #[doc = "PLL clock entry x 32"]
    #[inline(always)]
    pub fn mul32(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul32)
    }
    #[doc = "PLL clock entry x 48"]
    #[inline(always)]
    pub fn mul48(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul48)
    }
}
#[doc = "Field `PLLDIV` reader - PLL output division"]
pub type PLLDIV_R = crate::FieldReader<u8, PLLDIV_A>;
#[doc = "PLL output division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLDIV_A {
    #[doc = "1: PLLVCO / 2"]
    Div2 = 1,
    #[doc = "2: PLLVCO / 3"]
    Div3 = 2,
    #[doc = "3: PLLVCO / 4"]
    Div4 = 3,
}
impl From<PLLDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLDIV_A) -> Self {
        variant as _
    }
}
impl PLLDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLDIV_A> {
        match self.bits {
            1 => Some(PLLDIV_A::Div2),
            2 => Some(PLLDIV_A::Div3),
            3 => Some(PLLDIV_A::Div4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLDIV_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLDIV_A::Div3
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLDIV_A::Div4
    }
}
#[doc = "Field `PLLDIV` writer - PLL output division"]
pub type PLLDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PLLDIV_A, 2, O>;
impl<'a, const O: u8> PLLDIV_W<'a, O> {
    #[doc = "PLLVCO / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLDIV_A::Div2)
    }
    #[doc = "PLLVCO / 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLDIV_A::Div3)
    }
    #[doc = "PLLVCO / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLDIV_A::Div4)
    }
}
#[doc = "Field `MCOSEL` reader - Microcontroller clock output selection"]
pub type MCOSEL_R = crate::FieldReader<u8, MCOSEL_A>;
#[doc = "Microcontroller clock output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL_A {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI oscillator clock selected"]
    Hsi = 2,
    #[doc = "3: MSI oscillator clock selected"]
    Msi = 3,
    #[doc = "4: HSE oscillator clock selected"]
    Hse = 4,
    #[doc = "5: PLL clock selected"]
    Pll = 5,
    #[doc = "6: LSI oscillator clock selected"]
    Lsi = 6,
    #[doc = "7: LSE oscillator clock selected"]
    Lse = 7,
}
impl From<MCOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOSEL_A) -> Self {
        variant as _
    }
}
impl MCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCOSEL_A {
        match self.bits {
            0 => MCOSEL_A::NoClock,
            1 => MCOSEL_A::Sysclk,
            2 => MCOSEL_A::Hsi,
            3 => MCOSEL_A::Msi,
            4 => MCOSEL_A::Hse,
            5 => MCOSEL_A::Pll,
            6 => MCOSEL_A::Lsi,
            7 => MCOSEL_A::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoClock`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == MCOSEL_A::NoClock
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCOSEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCOSEL_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == MCOSEL_A::Msi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCOSEL_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCOSEL_A::Pll
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCOSEL_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCOSEL_A::Lse
    }
}
#[doc = "Field `MCOSEL` writer - Microcontroller clock output selection"]
pub type MCOSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, MCOSEL_A, 3, O>;
impl<'a, const O: u8> MCOSEL_W<'a, O> {
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(MCOSEL_A::NoClock)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCOSEL_A::Sysclk)
    }
    #[doc = "HSI oscillator clock selected"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCOSEL_A::Hsi)
    }
    #[doc = "MSI oscillator clock selected"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(MCOSEL_A::Msi)
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCOSEL_A::Hse)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCOSEL_A::Pll)
    }
    #[doc = "LSI oscillator clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCOSEL_A::Lsi)
    }
    #[doc = "LSE oscillator clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCOSEL_A::Lse)
    }
}
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type MCOPRE_R = crate::FieldReader<u8, MCOPRE_A>;
#[doc = "Microcontroller clock output prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE_A {
    #[doc = "0: No division"]
    Div1 = 0,
    #[doc = "1: Division by 2"]
    Div2 = 1,
    #[doc = "2: Division by 4"]
    Div4 = 2,
    #[doc = "3: Division by 8"]
    Div8 = 3,
    #[doc = "4: Division by 16"]
    Div16 = 4,
}
impl From<MCOPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE_A) -> Self {
        variant as _
    }
}
impl MCOPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCOPRE_A> {
        match self.bits {
            0 => Some(MCOPRE_A::Div1),
            1 => Some(MCOPRE_A::Div2),
            2 => Some(MCOPRE_A::Div4),
            3 => Some(MCOPRE_A::Div8),
            4 => Some(MCOPRE_A::Div16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRE_A::Div16
    }
}
#[doc = "Field `MCOPRE` writer - Microcontroller clock output prescaler"]
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCOPRE_A, 3, O>;
impl<'a, const O: u8> MCOPRE_W<'a, O> {
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div2)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div4)
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div8)
    }
    #[doc = "Division by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div16)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL multiplication factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - PLL output division"]
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output selection"]
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 8:10 - APB low-speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<16> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bits 18:21 - PLL multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<18> {
        PLLMUL_W::new(self)
    }
    #[doc = "Bits 22:23 - PLL output division"]
    #[inline(always)]
    #[must_use]
    pub fn plldiv(&mut self) -> PLLDIV_W<22> {
        PLLDIV_W::new(self)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<24> {
        MCOSEL_W::new(self)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<28> {
        MCOPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
