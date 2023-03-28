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
    Msi = 0,
    #[doc = "1: HSI16 oscillator used as system clock"]
    Hsi16 = 1,
    #[doc = "2: HSE32 oscillator used as system clock"]
    Hse32 = 2,
    #[doc = "3: PLLRCLK used as system clock"]
    Pllr = 3,
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
            0 => SW_A::Msi,
            1 => SW_A::Hsi16,
            2 => SW_A::Hse32,
            3 => SW_A::Pllr,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SW_A::Msi
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SW_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Hse32`"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == SW_A::Hse32
    }
    #[doc = "Checks if the value of the field is `Pllr`"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == SW_A::Pllr
    }
}
#[doc = "Field `SW` writer - System clock switch"]
pub type SW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, SW_A, 2, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    #[doc = "MSI oscillator used as system clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(SW_A::Msi)
    }
    #[doc = "HSI16 oscillator used as system clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(SW_A::Hsi16)
    }
    #[doc = "HSE32 oscillator used as system clock"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(SW_A::Hse32)
    }
    #[doc = "PLLRCLK used as system clock"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(SW_A::Pllr)
    }
}
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SWS_R = crate::FieldReader<u8, SWS_A>;
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWS_A {
    #[doc = "0: MSI oscillator used as system clock"]
    Msi = 0,
    #[doc = "1: HSI16 oscillator used as system clock"]
    Hsi16 = 1,
    #[doc = "2: HSE32 oscillator used as system clock"]
    Hse32 = 2,
    #[doc = "3: PLLRCLK used as system clock"]
    Pllr = 3,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWS_A {
        match self.bits {
            0 => SWS_A::Msi,
            1 => SWS_A::Hsi16,
            2 => SWS_A::Hse32,
            3 => SWS_A::Pllr,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SWS_A::Msi
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SWS_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Hse32`"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == SWS_A::Hse32
    }
    #[doc = "Checks if the value of the field is `Pllr`"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == SWS_A::Pllr
    }
}
#[doc = "Field `HPRE` reader - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
pub type HPRE_R = crate::FieldReader<u8, HPRE_A>;
#[doc = "HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "0: SYSCLK not divided"]
    Div1 = 0,
    #[doc = "1: SYSCLK divided by 3"]
    Div3 = 1,
    #[doc = "2: SYSCLK divided by 5"]
    Div5 = 2,
    #[doc = "5: SYSCLK divided by 6"]
    Div6 = 5,
    #[doc = "6: SYSCLK divided by 10"]
    Div10 = 6,
    #[doc = "7: SYSCLK divided by 32"]
    Div32 = 7,
    #[doc = "8: SYSCLK divided by 2"]
    Div2 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    Div4 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    Div8 = 10,
    #[doc = "11: SYSCLK divided by 16"]
    Div16 = 11,
    #[doc = "12: SYSCLK divided by 64"]
    Div64 = 12,
    #[doc = "13: SYSCLK divided by 128"]
    Div128 = 13,
    #[doc = "14: SYSCLK divided by 128"]
    Div256 = 14,
    #[doc = "15: SYSCLK divided by 512"]
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
            1 => Some(HPRE_A::Div3),
            2 => Some(HPRE_A::Div5),
            5 => Some(HPRE_A::Div6),
            6 => Some(HPRE_A::Div10),
            7 => Some(HPRE_A::Div32),
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
    #[doc = "Checks if the value of the field is `Div3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == HPRE_A::Div3
    }
    #[doc = "Checks if the value of the field is `Div5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == HPRE_A::Div5
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == HPRE_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == HPRE_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == HPRE_A::Div32
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
#[doc = "Field `HPRE` writer - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, HPRE_A, 4, O>;
impl<'a, const O: u8> HPRE_W<'a, O> {
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::Div1)
    }
    #[doc = "SYSCLK divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(HPRE_A::Div3)
    }
    #[doc = "SYSCLK divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(HPRE_A::Div5)
    }
    #[doc = "SYSCLK divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(HPRE_A::Div6)
    }
    #[doc = "SYSCLK divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(HPRE_A::Div10)
    }
    #[doc = "SYSCLK divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(HPRE_A::Div32)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::Div2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::Div4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::Div8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::Div16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::Div64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::Div128)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::Div256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::Div512)
    }
}
#[doc = "Field `PPRE1` reader - PCLK1 low-speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<u8, PPRE1_A>;
#[doc = "PCLK1 low-speed prescaler (APB1)\n\nValue on reset: 0"]
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
#[doc = "Field `PPRE1` writer - PCLK1 low-speed prescaler (APB1)"]
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
#[doc = "Field `PPRE2` reader - PCLK2 high-speed prescaler (APB2)"]
pub type PPRE2_R = crate::FieldReader<u8, PPRE2_A>;
#[doc = "PCLK2 high-speed prescaler (APB2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE2_A {
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
impl From<PPRE2_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE2_A) -> Self {
        variant as _
    }
}
impl PPRE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE2_A> {
        match self.bits {
            0 => Some(PPRE2_A::Div1),
            4 => Some(PPRE2_A::Div2),
            5 => Some(PPRE2_A::Div4),
            6 => Some(PPRE2_A::Div8),
            7 => Some(PPRE2_A::Div16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE2_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE2_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE2_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE2_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE2_A::Div16
    }
}
#[doc = "Field `PPRE2` writer - PCLK2 high-speed prescaler (APB2)"]
pub type PPRE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PPRE2_A, 3, O>;
impl<'a, const O: u8> PPRE2_W<'a, O> {
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE2_A::Div1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE2_A::Div2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE2_A::Div4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE2_A::Div8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE2_A::Div16)
    }
}
#[doc = "Field `STOPWUCK` reader - Wakeup from Stop and CSS backup clock selection"]
pub type STOPWUCK_R = crate::BitReader<STOPWUCK_A>;
#[doc = "Wakeup from Stop and CSS backup clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK_A {
    #[doc = "0: MSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    Msi = 0,
    #[doc = "1: HSI16 oscillator selected as wakeup from stop clock and CSS backup clock"]
    Hsi16 = 1,
}
impl From<STOPWUCK_A> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPWUCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPWUCK_A {
        match self.bits {
            false => STOPWUCK_A::Msi,
            true => STOPWUCK_A::Hsi16,
        }
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == STOPWUCK_A::Msi
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == STOPWUCK_A::Hsi16
    }
}
#[doc = "Field `STOPWUCK` writer - Wakeup from Stop and CSS backup clock selection"]
pub type STOPWUCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, STOPWUCK_A, O>;
impl<'a, const O: u8> STOPWUCK_W<'a, O> {
    #[doc = "MSI oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::Msi)
    }
    #[doc = "HSI16 oscillator selected as wakeup from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(STOPWUCK_A::Hsi16)
    }
}
#[doc = "Field `HPREF` reader - HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)"]
pub type HPREF_R = crate::BitReader<HPREF_A>;
#[doc = "HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPREF_A {
    #[doc = "0: HCLK1 prescaler value not yet applied"]
    NotApplied = 0,
    #[doc = "1: HCLK1 prescaler value applied"]
    Applied = 1,
}
impl From<HPREF_A> for bool {
    #[inline(always)]
    fn from(variant: HPREF_A) -> Self {
        variant as u8 != 0
    }
}
impl HPREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPREF_A {
        match self.bits {
            false => HPREF_A::NotApplied,
            true => HPREF_A::Applied,
        }
    }
    #[doc = "Checks if the value of the field is `NotApplied`"]
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == HPREF_A::NotApplied
    }
    #[doc = "Checks if the value of the field is `Applied`"]
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == HPREF_A::Applied
    }
}
#[doc = "Field `PPRE1F` reader - PCLK1 prescaler flag (APB1)"]
pub type PPRE1F_R = crate::BitReader<PPRE1F_A>;
#[doc = "PCLK1 prescaler flag (APB1)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPRE1F_A {
    #[doc = "0: PCLK1 prescaler value not yet applied"]
    NotApplied = 0,
    #[doc = "1: PCLK1 prescaler value applied"]
    Applied = 1,
}
impl From<PPRE1F_A> for bool {
    #[inline(always)]
    fn from(variant: PPRE1F_A) -> Self {
        variant as u8 != 0
    }
}
impl PPRE1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPRE1F_A {
        match self.bits {
            false => PPRE1F_A::NotApplied,
            true => PPRE1F_A::Applied,
        }
    }
    #[doc = "Checks if the value of the field is `NotApplied`"]
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == PPRE1F_A::NotApplied
    }
    #[doc = "Checks if the value of the field is `Applied`"]
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == PPRE1F_A::Applied
    }
}
#[doc = "Field `PPRE2F` reader - PCLK2 prescaler flag (APB2)"]
pub type PPRE2F_R = crate::BitReader<PPRE2F_A>;
#[doc = "PCLK2 prescaler flag (APB2)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPRE2F_A {
    #[doc = "0: PCLK2 prescaler value not yet applied"]
    NotApplied = 0,
    #[doc = "1: PCLK2 prescaler value applied"]
    Applied = 1,
}
impl From<PPRE2F_A> for bool {
    #[inline(always)]
    fn from(variant: PPRE2F_A) -> Self {
        variant as u8 != 0
    }
}
impl PPRE2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPRE2F_A {
        match self.bits {
            false => PPRE2F_A::NotApplied,
            true => PPRE2F_A::Applied,
        }
    }
    #[doc = "Checks if the value of the field is `NotApplied`"]
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == PPRE2F_A::NotApplied
    }
    #[doc = "Checks if the value of the field is `Applied`"]
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == PPRE2F_A::Applied
    }
}
#[doc = "Field `MCOSEL` reader - Microcontroller clock output"]
pub type MCOSEL_R = crate::FieldReader<u8, MCOSEL_A>;
#[doc = "Microcontroller clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL_A {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: MSI oscillator clock selected"]
    Msi = 2,
    #[doc = "3: HSI16 oscillator clock selected"]
    Hsi16 = 3,
    #[doc = "4: HSE32 oscillator clock selected"]
    Hse32 = 4,
    #[doc = "5: Main PLLRCLK clock selected"]
    Pllr = 5,
    #[doc = "6: LSI oscillator clock selected"]
    Lsi = 6,
    #[doc = "8: LSE oscillator clock selected"]
    Lse = 8,
    #[doc = "13: Main PLLPCLK clock selected"]
    Pllp = 13,
    #[doc = "14: Main PLLQCLK clock selected"]
    Pllq = 14,
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
    pub fn variant(&self) -> Option<MCOSEL_A> {
        match self.bits {
            0 => Some(MCOSEL_A::NoClock),
            1 => Some(MCOSEL_A::Sysclk),
            2 => Some(MCOSEL_A::Msi),
            3 => Some(MCOSEL_A::Hsi16),
            4 => Some(MCOSEL_A::Hse32),
            5 => Some(MCOSEL_A::Pllr),
            6 => Some(MCOSEL_A::Lsi),
            8 => Some(MCOSEL_A::Lse),
            13 => Some(MCOSEL_A::Pllp),
            14 => Some(MCOSEL_A::Pllq),
            _ => None,
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
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == MCOSEL_A::Msi
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == MCOSEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Hse32`"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == MCOSEL_A::Hse32
    }
    #[doc = "Checks if the value of the field is `Pllr`"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == MCOSEL_A::Pllr
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
    #[doc = "Checks if the value of the field is `Pllp`"]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == MCOSEL_A::Pllp
    }
    #[doc = "Checks if the value of the field is `Pllq`"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == MCOSEL_A::Pllq
    }
}
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
pub type MCOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCOSEL_A, 4, O>;
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
    #[doc = "MSI oscillator clock selected"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(MCOSEL_A::Msi)
    }
    #[doc = "HSI16 oscillator clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(MCOSEL_A::Hsi16)
    }
    #[doc = "HSE32 oscillator clock selected"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(MCOSEL_A::Hse32)
    }
    #[doc = "Main PLLRCLK clock selected"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(MCOSEL_A::Pllr)
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
    #[doc = "Main PLLPCLK clock selected"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(MCOSEL_A::Pllp)
    }
    #[doc = "Main PLLQCLK clock selected"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(MCOSEL_A::Pllq)
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
    #[doc = "Bits 4:7 - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - PCLK1 low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - PCLK2 high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)"]
    #[inline(always)]
    pub fn hpref(&self) -> HPREF_R {
        HPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PCLK1 prescaler flag (APB1)"]
    #[inline(always)]
    pub fn ppre1f(&self) -> PPRE1F_R {
        PPRE1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCLK2 prescaler flag (APB2)"]
    #[inline(always)]
    pub fn ppre2f(&self) -> PPRE2F_R {
        PPRE2F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
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
    #[doc = "Bits 4:7 - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 8:10 - PCLK1 low-speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 11:13 - PCLK2 high-speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<15> {
        STOPWUCK_W::new(self)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
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
#[doc = "`reset()` method sets CFGR to value 0x0007_0000"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0000;
}
