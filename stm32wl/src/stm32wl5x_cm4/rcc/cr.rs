#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSION` reader - MSI clock enable"]
pub type MSION_R = crate::BitReader<MSION_A>;
#[doc = "MSI clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSION_A {
    #[doc = "0: MSI oscillator off"]
    Disabled = 0,
    #[doc = "1: MSI oscillator on"]
    Enabled = 1,
}
impl From<MSION_A> for bool {
    #[inline(always)]
    fn from(variant: MSION_A) -> Self {
        variant as u8 != 0
    }
}
impl MSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSION_A {
        match self.bits {
            false => MSION_A::Disabled,
            true => MSION_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSION_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSION_A::Enabled
    }
}
#[doc = "Field `MSION` writer - MSI clock enable"]
pub type MSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MSION_A, O>;
impl<'a, const O: u8> MSION_W<'a, O> {
    #[doc = "MSI oscillator off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSION_A::Disabled)
    }
    #[doc = "MSI oscillator on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSION_A::Enabled)
    }
}
#[doc = "Field `MSIRDY` reader - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)"]
pub type MSIRDY_R = crate::BitReader<MSIRDY_A>;
#[doc = "MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDY_A {
    #[doc = "0: MSI oscillator not ready"]
    NotReady = 0,
    #[doc = "1: MSI oscillator ready"]
    Ready = 1,
}
impl From<MSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRDY_A {
        match self.bits {
            false => MSIRDY_A::NotReady,
            true => MSIRDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDY_A::Ready
    }
}
#[doc = "Field `MSIPLLEN` reader - MSI clock PLL enable"]
pub type MSIPLLEN_R = crate::BitReader<MSIPLLEN_A>;
#[doc = "MSI clock PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIPLLEN_A {
    #[doc = "0: MSI PLL Off"]
    Off = 0,
    #[doc = "1: MSI PLL On"]
    On = 1,
}
impl From<MSIPLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSIPLLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIPLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIPLLEN_A {
        match self.bits {
            false => MSIPLLEN_A::Off,
            true => MSIPLLEN_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MSIPLLEN_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == MSIPLLEN_A::On
    }
}
#[doc = "Field `MSIPLLEN` writer - MSI clock PLL enable"]
pub type MSIPLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MSIPLLEN_A, O>;
impl<'a, const O: u8> MSIPLLEN_W<'a, O> {
    #[doc = "MSI PLL Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MSIPLLEN_A::Off)
    }
    #[doc = "MSI PLL On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(MSIPLLEN_A::On)
    }
}
#[doc = "Field `MSIRGSEL` reader - MSI range control selection"]
pub type MSIRGSEL_R = crate::BitReader<MSIRGSEL_A>;
#[doc = "MSI range control selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRGSEL_A {
    #[doc = "0: MSI frequency range defined by MSISRANGE\\[3:0\\]
in the RCC_CSR register"]
    Csr = 0,
    #[doc = "1: MSI frequency range defined by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    Cr = 1,
}
impl From<MSIRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRGSEL_A {
        match self.bits {
            false => MSIRGSEL_A::Csr,
            true => MSIRGSEL_A::Cr,
        }
    }
    #[doc = "Checks if the value of the field is `Csr`"]
    #[inline(always)]
    pub fn is_csr(&self) -> bool {
        *self == MSIRGSEL_A::Csr
    }
    #[doc = "Checks if the value of the field is `Cr`"]
    #[inline(always)]
    pub fn is_cr(&self) -> bool {
        *self == MSIRGSEL_A::Cr
    }
}
#[doc = "Field `MSIRGSEL` writer - MSI range control selection"]
pub type MSIRGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MSIRGSEL_A, O>;
impl<'a, const O: u8> MSIRGSEL_W<'a, O> {
    #[doc = "MSI frequency range defined by MSISRANGE\\[3:0\\]
in the RCC_CSR register"]
    #[inline(always)]
    pub fn csr(self) -> &'a mut W {
        self.variant(MSIRGSEL_A::Csr)
    }
    #[doc = "MSI frequency range defined by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    #[inline(always)]
    pub fn cr(self) -> &'a mut W {
        self.variant(MSIRGSEL_A::Cr)
    }
}
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader<u8, MSIRANGE_A>;
#[doc = "MSI clock ranges\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIRANGE_A {
    #[doc = "0: range 0 around 100 kHz"]
    Range100k = 0,
    #[doc = "1: range 1 around 200 kHz"]
    Range200k = 1,
    #[doc = "2: range 2 around 400 kHz"]
    Range400k = 2,
    #[doc = "3: range 3 around 800 kHz"]
    Range800k = 3,
    #[doc = "4: range 4 around 1 MHz"]
    Range1m = 4,
    #[doc = "5: range 5 around 2 MHz"]
    Range2m = 5,
    #[doc = "6: range 6 around 4 MHz (reset value)"]
    Range4m = 6,
    #[doc = "7: range 7 around 8 MHz"]
    Range8m = 7,
    #[doc = "8: range 8 around 16 MHz"]
    Range16m = 8,
    #[doc = "9: range 9 around 24 MHz"]
    Range24m = 9,
    #[doc = "10: range 10 around 32 MHz"]
    Range32m = 10,
    #[doc = "11: range 11 around 48 MHz"]
    Range48m = 11,
}
impl From<MSIRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE_A) -> Self {
        variant as _
    }
}
impl MSIRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSIRANGE_A> {
        match self.bits {
            0 => Some(MSIRANGE_A::Range100k),
            1 => Some(MSIRANGE_A::Range200k),
            2 => Some(MSIRANGE_A::Range400k),
            3 => Some(MSIRANGE_A::Range800k),
            4 => Some(MSIRANGE_A::Range1m),
            5 => Some(MSIRANGE_A::Range2m),
            6 => Some(MSIRANGE_A::Range4m),
            7 => Some(MSIRANGE_A::Range8m),
            8 => Some(MSIRANGE_A::Range16m),
            9 => Some(MSIRANGE_A::Range24m),
            10 => Some(MSIRANGE_A::Range32m),
            11 => Some(MSIRANGE_A::Range48m),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Range100k`"]
    #[inline(always)]
    pub fn is_range100k(&self) -> bool {
        *self == MSIRANGE_A::Range100k
    }
    #[doc = "Checks if the value of the field is `Range200k`"]
    #[inline(always)]
    pub fn is_range200k(&self) -> bool {
        *self == MSIRANGE_A::Range200k
    }
    #[doc = "Checks if the value of the field is `Range400k`"]
    #[inline(always)]
    pub fn is_range400k(&self) -> bool {
        *self == MSIRANGE_A::Range400k
    }
    #[doc = "Checks if the value of the field is `Range800k`"]
    #[inline(always)]
    pub fn is_range800k(&self) -> bool {
        *self == MSIRANGE_A::Range800k
    }
    #[doc = "Checks if the value of the field is `Range1m`"]
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSIRANGE_A::Range1m
    }
    #[doc = "Checks if the value of the field is `Range2m`"]
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSIRANGE_A::Range2m
    }
    #[doc = "Checks if the value of the field is `Range4m`"]
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSIRANGE_A::Range4m
    }
    #[doc = "Checks if the value of the field is `Range8m`"]
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSIRANGE_A::Range8m
    }
    #[doc = "Checks if the value of the field is `Range16m`"]
    #[inline(always)]
    pub fn is_range16m(&self) -> bool {
        *self == MSIRANGE_A::Range16m
    }
    #[doc = "Checks if the value of the field is `Range24m`"]
    #[inline(always)]
    pub fn is_range24m(&self) -> bool {
        *self == MSIRANGE_A::Range24m
    }
    #[doc = "Checks if the value of the field is `Range32m`"]
    #[inline(always)]
    pub fn is_range32m(&self) -> bool {
        *self == MSIRANGE_A::Range32m
    }
    #[doc = "Checks if the value of the field is `Range48m`"]
    #[inline(always)]
    pub fn is_range48m(&self) -> bool {
        *self == MSIRANGE_A::Range48m
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, MSIRANGE_A, 4, O>;
impl<'a, const O: u8> MSIRANGE_W<'a, O> {
    #[doc = "range 0 around 100 kHz"]
    #[inline(always)]
    pub fn range100k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range100k)
    }
    #[doc = "range 1 around 200 kHz"]
    #[inline(always)]
    pub fn range200k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range200k)
    }
    #[doc = "range 2 around 400 kHz"]
    #[inline(always)]
    pub fn range400k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range400k)
    }
    #[doc = "range 3 around 800 kHz"]
    #[inline(always)]
    pub fn range800k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range800k)
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn range1m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range1m)
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn range2m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range2m)
    }
    #[doc = "range 6 around 4 MHz (reset value)"]
    #[inline(always)]
    pub fn range4m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range4m)
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn range8m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range8m)
    }
    #[doc = "range 8 around 16 MHz"]
    #[inline(always)]
    pub fn range16m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range16m)
    }
    #[doc = "range 9 around 24 MHz"]
    #[inline(always)]
    pub fn range24m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range24m)
    }
    #[doc = "range 10 around 32 MHz"]
    #[inline(always)]
    pub fn range32m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range32m)
    }
    #[doc = "range 11 around 48 MHz"]
    #[inline(always)]
    pub fn range48m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range48m)
    }
}
#[doc = "Field `HSION` reader - HSI16 clock enable"]
pub type HSION_R = crate::BitReader<HSION_A>;
#[doc = "HSI16 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION_A {
    #[doc = "0: HSI16 oscillator off"]
    Disabled = 0,
    #[doc = "1: HSI16 oscillator on"]
    Enabled = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::Disabled,
            true => HSION_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION_A::Enabled
    }
}
#[doc = "Field `HSION` writer - HSI16 clock enable"]
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSION_A, O>;
impl<'a, const O: u8> HSION_W<'a, O> {
    #[doc = "HSI16 oscillator off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSION_A::Disabled)
    }
    #[doc = "HSI16 oscillator on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSION_A::Enabled)
    }
}
#[doc = "Field `HSIKERON` reader - HSI16 always enable for peripheral kernel clocks."]
pub type HSIKERON_R = crate::BitReader<HSIKERON_A>;
#[doc = "HSI16 always enable for peripheral kernel clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON_A {
    #[doc = "0: No effect on HSI16 oscillator"]
    NotForced = 0,
    #[doc = "1: HSI16 oscillator forced on even in Stop modes"]
    Forced = 1,
}
impl From<HSIKERON_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIKERON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIKERON_A {
        match self.bits {
            false => HSIKERON_A::NotForced,
            true => HSIKERON_A::Forced,
        }
    }
    #[doc = "Checks if the value of the field is `NotForced`"]
    #[inline(always)]
    pub fn is_not_forced(&self) -> bool {
        *self == HSIKERON_A::NotForced
    }
    #[doc = "Checks if the value of the field is `Forced`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == HSIKERON_A::Forced
    }
}
#[doc = "Field `HSIKERON` writer - HSI16 always enable for peripheral kernel clocks."]
pub type HSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSIKERON_A, O>;
impl<'a, const O: u8> HSIKERON_W<'a, O> {
    #[doc = "No effect on HSI16 oscillator"]
    #[inline(always)]
    pub fn not_forced(self) -> &'a mut W {
        self.variant(HSIKERON_A::NotForced)
    }
    #[doc = "HSI16 oscillator forced on even in Stop modes"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(HSIKERON_A::Forced)
    }
}
#[doc = "Field `HSIRDY` reader - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)"]
pub type HSIRDY_R = crate::BitReader<HSIRDY_A>;
#[doc = "HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDY_A {
    #[doc = "0: HSI16 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI16 oscillator ready"]
    Ready = 1,
}
impl From<HSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDY_A {
        match self.bits {
            false => HSIRDY_A::NotReady,
            true => HSIRDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDY_A::Ready
    }
}
#[doc = "Field `HSIASFS` reader - HSI16 automatic start from Stop"]
pub type HSIASFS_R = crate::BitReader<HSIASFS_A>;
#[doc = "HSI16 automatic start from Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIASFS_A {
    #[doc = "0: HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock"]
    Disabled = 0,
    #[doc = "1: HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock"]
    Enabled = 1,
}
impl From<HSIASFS_A> for bool {
    #[inline(always)]
    fn from(variant: HSIASFS_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIASFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIASFS_A {
        match self.bits {
            false => HSIASFS_A::Disabled,
            true => HSIASFS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIASFS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIASFS_A::Enabled
    }
}
#[doc = "Field `HSIASFS` writer - HSI16 automatic start from Stop"]
pub type HSIASFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSIASFS_A, O>;
impl<'a, const O: u8> HSIASFS_W<'a, O> {
    #[doc = "HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIASFS_A::Disabled)
    }
    #[doc = "HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIASFS_A::Enabled)
    }
}
#[doc = "Field `HSIKERDY` reader - HSI16 kernel clock ready flag for peripherals requests."]
pub type HSIKERDY_R = crate::BitReader<HSIKERDY_A>;
#[doc = "HSI16 kernel clock ready flag for peripherals requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERDY_A {
    #[doc = "0: HSI16 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI16 oscillator ready"]
    Ready = 1,
}
impl From<HSIKERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIKERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIKERDY_A {
        match self.bits {
            false => HSIKERDY_A::NotReady,
            true => HSIKERDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIKERDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIKERDY_A::Ready
    }
}
#[doc = "Field `HSEON` reader - HSE32 clock enable"]
pub type HSEON_R = crate::BitReader<HSEON_A>;
#[doc = "HSE32 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON_A {
    #[doc = "0: HSE32 oscillator for CPU disabled"]
    Disabled = 0,
    #[doc = "1: HSE32 oscillator for CPU enabled"]
    Enabled = 1,
}
impl From<HSEON_A> for bool {
    #[inline(always)]
    fn from(variant: HSEON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEON_A {
        match self.bits {
            false => HSEON_A::Disabled,
            true => HSEON_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSEON_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSEON_A::Enabled
    }
}
#[doc = "Field `HSEON` writer - HSE32 clock enable"]
pub type HSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEON_A, O>;
impl<'a, const O: u8> HSEON_W<'a, O> {
    #[doc = "HSE32 oscillator for CPU disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSEON_A::Disabled)
    }
    #[doc = "HSE32 oscillator for CPU enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSEON_A::Enabled)
    }
}
#[doc = "Field `HSERDY` reader - HSE32 clock ready flag"]
pub type HSERDY_R = crate::BitReader<HSERDY_A>;
#[doc = "HSE32 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY_A {
    #[doc = "0: HSE32 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSE32 oscillator ready"]
    Ready = 1,
}
impl From<HSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDY_A {
        match self.bits {
            false => HSERDY_A::NotReady,
            true => HSERDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSERDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSERDY_A::Ready
    }
}
#[doc = "Field `CSSON` reader - HSE32 Clock security system enable"]
pub type CSSON_R = crate::BitReader<CSSON_A>;
#[doc = "HSE32 Clock security system enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON_A {
    #[doc = "0: HSE32 CSS off"]
    Disabled = 0,
    #[doc = "1: HSE32 CSS on if the HSE32 oscillator is stable and off if not"]
    Enabled = 1,
}
impl From<CSSON_A> for bool {
    #[inline(always)]
    fn from(variant: CSSON_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSON_A {
        match self.bits {
            false => CSSON_A::Disabled,
            true => CSSON_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSON_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSON_A::Enabled
    }
}
#[doc = "Field `CSSON` writer - HSE32 Clock security system enable"]
pub type CSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CSSON_A, O>;
impl<'a, const O: u8> CSSON_W<'a, O> {
    #[doc = "HSE32 CSS off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSSON_A::Disabled)
    }
    #[doc = "HSE32 CSS on if the HSE32 oscillator is stable and off if not"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSSON_A::Enabled)
    }
}
#[doc = "Field `HSEPRE` reader - HSE32 sysclk prescaler"]
pub type HSEPRE_R = crate::BitReader<HSEPRE_A>;
#[doc = "HSE32 sysclk prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEPRE_A {
    #[doc = "0: SYSCLK not divided (HSE32)"]
    Div1 = 0,
    #[doc = "1: SYSCLK divided by two (HSE32/2)"]
    Div2 = 1,
}
impl From<HSEPRE_A> for bool {
    #[inline(always)]
    fn from(variant: HSEPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEPRE_A {
        match self.bits {
            false => HSEPRE_A::Div1,
            true => HSEPRE_A::Div2,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSEPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSEPRE_A::Div2
    }
}
#[doc = "Field `HSEPRE` writer - HSE32 sysclk prescaler"]
pub type HSEPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEPRE_A, O>;
impl<'a, const O: u8> HSEPRE_W<'a, O> {
    #[doc = "SYSCLK not divided (HSE32)"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HSEPRE_A::Div1)
    }
    #[doc = "SYSCLK divided by two (HSE32/2)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HSEPRE_A::Div2)
    }
}
#[doc = "Field `HSEBYPPWR` reader - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
pub type HSEBYPPWR_R = crate::BitReader<HSEBYPPWR_A>;
#[doc = "Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYPPWR_A {
    #[doc = "0: PB0 selected"]
    Pb0 = 0,
    #[doc = "1: VDDTCXO selected"]
    Vddtcxo = 1,
}
impl From<HSEBYPPWR_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYPPWR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEBYPPWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEBYPPWR_A {
        match self.bits {
            false => HSEBYPPWR_A::Pb0,
            true => HSEBYPPWR_A::Vddtcxo,
        }
    }
    #[doc = "Checks if the value of the field is `Pb0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == HSEBYPPWR_A::Pb0
    }
    #[doc = "Checks if the value of the field is `Vddtcxo`"]
    #[inline(always)]
    pub fn is_vddtcxo(&self) -> bool {
        *self == HSEBYPPWR_A::Vddtcxo
    }
}
#[doc = "Field `HSEBYPPWR` writer - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
pub type HSEBYPPWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEBYPPWR_A, O>;
impl<'a, const O: u8> HSEBYPPWR_W<'a, O> {
    #[doc = "PB0 selected"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(HSEBYPPWR_A::Pb0)
    }
    #[doc = "VDDTCXO selected"]
    #[inline(always)]
    pub fn vddtcxo(self) -> &'a mut W {
        self.variant(HSEBYPPWR_A::Vddtcxo)
    }
}
#[doc = "Field `PLLON` reader - Main PLL enable"]
pub type PLLON_R = crate::BitReader<PLLON_A>;
#[doc = "Main PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLON_A {
    #[doc = "0: Main PLL Off"]
    Off = 0,
    #[doc = "1: Main PLL On"]
    On = 1,
}
impl From<PLLON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLON_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLON_A {
        match self.bits {
            false => PLLON_A::Off,
            true => PLLON_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PLLON_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PLLON_A::On
    }
}
#[doc = "Field `PLLON` writer - Main PLL enable"]
pub type PLLON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PLLON_A, O>;
impl<'a, const O: u8> PLLON_W<'a, O> {
    #[doc = "Main PLL Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLON_A::Off)
    }
    #[doc = "Main PLL On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLON_A::On)
    }
}
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader<PLLRDY_A>;
#[doc = "Main PLL clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDY_A {
    #[doc = "0: PLL unlocked"]
    Unlocked = 0,
    #[doc = "1: PLL Locked"]
    Locked = 1,
}
impl From<PLLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDY_A {
        match self.bits {
            false => PLLRDY_A::Unlocked,
            true => PLLRDY_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDY_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDY_A::Locked
    }
}
impl R {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSI range control selection"]
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernel clocks."]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HSI16 kernel clock ready flag for peripherals requests."]
    #[inline(always)]
    pub fn hsikerdy(&self) -> HSIKERDY_R {
        HSIKERDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE32 clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE32 clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE32 Clock security system enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HSE32 sysclk prescaler"]
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
    #[inline(always)]
    pub fn hsebyppwr(&self) -> HSEBYPPWR_R {
        HSEBYPPWR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<0> {
        MSION_W::new(self)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<2> {
        MSIPLLEN_W::new(self)
    }
    #[doc = "Bit 3 - MSI range control selection"]
    #[inline(always)]
    #[must_use]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<3> {
        MSIRGSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<4> {
        MSIRANGE_W::new(self)
    }
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<8> {
        HSION_W::new(self)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernel clocks."]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<9> {
        HSIKERON_W::new(self)
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop"]
    #[inline(always)]
    #[must_use]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<11> {
        HSIASFS_W::new(self)
    }
    #[doc = "Bit 16 - HSE32 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    #[doc = "Bit 19 - HSE32 Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<19> {
        CSSON_W::new(self)
    }
    #[doc = "Bit 20 - HSE32 sysclk prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hsepre(&mut self) -> HSEPRE_W<20> {
        HSEPRE_W::new(self)
    }
    #[doc = "Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
    #[inline(always)]
    #[must_use]
    pub fn hsebyppwr(&mut self) -> HSEBYPPWR_W<21> {
        HSEBYPPWR_W::new(self)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<24> {
        PLLON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x61"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x61;
}
