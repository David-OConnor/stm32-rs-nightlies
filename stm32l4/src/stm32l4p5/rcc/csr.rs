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
    #[doc = "0: LSI oscillator OFF"]
    Disabled = 0,
    #[doc = "1: LSI oscillator ON"]
    Enabled = 1,
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
            false => LSION_A::Disabled,
            true => LSION_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSION_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSION_A::Enabled
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable"]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSION_A, O>;
impl<'a, const O: u8> LSION_W<'a, O> {
    #[doc = "LSI oscillator OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSION_A::Disabled)
    }
    #[doc = "LSI oscillator ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSION_A::Enabled)
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
#[doc = "Field `LSIPREDIV` reader - Internal low-speed oscillator predivided by 128"]
pub type LSIPREDIV_R = crate::BitReader<LSIPREDIV_A>;
#[doc = "Internal low-speed oscillator predivided by 128\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIPREDIV_A {
    #[doc = "0: LSI PREDIV OFF"]
    Disabled = 0,
    #[doc = "1: LSI PREDIV ON"]
    Enabled = 1,
}
impl From<LSIPREDIV_A> for bool {
    #[inline(always)]
    fn from(variant: LSIPREDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIPREDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIPREDIV_A {
        match self.bits {
            false => LSIPREDIV_A::Disabled,
            true => LSIPREDIV_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIPREDIV_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIPREDIV_A::Enabled
    }
}
#[doc = "Field `LSIPREDIV` writer - Internal low-speed oscillator predivided by 128"]
pub type LSIPREDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSIPREDIV_A, O>;
impl<'a, const O: u8> LSIPREDIV_W<'a, O> {
    #[doc = "LSI PREDIV OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIPREDIV_A::Disabled)
    }
    #[doc = "LSI PREDIV ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIPREDIV_A::Enabled)
    }
}
#[doc = "Field `MSISRANGE` reader - SI range after Standby mode"]
pub type MSISRANGE_R = crate::FieldReader<u8, MSISRANGE_A>;
#[doc = "SI range after Standby mode\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSISRANGE_A {
    #[doc = "4: range 4 around 1 MHz"]
    Range1m = 4,
    #[doc = "5: range 5 around 2 MHz"]
    Range2m = 5,
    #[doc = "6: range 6 around 4 MHz"]
    Range4m = 6,
    #[doc = "7: range 7 around 8 MHz"]
    Range8m = 7,
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
            4 => Some(MSISRANGE_A::Range1m),
            5 => Some(MSISRANGE_A::Range2m),
            6 => Some(MSISRANGE_A::Range4m),
            7 => Some(MSISRANGE_A::Range8m),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Range1m`"]
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSISRANGE_A::Range1m
    }
    #[doc = "Checks if the value of the field is `Range2m`"]
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSISRANGE_A::Range2m
    }
    #[doc = "Checks if the value of the field is `Range4m`"]
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSISRANGE_A::Range4m
    }
    #[doc = "Checks if the value of the field is `Range8m`"]
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSISRANGE_A::Range8m
    }
}
#[doc = "Field `MSISRANGE` writer - SI range after Standby mode"]
pub type MSISRANGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSR_SPEC, u8, MSISRANGE_A, 4, O>;
impl<'a, const O: u8> MSISRANGE_W<'a, O> {
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn range1m(self) -> &'a mut W {
        self.variant(MSISRANGE_A::Range1m)
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn range2m(self) -> &'a mut W {
        self.variant(MSISRANGE_A::Range2m)
    }
    #[doc = "range 6 around 4 MHz"]
    #[inline(always)]
    pub fn range4m(self) -> &'a mut W {
        self.variant(MSISRANGE_A::Range4m)
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn range8m(self) -> &'a mut W {
        self.variant(MSISRANGE_A::Range8m)
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVF_A>;
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clear the reset flags"]
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
    #[doc = "Clear the reset flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVF_A::Clear)
    }
}
#[doc = "Field `FWRSTF` reader - Firewall reset flag"]
pub type FWRSTF_R = crate::BitReader<FWRSTF_A>;
#[doc = "Firewall reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWRSTF_A {
    #[doc = "0: No reset from the firewall occurred"]
    NotOccured = 0,
    #[doc = "1: Reset from the firewall occurred"]
    Occured = 1,
}
impl From<FWRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: FWRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl FWRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWRSTF_A {
        match self.bits {
            false => FWRSTF_A::NotOccured,
            true => FWRSTF_A::Occured,
        }
    }
    #[doc = "Checks if the value of the field is `NotOccured`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == FWRSTF_A::NotOccured
    }
    #[doc = "Checks if the value of the field is `Occured`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == FWRSTF_A::Occured
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OBLRSTF_R = crate::BitReader<OBLRSTF_A>;
#[doc = "Option byte loader reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF_A {
    #[doc = "0: No reset from Option Byte loading occurred"]
    NotOccured = 0,
    #[doc = "1: Reset from Option Byte loading occurred"]
    Occured = 1,
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
            false => OBLRSTF_A::NotOccured,
            true => OBLRSTF_A::Occured,
        }
    }
    #[doc = "Checks if the value of the field is `NotOccured`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == OBLRSTF_A::NotOccured
    }
    #[doc = "Checks if the value of the field is `Occured`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == OBLRSTF_A::Occured
    }
}
#[doc = "Field `PINRSTF` reader - Pin reset flag"]
pub type PINRSTF_R = crate::BitReader<PINRSTF_A>;
#[doc = "Pin reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTF_A {
    #[doc = "0: No reset from NRST pin occurred"]
    NotOccured = 0,
    #[doc = "1: Reset from NRST pin occurred"]
    Occured = 1,
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
            false => PINRSTF_A::NotOccured,
            true => PINRSTF_A::Occured,
        }
    }
    #[doc = "Checks if the value of the field is `NotOccured`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == PINRSTF_A::NotOccured
    }
    #[doc = "Checks if the value of the field is `Occured`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == PINRSTF_A::Occured
    }
}
#[doc = "Field `BORRSTF` reader - BOR flag"]
pub type BORRSTF_R = crate::BitReader<BORRSTF_A>;
#[doc = "BOR flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTF_A {
    #[doc = "0: No BOR occurred"]
    NotOccured = 0,
    #[doc = "1: BOR occurred"]
    Occured = 1,
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
            false => BORRSTF_A::NotOccured,
            true => BORRSTF_A::Occured,
        }
    }
    #[doc = "Checks if the value of the field is `NotOccured`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == BORRSTF_A::NotOccured
    }
    #[doc = "Checks if the value of the field is `Occured`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == BORRSTF_A::Occured
    }
}
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader<SFTRSTF_A>;
#[doc = "Software reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTF_A {
    #[doc = "0: No software reset occurred"]
    NotOccured = 0,
    #[doc = "1: Software reset occurred"]
    Occured = 1,
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
            false => SFTRSTF_A::NotOccured,
            true => SFTRSTF_A::Occured,
        }
    }
    #[doc = "Checks if the value of the field is `NotOccured`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == SFTRSTF_A::NotOccured
    }
    #[doc = "Checks if the value of the field is `Occured`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == SFTRSTF_A::Occured
    }
}
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub type IWDGRSTF_R = crate::BitReader<IWDGRSTF_A>;
#[doc = "Independent window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTF_A {
    #[doc = "0: No independent watchdog reset occurred"]
    NotOccured = 0,
    #[doc = "1: Independent watchdog reset occurred"]
    Occured = 1,
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
            false => IWDGRSTF_A::NotOccured,
            true => IWDGRSTF_A::Occured,
        }
    }
    #[doc = "Checks if the value of the field is `NotOccured`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == IWDGRSTF_A::NotOccured
    }
    #[doc = "Checks if the value of the field is `Occured`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == IWDGRSTF_A::Occured
    }
}
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader<WWDGRSTF_A>;
#[doc = "Window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTF_A {
    #[doc = "0: No window watchdog reset occurred"]
    NotOccured = 0,
    #[doc = "1: Window watchdog reset occurred"]
    Occured = 1,
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
            false => WWDGRSTF_A::NotOccured,
            true => WWDGRSTF_A::Occured,
        }
    }
    #[doc = "Checks if the value of the field is `NotOccured`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == WWDGRSTF_A::NotOccured
    }
    #[doc = "Checks if the value of the field is `Occured`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == WWDGRSTF_A::Occured
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub type LPWRRSTF_R = crate::BitReader<LPWRRSTF_A>;
#[doc = "Low-power reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTF_A {
    #[doc = "0: No illegal mode reset occurred"]
    NotOccured = 0,
    #[doc = "1: Illegal mode reset occurred"]
    Occured = 1,
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
            false => LPWRRSTF_A::NotOccured,
            true => LPWRRSTF_A::Occured,
        }
    }
    #[doc = "Checks if the value of the field is `NotOccured`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == LPWRRSTF_A::NotOccured
    }
    #[doc = "Checks if the value of the field is `Occured`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == LPWRRSTF_A::Occured
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
    #[doc = "Bit 4 - Internal low-speed oscillator predivided by 128"]
    #[inline(always)]
    pub fn lsiprediv(&self) -> LSIPREDIV_R {
        LSIPREDIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - SI range after Standby mode"]
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Firewall reset flag"]
    #[inline(always)]
    pub fn fwrstf(&self) -> FWRSTF_R {
        FWRSTF_R::new(((self.bits >> 24) & 1) != 0)
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
    #[doc = "Bit 4 - Internal low-speed oscillator predivided by 128"]
    #[inline(always)]
    #[must_use]
    pub fn lsiprediv(&mut self) -> LSIPREDIV_W<4> {
        LSIPREDIV_W::new(self)
    }
    #[doc = "Bits 8:11 - SI range after Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn msisrange(&mut self) -> MSISRANGE_W<8> {
        MSISRANGE_W::new(self)
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
#[doc = "CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
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
#[doc = "`reset()` method sets CSR to value 0x0c00_0600"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0600;
}
