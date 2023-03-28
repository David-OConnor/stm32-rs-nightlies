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
#[doc = "Field `COMP1EN` reader - Comparator 1 enable"]
pub type COMP1EN_R = crate::BitReader<COMP1EN_A>;
#[doc = "Comparator 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1EN_A {
    #[doc = "0: Comparator 1 disabled"]
    Disabled = 0,
    #[doc = "1: Comparator 1 enabled"]
    Enabled = 1,
}
impl From<COMP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1EN_A {
        match self.bits {
            false => COMP1EN_A::Disabled,
            true => COMP1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP1EN_A::Enabled
    }
}
#[doc = "Field `COMP1EN` writer - Comparator 1 enable"]
pub type COMP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, COMP1EN_A, O>;
impl<'a, const O: u8> COMP1EN_W<'a, O> {
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP1EN_A::Disabled)
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP1EN_A::Enabled)
    }
}
#[doc = "Field `COMP1SW1` reader - Comparator 1 non inverting input DAC switch"]
pub type COMP1SW1_R = crate::BitReader<COMP1SW1_A>;
#[doc = "Comparator 1 non inverting input DAC switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1SW1_A {
    #[doc = "0: Switch open"]
    Open = 0,
    #[doc = "1: Switch closed"]
    Closed = 1,
}
impl From<COMP1SW1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1SW1_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1SW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1SW1_A {
        match self.bits {
            false => COMP1SW1_A::Open,
            true => COMP1SW1_A::Closed,
        }
    }
    #[doc = "Checks if the value of the field is `Open`"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == COMP1SW1_A::Open
    }
    #[doc = "Checks if the value of the field is `Closed`"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == COMP1SW1_A::Closed
    }
}
#[doc = "Field `COMP1SW1` writer - Comparator 1 non inverting input DAC switch"]
pub type COMP1SW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, COMP1SW1_A, O>;
impl<'a, const O: u8> COMP1SW1_W<'a, O> {
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut W {
        self.variant(COMP1SW1_A::Open)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut W {
        self.variant(COMP1SW1_A::Closed)
    }
}
#[doc = "Field `COMP1MODE` reader - Comparator 1 mode"]
pub type COMP1MODE_R = crate::FieldReader<u8, COMP1MODE_A>;
#[doc = "Comparator 1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1MODE_A {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "2: Low speed / low power"]
    LowSpeed = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VeryLowSpeed = 3,
}
impl From<COMP1MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1MODE_A) -> Self {
        variant as _
    }
}
impl COMP1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1MODE_A {
        match self.bits {
            0 => COMP1MODE_A::HighSpeed,
            1 => COMP1MODE_A::MediumSpeed,
            2 => COMP1MODE_A::LowSpeed,
            3 => COMP1MODE_A::VeryLowSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HighSpeed`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == COMP1MODE_A::HighSpeed
    }
    #[doc = "Checks if the value of the field is `MediumSpeed`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == COMP1MODE_A::MediumSpeed
    }
    #[doc = "Checks if the value of the field is `LowSpeed`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == COMP1MODE_A::LowSpeed
    }
    #[doc = "Checks if the value of the field is `VeryLowSpeed`"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == COMP1MODE_A::VeryLowSpeed
    }
}
#[doc = "Field `COMP1MODE` writer - Comparator 1 mode"]
pub type COMP1MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, COMP1MODE_A, 2, O>;
impl<'a, const O: u8> COMP1MODE_W<'a, O> {
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(COMP1MODE_A::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(COMP1MODE_A::MediumSpeed)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(COMP1MODE_A::LowSpeed)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut W {
        self.variant(COMP1MODE_A::VeryLowSpeed)
    }
}
#[doc = "Field `COMP1INSEL` reader - Comparator 1 inverting input selection"]
pub type COMP1INSEL_R = crate::FieldReader<u8, COMP1INSEL_A>;
#[doc = "Comparator 1 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1INSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    Comp1Inm4 = 4,
    #[doc = "5: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    Comp1Inm5 = 5,
    #[doc = "6: COMP1_INM6 (PA0)"]
    Comp1Inm6 = 6,
}
impl From<COMP1INSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1INSEL_A) -> Self {
        variant as _
    }
}
impl COMP1INSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP1INSEL_A> {
        match self.bits {
            0 => Some(COMP1INSEL_A::OneQuarterVref),
            1 => Some(COMP1INSEL_A::OneHalfVref),
            2 => Some(COMP1INSEL_A::ThreeQuarterVref),
            3 => Some(COMP1INSEL_A::Vref),
            4 => Some(COMP1INSEL_A::Comp1Inm4),
            5 => Some(COMP1INSEL_A::Comp1Inm5),
            6 => Some(COMP1INSEL_A::Comp1Inm6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OneQuarterVref`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP1INSEL_A::OneQuarterVref
    }
    #[doc = "Checks if the value of the field is `OneHalfVref`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP1INSEL_A::OneHalfVref
    }
    #[doc = "Checks if the value of the field is `ThreeQuarterVref`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP1INSEL_A::ThreeQuarterVref
    }
    #[doc = "Checks if the value of the field is `Vref`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP1INSEL_A::Vref
    }
    #[doc = "Checks if the value of the field is `Comp1Inm4`"]
    #[inline(always)]
    pub fn is_comp1_inm4(&self) -> bool {
        *self == COMP1INSEL_A::Comp1Inm4
    }
    #[doc = "Checks if the value of the field is `Comp1Inm5`"]
    #[inline(always)]
    pub fn is_comp1_inm5(&self) -> bool {
        *self == COMP1INSEL_A::Comp1Inm5
    }
    #[doc = "Checks if the value of the field is `Comp1Inm6`"]
    #[inline(always)]
    pub fn is_comp1_inm6(&self) -> bool {
        *self == COMP1INSEL_A::Comp1Inm6
    }
}
#[doc = "Field `COMP1INSEL` writer - Comparator 1 inverting input selection"]
pub type COMP1INSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSR_SPEC, u8, COMP1INSEL_A, 3, O>;
impl<'a, const O: u8> COMP1INSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::Vref)
    }
    #[doc = "COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    #[inline(always)]
    pub fn comp1_inm4(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::Comp1Inm4)
    }
    #[doc = "COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    #[inline(always)]
    pub fn comp1_inm5(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::Comp1Inm5)
    }
    #[doc = "COMP1_INM6 (PA0)"]
    #[inline(always)]
    pub fn comp1_inm6(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::Comp1Inm6)
    }
}
#[doc = "Field `COMP1OUTSEL` reader - Comparator 1 output selection"]
pub type COMP1OUTSEL_R = crate::FieldReader<u8, COMP1OUTSEL_A>;
#[doc = "Comparator 1 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1OUTSEL_A {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 1 break input"]
    Timer1breakInput = 1,
    #[doc = "2: Timer 1 Input capture 1"]
    Timer1inputCapture1 = 2,
    #[doc = "3: Timer 1 OCrefclear input"]
    Timer1ocrefClearInput = 3,
    #[doc = "4: Timer 2 input capture 4"]
    Timer2inputCapture4 = 4,
    #[doc = "5: Timer 2 OCrefclear input"]
    Timer2ocrefClearInput = 5,
    #[doc = "6: Timer 3 input capture 1"]
    Timer3inputCapture1 = 6,
    #[doc = "7: Timer 3 OCrefclear input"]
    Timer3ocrefClearInput = 7,
}
impl From<COMP1OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1OUTSEL_A) -> Self {
        variant as _
    }
}
impl COMP1OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1OUTSEL_A {
        match self.bits {
            0 => COMP1OUTSEL_A::NoSelection,
            1 => COMP1OUTSEL_A::Timer1breakInput,
            2 => COMP1OUTSEL_A::Timer1inputCapture1,
            3 => COMP1OUTSEL_A::Timer1ocrefClearInput,
            4 => COMP1OUTSEL_A::Timer2inputCapture4,
            5 => COMP1OUTSEL_A::Timer2ocrefClearInput,
            6 => COMP1OUTSEL_A::Timer3inputCapture1,
            7 => COMP1OUTSEL_A::Timer3ocrefClearInput,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoSelection`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP1OUTSEL_A::NoSelection
    }
    #[doc = "Checks if the value of the field is `Timer1breakInput`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP1OUTSEL_A::Timer1breakInput
    }
    #[doc = "Checks if the value of the field is `Timer1inputCapture1`"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP1OUTSEL_A::Timer1inputCapture1
    }
    #[doc = "Checks if the value of the field is `Timer1ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL_A::Timer1ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer2inputCapture4`"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP1OUTSEL_A::Timer2inputCapture4
    }
    #[doc = "Checks if the value of the field is `Timer2ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL_A::Timer2ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer3inputCapture1`"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP1OUTSEL_A::Timer3inputCapture1
    }
    #[doc = "Checks if the value of the field is `Timer3ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL_A::Timer3ocrefClearInput
    }
}
#[doc = "Field `COMP1OUTSEL` writer - Comparator 1 output selection"]
pub type COMP1OUTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, COMP1OUTSEL_A, 3, O>;
impl<'a, const O: u8> COMP1OUTSEL_W<'a, O> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::NoSelection)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::Timer1breakInput)
    }
    #[doc = "Timer 1 Input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::Timer1inputCapture1)
    }
    #[doc = "Timer 1 OCrefclear input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::Timer1ocrefClearInput)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::Timer2inputCapture4)
    }
    #[doc = "Timer 2 OCrefclear input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::Timer2ocrefClearInput)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::Timer3inputCapture1)
    }
    #[doc = "Timer 3 OCrefclear input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::Timer3ocrefClearInput)
    }
}
#[doc = "Field `COMP1POL` reader - Comparator 1 output polarity"]
pub type COMP1POL_R = crate::BitReader<COMP1POL_A>;
#[doc = "Comparator 1 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1POL_A {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<COMP1POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1POL_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1POL_A {
        match self.bits {
            false => COMP1POL_A::NotInverted,
            true => COMP1POL_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP1POL_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP1POL_A::Inverted
    }
}
#[doc = "Field `COMP1POL` writer - Comparator 1 output polarity"]
pub type COMP1POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, COMP1POL_A, O>;
impl<'a, const O: u8> COMP1POL_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP1POL_A::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP1POL_A::Inverted)
    }
}
#[doc = "Field `COMP1HYST` reader - Comparator 1 hysteresis"]
pub type COMP1HYST_R = crate::FieldReader<u8, COMP1HYST_A>;
#[doc = "Comparator 1 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1HYST_A {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
}
impl From<COMP1HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1HYST_A) -> Self {
        variant as _
    }
}
impl COMP1HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1HYST_A {
        match self.bits {
            0 => COMP1HYST_A::NoHysteresis,
            1 => COMP1HYST_A::LowHysteresis,
            2 => COMP1HYST_A::MediumHysteresis,
            3 => COMP1HYST_A::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoHysteresis`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == COMP1HYST_A::NoHysteresis
    }
    #[doc = "Checks if the value of the field is `LowHysteresis`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == COMP1HYST_A::LowHysteresis
    }
    #[doc = "Checks if the value of the field is `MediumHysteresis`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == COMP1HYST_A::MediumHysteresis
    }
    #[doc = "Checks if the value of the field is `HighHysteresis`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == COMP1HYST_A::HighHysteresis
    }
}
#[doc = "Field `COMP1HYST` writer - Comparator 1 hysteresis"]
pub type COMP1HYST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, COMP1HYST_A, 2, O>;
impl<'a, const O: u8> COMP1HYST_W<'a, O> {
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(COMP1HYST_A::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(COMP1HYST_A::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(COMP1HYST_A::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(COMP1HYST_A::HighHysteresis)
    }
}
#[doc = "Field `COMP1OUT` reader - Comparator 1 output"]
pub type COMP1OUT_R = crate::BitReader<COMP1OUT_A>;
#[doc = "Comparator 1 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<COMP1OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1OUT_A {
        match self.bits {
            false => COMP1OUT_A::Low,
            true => COMP1OUT_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP1OUT_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP1OUT_A::High
    }
}
#[doc = "Field `COMP1LOCK` reader - Comparator 1 lock"]
pub type COMP1LOCK_R = crate::BitReader<COMP1LOCK_A>;
#[doc = "Comparator 1 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1LOCK_A {
    #[doc = "0: Comparator 1 CSR bits (CSR\\[15:0\\]) are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator 1 CSR bits (CSR\\[15:0\\]) are read-only"]
    Locked = 1,
}
impl From<COMP1LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1LOCK_A {
        match self.bits {
            false => COMP1LOCK_A::Unlocked,
            true => COMP1LOCK_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP1LOCK_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP1LOCK_A::Locked
    }
}
#[doc = "Field `COMP1LOCK` writer - Comparator 1 lock"]
pub type COMP1LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, COMP1LOCK_A, O>;
impl<'a, const O: u8> COMP1LOCK_W<'a, O> {
    #[doc = "Comparator 1 CSR bits (CSR\\[15:0\\]) are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP1LOCK_A::Unlocked)
    }
    #[doc = "Comparator 1 CSR bits (CSR\\[15:0\\]) are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP1LOCK_A::Locked)
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub type COMP2EN_R = crate::BitReader<COMP2EN_A>;
#[doc = "Comparator 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2EN_A {
    #[doc = "0: Comparator 2 disabled"]
    Disabled = 0,
    #[doc = "1: Comparator 2 enabled"]
    Enabled = 1,
}
impl From<COMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2EN_A {
        match self.bits {
            false => COMP2EN_A::Disabled,
            true => COMP2EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP2EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP2EN_A::Enabled
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub type COMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, COMP2EN_A, O>;
impl<'a, const O: u8> COMP2EN_W<'a, O> {
    #[doc = "Comparator 2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::Disabled)
    }
    #[doc = "Comparator 2 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::Enabled)
    }
}
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub type COMP2MODE_R = crate::FieldReader<u8, COMP2MODE_A>;
#[doc = "Comparator 2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2MODE_A {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "2: Low speed / low power"]
    LowSpeed = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VeryLowSpeed = 3,
}
impl From<COMP2MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2MODE_A) -> Self {
        variant as _
    }
}
impl COMP2MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2MODE_A {
        match self.bits {
            0 => COMP2MODE_A::HighSpeed,
            1 => COMP2MODE_A::MediumSpeed,
            2 => COMP2MODE_A::LowSpeed,
            3 => COMP2MODE_A::VeryLowSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HighSpeed`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == COMP2MODE_A::HighSpeed
    }
    #[doc = "Checks if the value of the field is `MediumSpeed`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == COMP2MODE_A::MediumSpeed
    }
    #[doc = "Checks if the value of the field is `LowSpeed`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == COMP2MODE_A::LowSpeed
    }
    #[doc = "Checks if the value of the field is `VeryLowSpeed`"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == COMP2MODE_A::VeryLowSpeed
    }
}
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub type COMP2MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, COMP2MODE_A, 2, O>;
impl<'a, const O: u8> COMP2MODE_W<'a, O> {
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(COMP2MODE_A::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(COMP2MODE_A::MediumSpeed)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(COMP2MODE_A::LowSpeed)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut W {
        self.variant(COMP2MODE_A::VeryLowSpeed)
    }
}
#[doc = "Field `COMP2INSEL` reader - Comparator 2 inverting input selection"]
pub type COMP2INSEL_R = crate::FieldReader<u8, COMP2INSEL_A>;
#[doc = "Comparator 2 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    Comp2Inm4 = 4,
    #[doc = "5: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    Comp2Inm5 = 5,
    #[doc = "6: COMP1_INM6 (PA2)"]
    Comp2Inm6 = 6,
}
impl From<COMP2INSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INSEL_A) -> Self {
        variant as _
    }
}
impl COMP2INSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2INSEL_A> {
        match self.bits {
            0 => Some(COMP2INSEL_A::OneQuarterVref),
            1 => Some(COMP2INSEL_A::OneHalfVref),
            2 => Some(COMP2INSEL_A::ThreeQuarterVref),
            3 => Some(COMP2INSEL_A::Vref),
            4 => Some(COMP2INSEL_A::Comp2Inm4),
            5 => Some(COMP2INSEL_A::Comp2Inm5),
            6 => Some(COMP2INSEL_A::Comp2Inm6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OneQuarterVref`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP2INSEL_A::OneQuarterVref
    }
    #[doc = "Checks if the value of the field is `OneHalfVref`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP2INSEL_A::OneHalfVref
    }
    #[doc = "Checks if the value of the field is `ThreeQuarterVref`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP2INSEL_A::ThreeQuarterVref
    }
    #[doc = "Checks if the value of the field is `Vref`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP2INSEL_A::Vref
    }
    #[doc = "Checks if the value of the field is `Comp2Inm4`"]
    #[inline(always)]
    pub fn is_comp2_inm4(&self) -> bool {
        *self == COMP2INSEL_A::Comp2Inm4
    }
    #[doc = "Checks if the value of the field is `Comp2Inm5`"]
    #[inline(always)]
    pub fn is_comp2_inm5(&self) -> bool {
        *self == COMP2INSEL_A::Comp2Inm5
    }
    #[doc = "Checks if the value of the field is `Comp2Inm6`"]
    #[inline(always)]
    pub fn is_comp2_inm6(&self) -> bool {
        *self == COMP2INSEL_A::Comp2Inm6
    }
}
#[doc = "Field `COMP2INSEL` writer - Comparator 2 inverting input selection"]
pub type COMP2INSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSR_SPEC, u8, COMP2INSEL_A, 3, O>;
impl<'a, const O: u8> COMP2INSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::Vref)
    }
    #[doc = "COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    #[inline(always)]
    pub fn comp2_inm4(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::Comp2Inm4)
    }
    #[doc = "COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    #[inline(always)]
    pub fn comp2_inm5(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::Comp2Inm5)
    }
    #[doc = "COMP1_INM6 (PA2)"]
    #[inline(always)]
    pub fn comp2_inm6(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::Comp2Inm6)
    }
}
#[doc = "Field `WNDWEN` reader - Window mode enable"]
pub type WNDWEN_R = crate::BitReader<WNDWEN_A>;
#[doc = "Window mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WNDWEN_A {
    #[doc = "0: Window mode disabled"]
    Disabled = 0,
    #[doc = "1: Window mode enabled"]
    Enabled = 1,
}
impl From<WNDWEN_A> for bool {
    #[inline(always)]
    fn from(variant: WNDWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WNDWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WNDWEN_A {
        match self.bits {
            false => WNDWEN_A::Disabled,
            true => WNDWEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WNDWEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WNDWEN_A::Enabled
    }
}
#[doc = "Field `WNDWEN` writer - Window mode enable"]
pub type WNDWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, WNDWEN_A, O>;
impl<'a, const O: u8> WNDWEN_W<'a, O> {
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WNDWEN_A::Disabled)
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WNDWEN_A::Enabled)
    }
}
#[doc = "Field `COMP2OUTSEL` reader - Comparator 2 output selection"]
pub type COMP2OUTSEL_R = crate::FieldReader<u8, COMP2OUTSEL_A>;
#[doc = "Comparator 2 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2OUTSEL_A {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 1 break input"]
    Timer1breakInput = 1,
    #[doc = "2: Timer 1 Input capture 1"]
    Timer1inputCapture1 = 2,
    #[doc = "3: Timer 1 OCrefclear input"]
    Timer1ocrefClearInput = 3,
    #[doc = "4: Timer 2 input capture 4"]
    Timer2inputCapture4 = 4,
    #[doc = "5: Timer 2 OCrefclear input"]
    Timer2ocrefClearInput = 5,
    #[doc = "6: Timer 3 input capture 1"]
    Timer3inputCapture1 = 6,
    #[doc = "7: Timer 3 OCrefclear input"]
    Timer3ocrefClearInput = 7,
}
impl From<COMP2OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2OUTSEL_A) -> Self {
        variant as _
    }
}
impl COMP2OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2OUTSEL_A {
        match self.bits {
            0 => COMP2OUTSEL_A::NoSelection,
            1 => COMP2OUTSEL_A::Timer1breakInput,
            2 => COMP2OUTSEL_A::Timer1inputCapture1,
            3 => COMP2OUTSEL_A::Timer1ocrefClearInput,
            4 => COMP2OUTSEL_A::Timer2inputCapture4,
            5 => COMP2OUTSEL_A::Timer2ocrefClearInput,
            6 => COMP2OUTSEL_A::Timer3inputCapture1,
            7 => COMP2OUTSEL_A::Timer3ocrefClearInput,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoSelection`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP2OUTSEL_A::NoSelection
    }
    #[doc = "Checks if the value of the field is `Timer1breakInput`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer1breakInput
    }
    #[doc = "Checks if the value of the field is `Timer1inputCapture1`"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer1inputCapture1
    }
    #[doc = "Checks if the value of the field is `Timer1ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer1ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer2inputCapture4`"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer2inputCapture4
    }
    #[doc = "Checks if the value of the field is `Timer2ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer2ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer3inputCapture1`"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer3inputCapture1
    }
    #[doc = "Checks if the value of the field is `Timer3ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer3ocrefClearInput
    }
}
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub type COMP2OUTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, COMP2OUTSEL_A, 3, O>;
impl<'a, const O: u8> COMP2OUTSEL_W<'a, O> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::NoSelection)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer1breakInput)
    }
    #[doc = "Timer 1 Input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer1inputCapture1)
    }
    #[doc = "Timer 1 OCrefclear input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer1ocrefClearInput)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer2inputCapture4)
    }
    #[doc = "Timer 2 OCrefclear input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer2ocrefClearInput)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer3inputCapture1)
    }
    #[doc = "Timer 3 OCrefclear input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer3ocrefClearInput)
    }
}
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub type COMP2POL_R = crate::BitReader<COMP2POL_A>;
#[doc = "Comparator 2 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2POL_A {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<COMP2POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2POL_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2POL_A {
        match self.bits {
            false => COMP2POL_A::NotInverted,
            true => COMP2POL_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP2POL_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP2POL_A::Inverted
    }
}
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub type COMP2POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, COMP2POL_A, O>;
impl<'a, const O: u8> COMP2POL_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP2POL_A::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP2POL_A::Inverted)
    }
}
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub type COMP2HYST_R = crate::FieldReader<u8, COMP2HYST_A>;
#[doc = "Comparator 2 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2HYST_A {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
}
impl From<COMP2HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2HYST_A) -> Self {
        variant as _
    }
}
impl COMP2HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2HYST_A {
        match self.bits {
            0 => COMP2HYST_A::NoHysteresis,
            1 => COMP2HYST_A::LowHysteresis,
            2 => COMP2HYST_A::MediumHysteresis,
            3 => COMP2HYST_A::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoHysteresis`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == COMP2HYST_A::NoHysteresis
    }
    #[doc = "Checks if the value of the field is `LowHysteresis`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == COMP2HYST_A::LowHysteresis
    }
    #[doc = "Checks if the value of the field is `MediumHysteresis`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == COMP2HYST_A::MediumHysteresis
    }
    #[doc = "Checks if the value of the field is `HighHysteresis`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == COMP2HYST_A::HighHysteresis
    }
}
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub type COMP2HYST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, COMP2HYST_A, 2, O>;
impl<'a, const O: u8> COMP2HYST_W<'a, O> {
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(COMP2HYST_A::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(COMP2HYST_A::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(COMP2HYST_A::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(COMP2HYST_A::HighHysteresis)
    }
}
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub type COMP2OUT_R = crate::BitReader<COMP2OUT_A>;
#[doc = "Comparator 2 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<COMP2OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2OUT_A {
        match self.bits {
            false => COMP2OUT_A::Low,
            true => COMP2OUT_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP2OUT_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP2OUT_A::High
    }
}
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub type COMP2LOCK_R = crate::BitReader<COMP2LOCK_A>;
#[doc = "Comparator 2 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LOCK_A {
    #[doc = "0: Comparator 2 CSR bits (CSR\\[31:16\\]) are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator 2 CSR bits (CSR\\[31:16\\]) are read-only"]
    Locked = 1,
}
impl From<COMP2LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LOCK_A {
        match self.bits {
            false => COMP2LOCK_A::Unlocked,
            true => COMP2LOCK_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP2LOCK_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP2LOCK_A::Locked
    }
}
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub type COMP2LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, COMP2LOCK_A, O>;
impl<'a, const O: u8> COMP2LOCK_W<'a, O> {
    #[doc = "Comparator 2 CSR bits (CSR\\[31:16\\]) are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::Unlocked)
    }
    #[doc = "Comparator 2 CSR bits (CSR\\[31:16\\]) are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input DAC switch"]
    #[inline(always)]
    pub fn comp1sw1(&self) -> COMP1SW1_R {
        COMP1SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&self) -> COMP1MODE_R {
        COMP1MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&self) -> COMP1INSEL_R {
        COMP1INSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1outsel(&self) -> COMP1OUTSEL_R {
        COMP1OUTSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&self) -> COMP1POL_R {
        COMP1POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&self) -> COMP1HYST_R {
        COMP1HYST_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator 1 output"]
    #[inline(always)]
    pub fn comp1out(&self) -> COMP1OUT_R {
        COMP1OUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wndwen(&self) -> WNDWEN_R {
        WNDWEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1en(&mut self) -> COMP1EN_W<0> {
        COMP1EN_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input DAC switch"]
    #[inline(always)]
    #[must_use]
    pub fn comp1sw1(&mut self) -> COMP1SW1_W<1> {
        COMP1SW1_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp1mode(&mut self) -> COMP1MODE_W<2> {
        COMP1MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1insel(&mut self) -> COMP1INSEL_W<4> {
        COMP1INSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1outsel(&mut self) -> COMP1OUTSEL_W<8> {
        COMP1OUTSEL_W::new(self)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp1pol(&mut self) -> COMP1POL_W<11> {
        COMP1POL_W::new(self)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp1hyst(&mut self) -> COMP1HYST_W<12> {
        COMP1HYST_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W<15> {
        COMP1LOCK_W::new(self)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<16> {
        COMP2EN_W::new(self)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp2mode(&mut self) -> COMP2MODE_W<18> {
        COMP2MODE_W::new(self)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W<20> {
        COMP2INSEL_W::new(self)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wndwen(&mut self) -> WNDWEN_W<23> {
        WNDWEN_W::new(self)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<24> {
        COMP2OUTSEL_W::new(self)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp2pol(&mut self) -> COMP2POL_W<27> {
        COMP2POL_W::new(self)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W<28> {
        COMP2HYST_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<31> {
        COMP2LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
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
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
