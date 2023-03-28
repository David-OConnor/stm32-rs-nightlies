#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSION` reader - LSI oscillator enable"]
pub type LSION_R = crate::BitReader<LSION_A>;
#[doc = "LSI oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION_A {
    #[doc = "0: LSI oscillator off"]
    Off = 0,
    #[doc = "1: LSI oscillator on"]
    On = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
impl LSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::Off,
            true => LSION_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION_A::On
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable"]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSION_A, O>;
impl<'a, const O: u8> LSION_W<'a, O> {
    #[doc = "LSI oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSION_A::Off)
    }
    #[doc = "LSI oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSION_A::On)
    }
}
#[doc = "Field `LSIRDY` reader - LSI oscillator ready"]
pub type LSIRDY_R = crate::BitReader<LSIRDY_A>;
#[doc = "LSI oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY_A {
    #[doc = "0: LSI oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSI oscillator ready"]
    Ready = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::NotReady,
            true => LSIRDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDY_A::Ready
    }
}
#[doc = "Field `LSIPRE` reader - LSI frequency prescaler"]
pub type LSIPRE_R = crate::BitReader<LSIPRE_A>;
#[doc = "LSI frequency prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIPRE_A {
    #[doc = "0: LSI clock not divided"]
    Div1 = 0,
    #[doc = "1: LSI clock divided by 128"]
    Div128 = 1,
}
impl From<LSIPRE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIPRE_A {
        match self.bits {
            false => LSIPRE_A::Div1,
            true => LSIPRE_A::Div128,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LSIPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LSIPRE_A::Div128
    }
}
#[doc = "Field `LSIPRE` writer - LSI frequency prescaler"]
pub type LSIPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSIPRE_A, O>;
impl<'a, const O: u8> LSIPRE_W<'a, O> {
    #[doc = "LSI clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LSIPRE_A::Div1)
    }
    #[doc = "LSI clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LSIPRE_A::Div128)
    }
}
#[doc = "Field `MSISRANGE` reader - MSI clock ranges"]
pub type MSISRANGE_R = crate::FieldReader<u8, MSISRANGE_A>;
#[doc = "MSI clock ranges\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSISRANGE_A {
    #[doc = "4: Range 4 around 1 MHz"]
    F1mhz = 4,
    #[doc = "5: Range 5 around 2 MHz"]
    F2mhz = 5,
    #[doc = "6: Range 6 around 4 MHz (reset value)"]
    F4mhz = 6,
    #[doc = "7: Range 7 around 8 MHz"]
    F8mhz = 7,
}
impl From<MSISRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSISRANGE_A) -> Self {
        variant as _
    }
}
impl MSISRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSISRANGE_A> {
        match self.bits {
            4 => Some(MSISRANGE_A::F1mhz),
            5 => Some(MSISRANGE_A::F2mhz),
            6 => Some(MSISRANGE_A::F4mhz),
            7 => Some(MSISRANGE_A::F8mhz),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `F1mhz`"]
    #[inline(always)]
    pub fn is_f_1mhz(&self) -> bool {
        *self == MSISRANGE_A::F1mhz
    }
    #[doc = "Checks if the value of the field is `F2mhz`"]
    #[inline(always)]
    pub fn is_f_2mhz(&self) -> bool {
        *self == MSISRANGE_A::F2mhz
    }
    #[doc = "Checks if the value of the field is `F4mhz`"]
    #[inline(always)]
    pub fn is_f_4mhz(&self) -> bool {
        *self == MSISRANGE_A::F4mhz
    }
    #[doc = "Checks if the value of the field is `F8mhz`"]
    #[inline(always)]
    pub fn is_f_8mhz(&self) -> bool {
        *self == MSISRANGE_A::F8mhz
    }
}
#[doc = "Field `MSISRANGE` writer - MSI clock ranges"]
pub type MSISRANGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSR_SPEC, u8, MSISRANGE_A, 4, O>;
impl<'a, const O: u8> MSISRANGE_W<'a, O> {
    #[doc = "Range 4 around 1 MHz"]
    #[inline(always)]
    pub fn f_1mhz(self) -> &'a mut W {
        self.variant(MSISRANGE_A::F1mhz)
    }
    #[doc = "Range 5 around 2 MHz"]
    #[inline(always)]
    pub fn f_2mhz(self) -> &'a mut W {
        self.variant(MSISRANGE_A::F2mhz)
    }
    #[doc = "Range 6 around 4 MHz (reset value)"]
    #[inline(always)]
    pub fn f_4mhz(self) -> &'a mut W {
        self.variant(MSISRANGE_A::F4mhz)
    }
    #[doc = "Range 7 around 8 MHz"]
    #[inline(always)]
    pub fn f_8mhz(self) -> &'a mut W {
        self.variant(MSISRANGE_A::F8mhz)
    }
}
#[doc = "Field `RFRSTF` reader - Radio in reset status flag"]
pub type RFRSTF_R = crate::BitReader<RFRSTF_A>;
#[doc = "Radio in reset status flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRSTF_A {
    #[doc = "0: Sub-GHz radio out of reset"]
    NoReset = 0,
    #[doc = "1: Sub-GHz radio in reset"]
    Reset = 1,
}
impl From<RFRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: RFRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRSTF_A {
        match self.bits {
            false => RFRSTF_A::NoReset,
            true => RFRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == RFRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RFRSTF_A::Reset
    }
}
#[doc = "Field `RFRST` reader - Radio reset"]
pub type RFRST_R = crate::BitReader<RFRST_A>;
#[doc = "Radio reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_A {
    #[doc = "0: Sub-GHz radio software reset removed"]
    Removed = 0,
    #[doc = "1: Sub-GHz radio software reset active"]
    Reset = 1,
}
impl From<RFRST_A> for bool {
    #[inline(always)]
    fn from(variant: RFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRST_A {
        match self.bits {
            false => RFRST_A::Removed,
            true => RFRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `Removed`"]
    #[inline(always)]
    pub fn is_removed(&self) -> bool {
        *self == RFRST_A::Removed
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RFRST_A::Reset
    }
}
#[doc = "Field `RFRST` writer - Radio reset"]
pub type RFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RFRST_A, O>;
impl<'a, const O: u8> RFRST_W<'a, O> {
    #[doc = "Sub-GHz radio software reset removed"]
    #[inline(always)]
    pub fn removed(self) -> &'a mut W {
        self.variant(RFRST_A::Removed)
    }
    #[doc = "Sub-GHz radio software reset active"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RFRST_A::Reset)
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVF_A>;
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset flags reset"]
    Clear = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMVF_A {
        match self.bits {
            false => RMVF_A::NoEffect,
            true => RMVF_A::Clear,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RMVF_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF_A::Clear
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RMVF_A, O>;
impl<'a, const O: u8> RMVF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RMVF_A::NoEffect)
    }
    #[doc = "Reset flags reset"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVF_A::Clear)
    }
}
#[doc = "Field `RFILARSTF` reader - Radio illegal access flag"]
pub type RFILARSTF_R = crate::BitReader<RFILARSTF_A>;
#[doc = "Radio illegal access flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFILARSTF_A {
    #[doc = "0: No SUBGHZ radio illegal command occurred"]
    NoIllegalCommand = 0,
    #[doc = "1: SUBGHZ radio illegal command occurred"]
    IllegalCommand = 1,
}
impl From<RFILARSTF_A> for bool {
    #[inline(always)]
    fn from(variant: RFILARSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFILARSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFILARSTF_A {
        match self.bits {
            false => RFILARSTF_A::NoIllegalCommand,
            true => RFILARSTF_A::IllegalCommand,
        }
    }
    #[doc = "Checks if the value of the field is `NoIllegalCommand`"]
    #[inline(always)]
    pub fn is_no_illegal_command(&self) -> bool {
        *self == RFILARSTF_A::NoIllegalCommand
    }
    #[doc = "Checks if the value of the field is `IllegalCommand`"]
    #[inline(always)]
    pub fn is_illegal_command(&self) -> bool {
        *self == RFILARSTF_A::IllegalCommand
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OBLRSTF_R = crate::BitReader<OBLRSTF_A>;
#[doc = "Option byte loader reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF_A {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<OBLRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl OBLRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBLRSTF_A {
        match self.bits {
            false => OBLRSTF_A::NoReset,
            true => OBLRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTF_A::Reset
    }
}
#[doc = "Field `PINRSTF` reader - Pin reset flag"]
pub type PINRSTF_R = crate::BitReader<PINRSTF_A>;
#[doc = "Pin reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTF_A {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<PINRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: PINRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl PINRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINRSTF_A {
        match self.bits {
            false => PINRSTF_A::NoReset,
            true => PINRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == PINRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PINRSTF_A::Reset
    }
}
#[doc = "Field `BORRSTF` reader - BOR flag"]
pub type BORRSTF_R = crate::BitReader<BORRSTF_A>;
#[doc = "BOR flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTF_A {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<BORRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: BORRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl BORRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORRSTF_A {
        match self.bits {
            false => BORRSTF_A::NoReset,
            true => BORRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BORRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BORRSTF_A::Reset
    }
}
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader<SFTRSTF_A>;
#[doc = "Software reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTF_A {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<SFTRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: SFTRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl SFTRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFTRSTF_A {
        match self.bits {
            false => SFTRSTF_A::NoReset,
            true => SFTRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == SFTRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SFTRSTF_A::Reset
    }
}
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub type IWDGRSTF_R = crate::BitReader<IWDGRSTF_A>;
#[doc = "Independent window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTF_A {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<IWDGRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDGRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDGRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDGRSTF_A {
        match self.bits {
            false => IWDGRSTF_A::NoReset,
            true => IWDGRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == IWDGRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IWDGRSTF_A::Reset
    }
}
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader<WWDGRSTF_A>;
#[doc = "Window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTF_A {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<WWDGRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDGRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDGRSTF_A {
        match self.bits {
            false => WWDGRSTF_A::NoReset,
            true => WWDGRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == WWDGRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WWDGRSTF_A::Reset
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub type LPWRRSTF_R = crate::BitReader<LPWRRSTF_A>;
#[doc = "Low-power reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTF_A {
    #[doc = "0: No reset occurred"]
    NoReset = 0,
    #[doc = "1: Reset occurred"]
    Reset = 1,
}
impl From<LPWRRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl LPWRRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPWRRSTF_A {
        match self.bits {
            false => LPWRRSTF_A::NoReset,
            true => LPWRRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == LPWRRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPWRRSTF_A::Reset
    }
}
impl R {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LSI frequency prescaler"]
    #[inline(always)]
    pub fn lsipre(&self) -> LSIPRE_R {
        LSIPRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - MSI clock ranges"]
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Radio in reset status flag"]
    #[inline(always)]
    pub fn rfrstf(&self) -> RFRSTF_R {
        RFRSTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Radio reset"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Radio illegal access flag"]
    #[inline(always)]
    pub fn rfilarstf(&self) -> RFILARSTF_R {
        RFILARSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    #[doc = "Bit 4 - LSI frequency prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn lsipre(&mut self) -> LSIPRE_W<4> {
        LSIPRE_W::new(self)
    }
    #[doc = "Bits 8:11 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msisrange(&mut self) -> MSISRANGE_W<8> {
        MSISRANGE_W::new(self)
    }
    #[doc = "Bit 15 - Radio reset"]
    #[inline(always)]
    #[must_use]
    pub fn rfrst(&mut self) -> RFRST_W<15> {
        RFRST_W::new(self)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<23> {
        RMVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0x0c01_c600"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c01_c600;
}
