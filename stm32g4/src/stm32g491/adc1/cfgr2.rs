#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROVSE` reader - Regular Oversampling Enable"]
pub type ROVSE_R = crate::BitReader<ROVSE_A>;
#[doc = "Regular Oversampling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE_A {
    #[doc = "0: Regular oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Regular oversampling enabled"]
    Enabled = 1,
}
impl From<ROVSE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVSE_A {
        match self.bits {
            false => ROVSE_A::Disabled,
            true => ROVSE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE_A::Enabled
    }
}
#[doc = "Field `ROVSE` writer - Regular Oversampling Enable"]
pub type ROVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ROVSE_A, O>;
impl<'a, const O: u8> ROVSE_W<'a, O> {
    #[doc = "Regular oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROVSE_A::Disabled)
    }
    #[doc = "Regular oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROVSE_A::Enabled)
    }
}
#[doc = "Field `JOVSE` reader - Injected Oversampling Enable"]
pub type JOVSE_R = crate::BitReader<JOVSE_A>;
#[doc = "Injected Oversampling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE_A {
    #[doc = "0: Injected oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Injected oversampling enabled"]
    Enabled = 1,
}
impl From<JOVSE_A> for bool {
    #[inline(always)]
    fn from(variant: JOVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl JOVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVSE_A {
        match self.bits {
            false => JOVSE_A::Disabled,
            true => JOVSE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE_A::Enabled
    }
}
#[doc = "Field `JOVSE` writer - Injected Oversampling Enable"]
pub type JOVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, JOVSE_A, O>;
impl<'a, const O: u8> JOVSE_W<'a, O> {
    #[doc = "Injected oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JOVSE_A::Disabled)
    }
    #[doc = "Injected oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JOVSE_A::Enabled)
    }
}
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OVSR_R = crate::FieldReader<u8, OVSR_A>;
#[doc = "Oversampling ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: Oversampling ratio of 2"]
    Os2 = 0,
    #[doc = "1: Oversampling ratio of 4"]
    Os4 = 1,
    #[doc = "2: Oversampling ratio of 8"]
    Os8 = 2,
    #[doc = "3: Oversampling ratio of 16"]
    Os16 = 3,
    #[doc = "4: Oversampling ratio of 32"]
    Os32 = 4,
    #[doc = "5: Oversampling ratio of 64"]
    Os64 = 5,
    #[doc = "6: Oversampling ratio of 128"]
    Os128 = 6,
    #[doc = "7: Oversampling ratio of 256"]
    Os256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::Os2,
            1 => OVSR_A::Os4,
            2 => OVSR_A::Os8,
            3 => OVSR_A::Os16,
            4 => OVSR_A::Os32,
            5 => OVSR_A::Os64,
            6 => OVSR_A::Os128,
            7 => OVSR_A::Os256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Os2`"]
    #[inline(always)]
    pub fn is_os2(&self) -> bool {
        *self == OVSR_A::Os2
    }
    #[doc = "Checks if the value of the field is `Os4`"]
    #[inline(always)]
    pub fn is_os4(&self) -> bool {
        *self == OVSR_A::Os4
    }
    #[doc = "Checks if the value of the field is `Os8`"]
    #[inline(always)]
    pub fn is_os8(&self) -> bool {
        *self == OVSR_A::Os8
    }
    #[doc = "Checks if the value of the field is `Os16`"]
    #[inline(always)]
    pub fn is_os16(&self) -> bool {
        *self == OVSR_A::Os16
    }
    #[doc = "Checks if the value of the field is `Os32`"]
    #[inline(always)]
    pub fn is_os32(&self) -> bool {
        *self == OVSR_A::Os32
    }
    #[doc = "Checks if the value of the field is `Os64`"]
    #[inline(always)]
    pub fn is_os64(&self) -> bool {
        *self == OVSR_A::Os64
    }
    #[doc = "Checks if the value of the field is `Os128`"]
    #[inline(always)]
    pub fn is_os128(&self) -> bool {
        *self == OVSR_A::Os128
    }
    #[doc = "Checks if the value of the field is `Os256`"]
    #[inline(always)]
    pub fn is_os256(&self) -> bool {
        *self == OVSR_A::Os256
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio"]
pub type OVSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u8, OVSR_A, 3, O>;
impl<'a, const O: u8> OVSR_W<'a, O> {
    #[doc = "Oversampling ratio of 2"]
    #[inline(always)]
    pub fn os2(self) -> &'a mut W {
        self.variant(OVSR_A::Os2)
    }
    #[doc = "Oversampling ratio of 4"]
    #[inline(always)]
    pub fn os4(self) -> &'a mut W {
        self.variant(OVSR_A::Os4)
    }
    #[doc = "Oversampling ratio of 8"]
    #[inline(always)]
    pub fn os8(self) -> &'a mut W {
        self.variant(OVSR_A::Os8)
    }
    #[doc = "Oversampling ratio of 16"]
    #[inline(always)]
    pub fn os16(self) -> &'a mut W {
        self.variant(OVSR_A::Os16)
    }
    #[doc = "Oversampling ratio of 32"]
    #[inline(always)]
    pub fn os32(self) -> &'a mut W {
        self.variant(OVSR_A::Os32)
    }
    #[doc = "Oversampling ratio of 64"]
    #[inline(always)]
    pub fn os64(self) -> &'a mut W {
        self.variant(OVSR_A::Os64)
    }
    #[doc = "Oversampling ratio of 128"]
    #[inline(always)]
    pub fn os128(self) -> &'a mut W {
        self.variant(OVSR_A::Os128)
    }
    #[doc = "Oversampling ratio of 256"]
    #[inline(always)]
    pub fn os256(self) -> &'a mut W {
        self.variant(OVSR_A::Os256)
    }
}
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OVSS_R = crate::FieldReader<u8, OVSS_A>;
#[doc = "Oversampling shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS_A {
    #[doc = "0: No right shift applied to oversampling result"]
    NoShift = 0,
    #[doc = "1: Shift oversampling result right by 1 bit"]
    Shift1 = 1,
    #[doc = "2: Shift oversampling result right by 2 bits"]
    Shift2 = 2,
    #[doc = "3: Shift oversampling result right by 3 bits"]
    Shift3 = 3,
    #[doc = "4: Shift oversampling result right by 4 bits"]
    Shift4 = 4,
    #[doc = "5: Shift oversampling result right by 5 bits"]
    Shift5 = 5,
    #[doc = "6: Shift oversampling result right by 6 bits"]
    Shift6 = 6,
    #[doc = "7: Shift oversampling result right by 7 bits"]
    Shift7 = 7,
    #[doc = "8: Shift oversampling result right by 8 bits"]
    Shift8 = 8,
}
impl From<OVSS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSS_A) -> Self {
        variant as _
    }
}
impl OVSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVSS_A> {
        match self.bits {
            0 => Some(OVSS_A::NoShift),
            1 => Some(OVSS_A::Shift1),
            2 => Some(OVSS_A::Shift2),
            3 => Some(OVSS_A::Shift3),
            4 => Some(OVSS_A::Shift4),
            5 => Some(OVSS_A::Shift5),
            6 => Some(OVSS_A::Shift6),
            7 => Some(OVSS_A::Shift7),
            8 => Some(OVSS_A::Shift8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoShift`"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS_A::NoShift
    }
    #[doc = "Checks if the value of the field is `Shift1`"]
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == OVSS_A::Shift1
    }
    #[doc = "Checks if the value of the field is `Shift2`"]
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == OVSS_A::Shift2
    }
    #[doc = "Checks if the value of the field is `Shift3`"]
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == OVSS_A::Shift3
    }
    #[doc = "Checks if the value of the field is `Shift4`"]
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == OVSS_A::Shift4
    }
    #[doc = "Checks if the value of the field is `Shift5`"]
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == OVSS_A::Shift5
    }
    #[doc = "Checks if the value of the field is `Shift6`"]
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == OVSS_A::Shift6
    }
    #[doc = "Checks if the value of the field is `Shift7`"]
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == OVSS_A::Shift7
    }
    #[doc = "Checks if the value of the field is `Shift8`"]
    #[inline(always)]
    pub fn is_shift8(&self) -> bool {
        *self == OVSS_A::Shift8
    }
}
#[doc = "Field `OVSS` writer - Oversampling shift"]
pub type OVSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, OVSS_A, 4, O>;
impl<'a, const O: u8> OVSS_W<'a, O> {
    #[doc = "No right shift applied to oversampling result"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut W {
        self.variant(OVSS_A::NoShift)
    }
    #[doc = "Shift oversampling result right by 1 bit"]
    #[inline(always)]
    pub fn shift1(self) -> &'a mut W {
        self.variant(OVSS_A::Shift1)
    }
    #[doc = "Shift oversampling result right by 2 bits"]
    #[inline(always)]
    pub fn shift2(self) -> &'a mut W {
        self.variant(OVSS_A::Shift2)
    }
    #[doc = "Shift oversampling result right by 3 bits"]
    #[inline(always)]
    pub fn shift3(self) -> &'a mut W {
        self.variant(OVSS_A::Shift3)
    }
    #[doc = "Shift oversampling result right by 4 bits"]
    #[inline(always)]
    pub fn shift4(self) -> &'a mut W {
        self.variant(OVSS_A::Shift4)
    }
    #[doc = "Shift oversampling result right by 5 bits"]
    #[inline(always)]
    pub fn shift5(self) -> &'a mut W {
        self.variant(OVSS_A::Shift5)
    }
    #[doc = "Shift oversampling result right by 6 bits"]
    #[inline(always)]
    pub fn shift6(self) -> &'a mut W {
        self.variant(OVSS_A::Shift6)
    }
    #[doc = "Shift oversampling result right by 7 bits"]
    #[inline(always)]
    pub fn shift7(self) -> &'a mut W {
        self.variant(OVSS_A::Shift7)
    }
    #[doc = "Shift oversampling result right by 8 bits"]
    #[inline(always)]
    pub fn shift8(self) -> &'a mut W {
        self.variant(OVSS_A::Shift8)
    }
}
#[doc = "Field `TROVS` reader - Triggered Regular Oversampling"]
pub type TROVS_R = crate::BitReader<TROVS_A>;
#[doc = "Triggered Regular Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS_A {
    #[doc = "0: All oversampled conversions for a channel are run following a trigger"]
    Automatic = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    Triggered = 1,
}
impl From<TROVS_A> for bool {
    #[inline(always)]
    fn from(variant: TROVS_A) -> Self {
        variant as u8 != 0
    }
}
impl TROVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TROVS_A {
        match self.bits {
            false => TROVS_A::Automatic,
            true => TROVS_A::Triggered,
        }
    }
    #[doc = "Checks if the value of the field is `Automatic`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS_A::Automatic
    }
    #[doc = "Checks if the value of the field is `Triggered`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS_A::Triggered
    }
}
#[doc = "Field `TROVS` writer - Triggered Regular Oversampling"]
pub type TROVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, TROVS_A, O>;
impl<'a, const O: u8> TROVS_W<'a, O> {
    #[doc = "All oversampled conversions for a channel are run following a trigger"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(TROVS_A::Automatic)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut W {
        self.variant(TROVS_A::Triggered)
    }
}
#[doc = "Field `ROVSM` reader - Regular Oversampling mode"]
pub type ROVSM_R = crate::BitReader<ROVSM_A>;
#[doc = "Regular Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM_A {
    #[doc = "0: Oversampling is temporary stopped and continued after injection sequence"]
    Continued = 0,
    #[doc = "1: Oversampling is aborted and resumed from start after injection sequence"]
    Resumed = 1,
}
impl From<ROVSM_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSM_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVSM_A {
        match self.bits {
            false => ROVSM_A::Continued,
            true => ROVSM_A::Resumed,
        }
    }
    #[doc = "Checks if the value of the field is `Continued`"]
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM_A::Continued
    }
    #[doc = "Checks if the value of the field is `Resumed`"]
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM_A::Resumed
    }
}
#[doc = "Field `ROVSM` writer - Regular Oversampling mode"]
pub type ROVSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ROVSM_A, O>;
impl<'a, const O: u8> ROVSM_W<'a, O> {
    #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
    #[inline(always)]
    pub fn continued(self) -> &'a mut W {
        self.variant(ROVSM_A::Continued)
    }
    #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
    #[inline(always)]
    pub fn resumed(self) -> &'a mut W {
        self.variant(ROVSM_A::Resumed)
    }
}
#[doc = "Field `GCOMP` reader - Gain compensation mode"]
pub type GCOMP_R = crate::BitReader<GCOMP_A>;
#[doc = "Gain compensation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCOMP_A {
    #[doc = "0: Regular ADC operating mode"]
    Disabled = 0,
    #[doc = "1: Gain compensation enabled and applies to all channels"]
    Enabled = 1,
}
impl From<GCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: GCOMP_A) -> Self {
        variant as u8 != 0
    }
}
impl GCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCOMP_A {
        match self.bits {
            false => GCOMP_A::Disabled,
            true => GCOMP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCOMP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCOMP_A::Enabled
    }
}
#[doc = "Field `GCOMP` writer - Gain compensation mode"]
pub type GCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, GCOMP_A, O>;
impl<'a, const O: u8> GCOMP_W<'a, O> {
    #[doc = "Regular ADC operating mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCOMP_A::Disabled)
    }
    #[doc = "Gain compensation enabled and applies to all channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCOMP_A::Enabled)
    }
}
#[doc = "Field `SWTRIG` reader - Software trigger bit for sampling time control trigger mode"]
pub type SWTRIG_R = crate::BitReader<SWTRIG_A>;
#[doc = "Software trigger bit for sampling time control trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG_A {
    #[doc = "0: End sampling period and start conversion"]
    Disabled = 0,
    #[doc = "1: Start sampling period"]
    Enabled = 1,
}
impl From<SWTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl SWTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWTRIG_A {
        match self.bits {
            false => SWTRIG_A::Disabled,
            true => SWTRIG_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWTRIG_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWTRIG_A::Enabled
    }
}
#[doc = "Field `SWTRIG` writer - Software trigger bit for sampling time control trigger mode"]
pub type SWTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SWTRIG_A, O>;
impl<'a, const O: u8> SWTRIG_W<'a, O> {
    #[doc = "End sampling period and start conversion"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG_A::Disabled)
    }
    #[doc = "Start sampling period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG_A::Enabled)
    }
}
#[doc = "Field `BULB` reader - Bulb sampling mode"]
pub type BULB_R = crate::BitReader<BULB_A>;
#[doc = "Bulb sampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BULB_A {
    #[doc = "0: Bulb sampling mode disabled"]
    Disabled = 0,
    #[doc = "1: Bulb sampling mode enabled. Immediately start sampling after last conversion finishes."]
    Enabled = 1,
}
impl From<BULB_A> for bool {
    #[inline(always)]
    fn from(variant: BULB_A) -> Self {
        variant as u8 != 0
    }
}
impl BULB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BULB_A {
        match self.bits {
            false => BULB_A::Disabled,
            true => BULB_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BULB_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BULB_A::Enabled
    }
}
#[doc = "Field `BULB` writer - Bulb sampling mode"]
pub type BULB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, BULB_A, O>;
impl<'a, const O: u8> BULB_W<'a, O> {
    #[doc = "Bulb sampling mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BULB_A::Disabled)
    }
    #[doc = "Bulb sampling mode enabled. Immediately start sampling after last conversion finishes."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BULB_A::Enabled)
    }
}
#[doc = "Field `SMPTRIG` reader - Sampling time control trigger mode"]
pub type SMPTRIG_R = crate::BitReader<SMPTRIG_A>;
#[doc = "Sampling time control trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPTRIG_A {
    #[doc = "0: Sampling time control trigger mode disabled"]
    Disabled = 0,
    #[doc = "1: Sampling time control trigger mode enabled"]
    Enabled = 1,
}
impl From<SMPTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SMPTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPTRIG_A {
        match self.bits {
            false => SMPTRIG_A::Disabled,
            true => SMPTRIG_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPTRIG_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPTRIG_A::Enabled
    }
}
#[doc = "Field `SMPTRIG` writer - Sampling time control trigger mode"]
pub type SMPTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SMPTRIG_A, O>;
impl<'a, const O: u8> SMPTRIG_W<'a, O> {
    #[doc = "Sampling time control trigger mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMPTRIG_A::Disabled)
    }
    #[doc = "Sampling time control trigger mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMPTRIG_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Regular Oversampling Enable"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Injected Oversampling Enable"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Gain compensation mode"]
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - Software trigger bit for sampling time control trigger mode"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bulb sampling mode"]
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Sampling time control trigger mode"]
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Regular Oversampling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<0> {
        ROVSE_W::new(self)
    }
    #[doc = "Bit 1 - Injected Oversampling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<1> {
        JOVSE_W::new(self)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<2> {
        OVSR_W::new(self)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<5> {
        OVSS_W::new(self)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn trovs(&mut self) -> TROVS_W<9> {
        TROVS_W::new(self)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<10> {
        ROVSM_W::new(self)
    }
    #[doc = "Bit 16 - Gain compensation mode"]
    #[inline(always)]
    #[must_use]
    pub fn gcomp(&mut self) -> GCOMP_W<16> {
        GCOMP_W::new(self)
    }
    #[doc = "Bit 25 - Software trigger bit for sampling time control trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SWTRIG_W<25> {
        SWTRIG_W::new(self)
    }
    #[doc = "Bit 26 - Bulb sampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn bulb(&mut self) -> BULB_W<26> {
        BULB_W::new(self)
    }
    #[doc = "Bit 27 - Sampling time control trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn smptrig(&mut self) -> SMPTRIG_W<27> {
        SMPTRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
