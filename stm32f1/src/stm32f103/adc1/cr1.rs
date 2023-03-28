#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWDCH` reader - Analog watchdog channel select bits"]
pub type AWDCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWDCH` writer - Analog watchdog channel select bits"]
pub type AWDCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EOCIE_R = crate::BitReader<EOCIE_A>;
#[doc = "Interrupt enable for EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE_A {
    #[doc = "0: EOC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    Enabled = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::Disabled,
            true => EOCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE_A::Enabled
    }
}
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, EOCIE_A, O>;
impl<'a, const O: u8> EOCIE_W<'a, O> {
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIE_A::Disabled)
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIE_A::Enabled)
    }
}
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AWDIE_R = crate::BitReader<AWDIE_A>;
#[doc = "Analog watchdog interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDIE_A {
    #[doc = "0: Analog watchdog interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog interrupt enabled"]
    Enabled = 1,
}
impl From<AWDIE_A> for bool {
    #[inline(always)]
    fn from(variant: AWDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AWDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDIE_A {
        match self.bits {
            false => AWDIE_A::Disabled,
            true => AWDIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE_A::Enabled
    }
}
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AWDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, AWDIE_A, O>;
impl<'a, const O: u8> AWDIE_W<'a, O> {
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDIE_A::Disabled)
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDIE_A::Enabled)
    }
}
#[doc = "Field `JEOCIE` reader - Interrupt enable for injected channels"]
pub type JEOCIE_R = crate::BitReader<JEOCIE_A>;
#[doc = "Interrupt enable for injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE_A {
    #[doc = "0: JEOC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set"]
    Enabled = 1,
}
impl From<JEOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCIE_A {
        match self.bits {
            false => JEOCIE_A::Disabled,
            true => JEOCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIE_A::Enabled
    }
}
#[doc = "Field `JEOCIE` writer - Interrupt enable for injected channels"]
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, JEOCIE_A, O>;
impl<'a, const O: u8> JEOCIE_W<'a, O> {
    #[doc = "JEOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::Disabled)
    }
    #[doc = "JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::Enabled)
    }
}
#[doc = "Field `SCAN` reader - Scan mode"]
pub type SCAN_R = crate::BitReader<SCAN_A>;
#[doc = "Scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCAN_A {
    #[doc = "0: Scan mode disabled"]
    Disabled = 0,
    #[doc = "1: Scan mode enabled"]
    Enabled = 1,
}
impl From<SCAN_A> for bool {
    #[inline(always)]
    fn from(variant: SCAN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCAN_A {
        match self.bits {
            false => SCAN_A::Disabled,
            true => SCAN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCAN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCAN_A::Enabled
    }
}
#[doc = "Field `SCAN` writer - Scan mode"]
pub type SCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SCAN_A, O>;
impl<'a, const O: u8> SCAN_W<'a, O> {
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCAN_A::Disabled)
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCAN_A::Enabled)
    }
}
#[doc = "Field `AWDSGL` reader - Enable the watchdog on a single channel in scan mode"]
pub type AWDSGL_R = crate::BitReader<AWDSGL_A>;
#[doc = "Enable the watchdog on a single channel in scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDSGL_A {
    #[doc = "0: Analog watchdog enabled on all channels"]
    All = 0,
    #[doc = "1: Analog watchdog enabled on a single channel"]
    Single = 1,
}
impl From<AWDSGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWDSGL_A) -> Self {
        variant as u8 != 0
    }
}
impl AWDSGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDSGL_A {
        match self.bits {
            false => AWDSGL_A::All,
            true => AWDSGL_A::Single,
        }
    }
    #[doc = "Checks if the value of the field is `All`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == AWDSGL_A::All
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AWDSGL_A::Single
    }
}
#[doc = "Field `AWDSGL` writer - Enable the watchdog on a single channel in scan mode"]
pub type AWDSGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, AWDSGL_A, O>;
impl<'a, const O: u8> AWDSGL_W<'a, O> {
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(AWDSGL_A::All)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(AWDSGL_A::Single)
    }
}
#[doc = "Field `JAUTO` reader - Automatic injected group conversion"]
pub type JAUTO_R = crate::BitReader<JAUTO_A>;
#[doc = "Automatic injected group conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO_A {
    #[doc = "0: Automatic injected group conversion disabled"]
    Disabled = 0,
    #[doc = "1: Automatic injected group conversion enabled"]
    Enabled = 1,
}
impl From<JAUTO_A> for bool {
    #[inline(always)]
    fn from(variant: JAUTO_A) -> Self {
        variant as u8 != 0
    }
}
impl JAUTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAUTO_A {
        match self.bits {
            false => JAUTO_A::Disabled,
            true => JAUTO_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO_A::Enabled
    }
}
#[doc = "Field `JAUTO` writer - Automatic injected group conversion"]
pub type JAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, JAUTO_A, O>;
impl<'a, const O: u8> JAUTO_W<'a, O> {
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAUTO_A::Disabled)
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAUTO_A::Enabled)
    }
}
#[doc = "Field `DISCEN` reader - Discontinuous mode on regular channels"]
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
#[doc = "Discontinuous mode on regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN_A {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    Enabled = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::Disabled,
            true => DISCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN_A::Enabled
    }
}
#[doc = "Field `DISCEN` writer - Discontinuous mode on regular channels"]
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, DISCEN_A, O>;
impl<'a, const O: u8> DISCEN_W<'a, O> {
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Disabled)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Enabled)
    }
}
#[doc = "Field `JDISCEN` reader - Discontinuous mode on injected channels"]
pub type JDISCEN_R = crate::BitReader<JDISCEN_A>;
#[doc = "Discontinuous mode on injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN_A {
    #[doc = "0: Discontinuous mode on injected channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on injected channels enabled"]
    Enabled = 1,
}
impl From<JDISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl JDISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JDISCEN_A {
        match self.bits {
            false => JDISCEN_A::Disabled,
            true => JDISCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN_A::Enabled
    }
}
#[doc = "Field `JDISCEN` writer - Discontinuous mode on injected channels"]
pub type JDISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, JDISCEN_A, O>;
impl<'a, const O: u8> JDISCEN_W<'a, O> {
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::Disabled)
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::Enabled)
    }
}
#[doc = "Field `DISCNUM` reader - Discontinuous mode channel count"]
pub type DISCNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISCNUM` writer - Discontinuous mode channel count"]
pub type DISCNUM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `DUALMOD` reader - Dual mode selection"]
pub type DUALMOD_R = crate::FieldReader<u8, DUALMOD_A>;
#[doc = "Dual mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUALMOD_A {
    #[doc = "0: Independent mode"]
    Independent = 0,
    #[doc = "1: Combined regular simultaneous + injected simultaneous mode"]
    RegularInjected = 1,
    #[doc = "2: Combined regular simultaneous + alternate trigger mode"]
    RegularAlternateTrigger = 2,
    #[doc = "3: Combined injected simultaneous + fast interleaved mode"]
    InjectedFastInterleaved = 3,
    #[doc = "4: Combined injected simultaneous + slow interleaved mode"]
    InjectedSlowInterleaved = 4,
    #[doc = "5: Injected simultaneous mode only"]
    Injected = 5,
    #[doc = "6: Regular simultaneous mode only"]
    Regular = 6,
    #[doc = "7: Fast interleaved mode only"]
    FastInterleaved = 7,
    #[doc = "8: Slow interleaved mode only"]
    SlowInterleaved = 8,
    #[doc = "9: Alternate trigger mode only"]
    AlternateTrigger = 9,
}
impl From<DUALMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: DUALMOD_A) -> Self {
        variant as _
    }
}
impl DUALMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DUALMOD_A> {
        match self.bits {
            0 => Some(DUALMOD_A::Independent),
            1 => Some(DUALMOD_A::RegularInjected),
            2 => Some(DUALMOD_A::RegularAlternateTrigger),
            3 => Some(DUALMOD_A::InjectedFastInterleaved),
            4 => Some(DUALMOD_A::InjectedSlowInterleaved),
            5 => Some(DUALMOD_A::Injected),
            6 => Some(DUALMOD_A::Regular),
            7 => Some(DUALMOD_A::FastInterleaved),
            8 => Some(DUALMOD_A::SlowInterleaved),
            9 => Some(DUALMOD_A::AlternateTrigger),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Independent`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUALMOD_A::Independent
    }
    #[doc = "Checks if the value of the field is `RegularInjected`"]
    #[inline(always)]
    pub fn is_regular_injected(&self) -> bool {
        *self == DUALMOD_A::RegularInjected
    }
    #[doc = "Checks if the value of the field is `RegularAlternateTrigger`"]
    #[inline(always)]
    pub fn is_regular_alternate_trigger(&self) -> bool {
        *self == DUALMOD_A::RegularAlternateTrigger
    }
    #[doc = "Checks if the value of the field is `InjectedFastInterleaved`"]
    #[inline(always)]
    pub fn is_injected_fast_interleaved(&self) -> bool {
        *self == DUALMOD_A::InjectedFastInterleaved
    }
    #[doc = "Checks if the value of the field is `InjectedSlowInterleaved`"]
    #[inline(always)]
    pub fn is_injected_slow_interleaved(&self) -> bool {
        *self == DUALMOD_A::InjectedSlowInterleaved
    }
    #[doc = "Checks if the value of the field is `Injected`"]
    #[inline(always)]
    pub fn is_injected(&self) -> bool {
        *self == DUALMOD_A::Injected
    }
    #[doc = "Checks if the value of the field is `Regular`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == DUALMOD_A::Regular
    }
    #[doc = "Checks if the value of the field is `FastInterleaved`"]
    #[inline(always)]
    pub fn is_fast_interleaved(&self) -> bool {
        *self == DUALMOD_A::FastInterleaved
    }
    #[doc = "Checks if the value of the field is `SlowInterleaved`"]
    #[inline(always)]
    pub fn is_slow_interleaved(&self) -> bool {
        *self == DUALMOD_A::SlowInterleaved
    }
    #[doc = "Checks if the value of the field is `AlternateTrigger`"]
    #[inline(always)]
    pub fn is_alternate_trigger(&self) -> bool {
        *self == DUALMOD_A::AlternateTrigger
    }
}
#[doc = "Field `DUALMOD` writer - Dual mode selection"]
pub type DUALMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, DUALMOD_A, 4, O>;
impl<'a, const O: u8> DUALMOD_W<'a, O> {
    #[doc = "Independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(DUALMOD_A::Independent)
    }
    #[doc = "Combined regular simultaneous + injected simultaneous mode"]
    #[inline(always)]
    pub fn regular_injected(self) -> &'a mut W {
        self.variant(DUALMOD_A::RegularInjected)
    }
    #[doc = "Combined regular simultaneous + alternate trigger mode"]
    #[inline(always)]
    pub fn regular_alternate_trigger(self) -> &'a mut W {
        self.variant(DUALMOD_A::RegularAlternateTrigger)
    }
    #[doc = "Combined injected simultaneous + fast interleaved mode"]
    #[inline(always)]
    pub fn injected_fast_interleaved(self) -> &'a mut W {
        self.variant(DUALMOD_A::InjectedFastInterleaved)
    }
    #[doc = "Combined injected simultaneous + slow interleaved mode"]
    #[inline(always)]
    pub fn injected_slow_interleaved(self) -> &'a mut W {
        self.variant(DUALMOD_A::InjectedSlowInterleaved)
    }
    #[doc = "Injected simultaneous mode only"]
    #[inline(always)]
    pub fn injected(self) -> &'a mut W {
        self.variant(DUALMOD_A::Injected)
    }
    #[doc = "Regular simultaneous mode only"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(DUALMOD_A::Regular)
    }
    #[doc = "Fast interleaved mode only"]
    #[inline(always)]
    pub fn fast_interleaved(self) -> &'a mut W {
        self.variant(DUALMOD_A::FastInterleaved)
    }
    #[doc = "Slow interleaved mode only"]
    #[inline(always)]
    pub fn slow_interleaved(self) -> &'a mut W {
        self.variant(DUALMOD_A::SlowInterleaved)
    }
    #[doc = "Alternate trigger mode only"]
    #[inline(always)]
    pub fn alternate_trigger(self) -> &'a mut W {
        self.variant(DUALMOD_A::AlternateTrigger)
    }
}
#[doc = "Field `JAWDEN` reader - Analog watchdog enable on injected channels"]
pub type JAWDEN_R = crate::BitReader<JAWDEN_A>;
#[doc = "Analog watchdog enable on injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWDEN_A {
    #[doc = "0: Analog watchdog disabled on injected channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog enabled on injected channels"]
    Enabled = 1,
}
impl From<JAWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: JAWDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl JAWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAWDEN_A {
        match self.bits {
            false => JAWDEN_A::Disabled,
            true => JAWDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWDEN_A::Enabled
    }
}
#[doc = "Field `JAWDEN` writer - Analog watchdog enable on injected channels"]
pub type JAWDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, JAWDEN_A, O>;
impl<'a, const O: u8> JAWDEN_W<'a, O> {
    #[doc = "Analog watchdog disabled on injected channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAWDEN_A::Disabled)
    }
    #[doc = "Analog watchdog enabled on injected channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAWDEN_A::Enabled)
    }
}
#[doc = "Field `AWDEN` reader - Analog watchdog enable on regular channels"]
pub type AWDEN_R = crate::BitReader<AWDEN_A>;
#[doc = "Analog watchdog enable on regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDEN_A {
    #[doc = "0: Analog watchdog disabled on regular channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog enabled on regular channels"]
    Enabled = 1,
}
impl From<AWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: AWDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDEN_A {
        match self.bits {
            false => AWDEN_A::Disabled,
            true => AWDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDEN_A::Enabled
    }
}
#[doc = "Field `AWDEN` writer - Analog watchdog enable on regular channels"]
pub type AWDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, AWDEN_A, O>;
impl<'a, const O: u8> AWDEN_W<'a, O> {
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDEN_A::Disabled)
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Dual mode selection"]
    #[inline(always)]
    pub fn dualmod(&self) -> DUALMOD_R {
        DUALMOD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<0> {
        AWDCH_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<5> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<6> {
        AWDIE_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<7> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<8> {
        SCAN_W::new(self)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn awdsgl(&mut self) -> AWDSGL_W<9> {
        AWDSGL_W::new(self)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<10> {
        JAUTO_W::new(self)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<11> {
        DISCEN_W::new(self)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<12> {
        JDISCEN_W::new(self)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<13> {
        DISCNUM_W::new(self)
    }
    #[doc = "Bits 16:19 - Dual mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn dualmod(&mut self) -> DUALMOD_W<16> {
        DUALMOD_W::new(self)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jawden(&mut self) -> JAWDEN_W<22> {
        JAWDEN_W::new(self)
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn awden(&mut self) -> AWDEN_W<23> {
        AWDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
