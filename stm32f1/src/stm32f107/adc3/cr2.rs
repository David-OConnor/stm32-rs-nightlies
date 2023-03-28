#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADON` reader - A/D converter ON / OFF"]
pub type ADON_R = crate::BitReader<ADON_A>;
#[doc = "A/D converter ON / OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADON_A {
    #[doc = "0: Disable ADC conversion/calibration and go to power down mode"]
    Disabled = 0,
    #[doc = "1: Enable ADC and to start conversion"]
    Enabled = 1,
}
impl From<ADON_A> for bool {
    #[inline(always)]
    fn from(variant: ADON_A) -> Self {
        variant as u8 != 0
    }
}
impl ADON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADON_A {
        match self.bits {
            false => ADON_A::Disabled,
            true => ADON_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADON_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADON_A::Enabled
    }
}
#[doc = "Field `ADON` writer - A/D converter ON / OFF"]
pub type ADON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ADON_A, O>;
impl<'a, const O: u8> ADON_W<'a, O> {
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADON_A::Disabled)
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADON_A::Enabled)
    }
}
#[doc = "Field `CONT` reader - Continuous conversion"]
pub type CONT_R = crate::BitReader<CONT_A>;
#[doc = "Continuous conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    Single = 0,
    #[doc = "1: Continuous conversion mode"]
    Continuous = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::Single,
            true => CONT_A::Continuous,
        }
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT_A::Single
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::Continuous
    }
}
#[doc = "Field `CONT` writer - Continuous conversion"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::Single)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::Continuous)
    }
}
#[doc = "Field `CAL` reader - A/D calibration"]
pub type CAL_R = crate::BitReader<CALR_A>;
#[doc = "A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALR_A {
    #[doc = "0: Calibration completed"]
    Complete = 0,
    #[doc = "1: Calibrating"]
    NotComplete = 1,
}
impl From<CALR_A> for bool {
    #[inline(always)]
    fn from(variant: CALR_A) -> Self {
        variant as u8 != 0
    }
}
impl CAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALR_A {
        match self.bits {
            false => CALR_A::Complete,
            true => CALR_A::NotComplete,
        }
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CALR_A::Complete
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CALR_A::NotComplete
    }
}
#[doc = "A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW_AW {
    #[doc = "1: Enable calibration"]
    Start = 1,
}
impl From<CALW_AW> for bool {
    #[inline(always)]
    fn from(variant: CALW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` writer - A/D calibration"]
pub type CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CALW_AW, O>;
impl<'a, const O: u8> CAL_W<'a, O> {
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CALW_AW::Start)
    }
}
#[doc = "Field `RSTCAL` reader - Reset calibration"]
pub type RSTCAL_R = crate::BitReader<RSTCALR_A>;
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCALR_A {
    #[doc = "0: Calibration register initialized"]
    Initialized = 0,
    #[doc = "1: Initializing calibration register"]
    NotInitialized = 1,
}
impl From<RSTCALR_A> for bool {
    #[inline(always)]
    fn from(variant: RSTCALR_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTCALR_A {
        match self.bits {
            false => RSTCALR_A::Initialized,
            true => RSTCALR_A::NotInitialized,
        }
    }
    #[doc = "Checks if the value of the field is `Initialized`"]
    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        *self == RSTCALR_A::Initialized
    }
    #[doc = "Checks if the value of the field is `NotInitialized`"]
    #[inline(always)]
    pub fn is_not_initialized(&self) -> bool {
        *self == RSTCALR_A::NotInitialized
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCALW_AW {
    #[doc = "1: Initialize calibration register"]
    Initialize = 1,
}
impl From<RSTCALW_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTCALW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCAL` writer - Reset calibration"]
pub type RSTCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RSTCALW_AW, O>;
impl<'a, const O: u8> RSTCAL_W<'a, O> {
    #[doc = "Initialize calibration register"]
    #[inline(always)]
    pub fn initialize(self) -> &'a mut W {
        self.variant(RSTCALW_AW::Initialize)
    }
}
#[doc = "Field `DMA` reader - Direct memory access mode"]
pub type DMA_R = crate::BitReader<DMA_A>;
#[doc = "Direct memory access mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    #[doc = "0: DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DMA mode enabled"]
    Enabled = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::Disabled,
            true => DMA_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_A::Enabled
    }
}
#[doc = "Field `DMA` writer - Direct memory access mode"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, DMA_A, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::Disabled)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_A::Enabled)
    }
}
#[doc = "Field `ALIGN` reader - Data alignment"]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    #[doc = "0: Right Alignment"]
    Right = 0,
    #[doc = "1: Left Alignment"]
    Left = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::Right,
            true => ALIGN_A::Left,
        }
    }
    #[doc = "Checks if the value of the field is `Right`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::Right
    }
    #[doc = "Checks if the value of the field is `Left`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::Left
    }
}
#[doc = "Field `ALIGN` writer - Data alignment"]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ALIGN_A, O>;
impl<'a, const O: u8> ALIGN_W<'a, O> {
    #[doc = "Right Alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::Right)
    }
    #[doc = "Left Alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::Left)
    }
}
#[doc = "Field `JEXTSEL` reader - External event select for injected group"]
pub type JEXTSEL_R = crate::FieldReader<u8, JEXTSEL_A>;
#[doc = "External event select for injected group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    #[doc = "0: Timer 1 TRGO event"]
    Tim1trgo = 0,
    #[doc = "1: Timer 1 CC4 event"]
    Tim1cc4 = 1,
    #[doc = "2: Timer 4 CC3 event"]
    Tim4cc3 = 2,
    #[doc = "3: Timer 8 CC2 event"]
    Tim8cc2 = 3,
    #[doc = "4: Timer 8 CC4 event"]
    Tim8cc4 = 4,
    #[doc = "5: Timer 5 TRGO event"]
    Tim5trgo = 5,
    #[doc = "6: Timer 5 CC4 event"]
    Tim5cc4 = 6,
    #[doc = "7: JSWSTART"]
    Jswstart = 7,
}
impl From<JEXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL_A) -> Self {
        variant as _
    }
}
impl JEXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTSEL_A {
        match self.bits {
            0 => JEXTSEL_A::Tim1trgo,
            1 => JEXTSEL_A::Tim1cc4,
            2 => JEXTSEL_A::Tim4cc3,
            3 => JEXTSEL_A::Tim8cc2,
            4 => JEXTSEL_A::Tim8cc4,
            5 => JEXTSEL_A::Tim5trgo,
            6 => JEXTSEL_A::Tim5cc4,
            7 => JEXTSEL_A::Jswstart,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Tim1trgo`"]
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim1trgo
    }
    #[doc = "Checks if the value of the field is `Tim1cc4`"]
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim1cc4
    }
    #[doc = "Checks if the value of the field is `Tim4cc3`"]
    #[inline(always)]
    pub fn is_tim4cc3(&self) -> bool {
        *self == JEXTSEL_A::Tim4cc3
    }
    #[doc = "Checks if the value of the field is `Tim8cc2`"]
    #[inline(always)]
    pub fn is_tim8cc2(&self) -> bool {
        *self == JEXTSEL_A::Tim8cc2
    }
    #[doc = "Checks if the value of the field is `Tim8cc4`"]
    #[inline(always)]
    pub fn is_tim8cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim8cc4
    }
    #[doc = "Checks if the value of the field is `Tim5trgo`"]
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim5trgo
    }
    #[doc = "Checks if the value of the field is `Tim5cc4`"]
    #[inline(always)]
    pub fn is_tim5cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim5cc4
    }
    #[doc = "Checks if the value of the field is `Jswstart`"]
    #[inline(always)]
    pub fn is_jswstart(&self) -> bool {
        *self == JEXTSEL_A::Jswstart
    }
}
#[doc = "Field `JEXTSEL` writer - External event select for injected group"]
pub type JEXTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, JEXTSEL_A, 3, O>;
impl<'a, const O: u8> JEXTSEL_W<'a, O> {
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1trgo)
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1cc4)
    }
    #[doc = "Timer 4 CC3 event"]
    #[inline(always)]
    pub fn tim4cc3(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim4cc3)
    }
    #[doc = "Timer 8 CC2 event"]
    #[inline(always)]
    pub fn tim8cc2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim8cc2)
    }
    #[doc = "Timer 8 CC4 event"]
    #[inline(always)]
    pub fn tim8cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim8cc4)
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim5trgo)
    }
    #[doc = "Timer 5 CC4 event"]
    #[inline(always)]
    pub fn tim5cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim5cc4)
    }
    #[doc = "JSWSTART"]
    #[inline(always)]
    pub fn jswstart(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Jswstart)
    }
}
#[doc = "Field `JEXTTRIG` reader - External trigger conversion mode for injected channels"]
pub type JEXTTRIG_R = crate::BitReader<JEXTTRIG_A>;
#[doc = "External trigger conversion mode for injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEXTTRIG_A {
    #[doc = "0: Conversion on external event disabled"]
    Disabled = 0,
    #[doc = "1: Conversion on external event enabled"]
    Enabled = 1,
}
impl From<JEXTTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: JEXTTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl JEXTTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTTRIG_A {
        match self.bits {
            false => JEXTTRIG_A::Disabled,
            true => JEXTTRIG_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTTRIG_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEXTTRIG_A::Enabled
    }
}
#[doc = "Field `JEXTTRIG` writer - External trigger conversion mode for injected channels"]
pub type JEXTTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, JEXTTRIG_A, O>;
impl<'a, const O: u8> JEXTTRIG_W<'a, O> {
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTTRIG_A::Disabled)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEXTTRIG_A::Enabled)
    }
}
#[doc = "Field `EXTSEL` reader - External event select for regular group"]
pub type EXTSEL_R = crate::FieldReader<u8, EXTSEL_A>;
#[doc = "External event select for regular group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "0: Timer 3 CC1 event"]
    Tim3cc1 = 0,
    #[doc = "1: Timer 2 CC3 event"]
    Tim2cc3 = 1,
    #[doc = "2: Timer 1 CC3 event"]
    Tim1cc3 = 2,
    #[doc = "3: Timer 8 CC1 event"]
    Tim8cc1 = 3,
    #[doc = "4: Timer 8 TRGO event"]
    Tim8trgo = 4,
    #[doc = "5: Timer 5 CC1 event"]
    Tim5cc1 = 5,
    #[doc = "6: Timer 5 CC3 event"]
    Tim5cc3 = 6,
    #[doc = "7: SWSTART"]
    Swstart = 7,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSEL_A {
        match self.bits {
            0 => EXTSEL_A::Tim3cc1,
            1 => EXTSEL_A::Tim2cc3,
            2 => EXTSEL_A::Tim1cc3,
            3 => EXTSEL_A::Tim8cc1,
            4 => EXTSEL_A::Tim8trgo,
            5 => EXTSEL_A::Tim5cc1,
            6 => EXTSEL_A::Tim5cc3,
            7 => EXTSEL_A::Swstart,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Tim3cc1`"]
    #[inline(always)]
    pub fn is_tim3cc1(&self) -> bool {
        *self == EXTSEL_A::Tim3cc1
    }
    #[doc = "Checks if the value of the field is `Tim2cc3`"]
    #[inline(always)]
    pub fn is_tim2cc3(&self) -> bool {
        *self == EXTSEL_A::Tim2cc3
    }
    #[doc = "Checks if the value of the field is `Tim1cc3`"]
    #[inline(always)]
    pub fn is_tim1cc3(&self) -> bool {
        *self == EXTSEL_A::Tim1cc3
    }
    #[doc = "Checks if the value of the field is `Tim8cc1`"]
    #[inline(always)]
    pub fn is_tim8cc1(&self) -> bool {
        *self == EXTSEL_A::Tim8cc1
    }
    #[doc = "Checks if the value of the field is `Tim8trgo`"]
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        *self == EXTSEL_A::Tim8trgo
    }
    #[doc = "Checks if the value of the field is `Tim5cc1`"]
    #[inline(always)]
    pub fn is_tim5cc1(&self) -> bool {
        *self == EXTSEL_A::Tim5cc1
    }
    #[doc = "Checks if the value of the field is `Tim5cc3`"]
    #[inline(always)]
    pub fn is_tim5cc3(&self) -> bool {
        *self == EXTSEL_A::Tim5cc3
    }
    #[doc = "Checks if the value of the field is `Swstart`"]
    #[inline(always)]
    pub fn is_swstart(&self) -> bool {
        *self == EXTSEL_A::Swstart
    }
}
#[doc = "Field `EXTSEL` writer - External event select for regular group"]
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, EXTSEL_A, 3, O>;
impl<'a, const O: u8> EXTSEL_W<'a, O> {
    #[doc = "Timer 3 CC1 event"]
    #[inline(always)]
    pub fn tim3cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim3cc1)
    }
    #[doc = "Timer 2 CC3 event"]
    #[inline(always)]
    pub fn tim2cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2cc3)
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline(always)]
    pub fn tim1cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1cc3)
    }
    #[doc = "Timer 8 CC1 event"]
    #[inline(always)]
    pub fn tim8cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim8cc1)
    }
    #[doc = "Timer 8 TRGO event"]
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim8trgo)
    }
    #[doc = "Timer 5 CC1 event"]
    #[inline(always)]
    pub fn tim5cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim5cc1)
    }
    #[doc = "Timer 5 CC3 event"]
    #[inline(always)]
    pub fn tim5cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim5cc3)
    }
    #[doc = "SWSTART"]
    #[inline(always)]
    pub fn swstart(self) -> &'a mut W {
        self.variant(EXTSEL_A::Swstart)
    }
}
#[doc = "Field `EXTTRIG` reader - External trigger conversion mode for regular channels"]
pub type EXTTRIG_R = crate::BitReader<EXTTRIG_A>;
#[doc = "External trigger conversion mode for regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIG_A {
    #[doc = "0: Conversion on external event disabled"]
    Disabled = 0,
    #[doc = "1: Conversion on external event enabled"]
    Enabled = 1,
}
impl From<EXTTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTTRIG_A {
        match self.bits {
            false => EXTTRIG_A::Disabled,
            true => EXTTRIG_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTTRIG_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTTRIG_A::Enabled
    }
}
#[doc = "Field `EXTTRIG` writer - External trigger conversion mode for regular channels"]
pub type EXTTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, EXTTRIG_A, O>;
impl<'a, const O: u8> EXTTRIG_W<'a, O> {
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTTRIG_A::Disabled)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTTRIG_A::Enabled)
    }
}
#[doc = "Field `JSWSTART` reader - Start conversion of injected channels"]
pub type JSWSTART_R = crate::BitReader<JSWSTARTR_A>;
#[doc = "Start conversion of injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTR_A {
    #[doc = "0: Reset state"]
    Started = 0,
    #[doc = "1: Starting conversion of injected channels"]
    NotStarted = 1,
}
impl From<JSWSTARTR_A> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl JSWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSWSTARTR_A {
        match self.bits {
            false => JSWSTARTR_A::Started,
            true => JSWSTARTR_A::NotStarted,
        }
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSWSTARTR_A::Started
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSWSTARTR_A::NotStarted
    }
}
#[doc = "Start conversion of injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW_AW {
    #[doc = "1: Start conversion of injected channels"]
    Start = 1,
}
impl From<JSWSTARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSWSTART` writer - Start conversion of injected channels"]
pub type JSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, JSWSTARTW_AW, O>;
impl<'a, const O: u8> JSWSTART_W<'a, O> {
    #[doc = "Start conversion of injected channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JSWSTARTW_AW::Start)
    }
}
#[doc = "Field `SWSTART` reader - Start conversion of regular channels"]
pub type SWSTART_R = crate::BitReader<SWSTARTR_A>;
#[doc = "Start conversion of regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTR_A {
    #[doc = "0: Reset state"]
    Started = 0,
    #[doc = "1: Starting conversion of regular channels"]
    NotStarted = 1,
}
impl From<SWSTARTR_A> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSTARTR_A {
        match self.bits {
            false => SWSTARTR_A::Started,
            true => SWSTARTR_A::NotStarted,
        }
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SWSTARTR_A::Started
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == SWSTARTR_A::NotStarted
    }
}
#[doc = "Start conversion of regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTW_AW {
    #[doc = "1: Start conversion of regular channels"]
    Start = 1,
}
impl From<SWSTARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSTART` writer - Start conversion of regular channels"]
pub type SWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, SWSTARTW_AW, O>;
impl<'a, const O: u8> SWSTART_W<'a, O> {
    #[doc = "Start conversion of regular channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SWSTARTW_AW::Start)
    }
}
#[doc = "Field `TSVREFE` reader - Temperature sensor and VREFINT enable"]
pub type TSVREFE_R = crate::BitReader<TSVREFE_A>;
#[doc = "Temperature sensor and VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSVREFE_A {
    #[doc = "0: Temperature sensor and V_REFINT channel disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor and V_REFINT channel enabled"]
    Enabled = 1,
}
impl From<TSVREFE_A> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSVREFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSVREFE_A {
        match self.bits {
            false => TSVREFE_A::Disabled,
            true => TSVREFE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE_A::Enabled
    }
}
#[doc = "Field `TSVREFE` writer - Temperature sensor and VREFINT enable"]
pub type TSVREFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TSVREFE_A, O>;
impl<'a, const O: u8> TSVREFE_W<'a, O> {
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::Disabled)
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jexttrig(&self) -> JEXTTRIG_R {
        JEXTTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    #[must_use]
    pub fn adon(&mut self) -> ADON_W<0> {
        ADON_W::new(self)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<1> {
        CONT_W::new(self)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<2> {
        CAL_W::new(self)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rstcal(&mut self) -> RSTCAL_W<3> {
        RSTCAL_W::new(self)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<8> {
        DMA_W::new(self)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<11> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<12> {
        JEXTSEL_W::new(self)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jexttrig(&mut self) -> JEXTTRIG_W<15> {
        JEXTTRIG_W::new(self)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<17> {
        EXTSEL_W::new(self)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn exttrig(&mut self) -> EXTTRIG_W<20> {
        EXTTRIG_W::new(self)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<21> {
        JSWSTART_W::new(self)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn swstart(&mut self) -> SWSTART_W<22> {
        SWSTART_W::new(self)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<23> {
        TSVREFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
