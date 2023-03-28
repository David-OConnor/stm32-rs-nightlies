#[doc = "Register `COMP1_CSR` reader"]
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_CSR` writer"]
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Comparator 1 enable bit"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Comparator 1 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Comparator 1 disabled"]
    Disabled = 0,
    #[doc = "1: Comparator 1 enabled"]
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
#[doc = "Field `EN` writer - Comparator 1 enable bit"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
#[doc = "Field `PWRMODE` reader - Power Mode of the comparator 1"]
pub type PWRMODE_R = crate::FieldReader<u8, PWRMODE_A>;
#[doc = "Power Mode of the comparator 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRMODE_A {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "3: Low speed / ultra-low power"]
    LowSpeed = 3,
}
impl From<PWRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE_A) -> Self {
        variant as _
    }
}
impl PWRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWRMODE_A> {
        match self.bits {
            0 => Some(PWRMODE_A::HighSpeed),
            1 => Some(PWRMODE_A::MediumSpeed),
            3 => Some(PWRMODE_A::LowSpeed),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HighSpeed`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == PWRMODE_A::HighSpeed
    }
    #[doc = "Checks if the value of the field is `MediumSpeed`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == PWRMODE_A::MediumSpeed
    }
    #[doc = "Checks if the value of the field is `LowSpeed`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == PWRMODE_A::LowSpeed
    }
}
#[doc = "Field `PWRMODE` writer - Power Mode of the comparator 1"]
pub type PWRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, PWRMODE_A, 2, O>;
impl<'a, const O: u8> PWRMODE_W<'a, O> {
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::MediumSpeed)
    }
    #[doc = "Low speed / ultra-low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::LowSpeed)
    }
}
#[doc = "Field `INMSEL` reader - Comparator 1 Input Minus connection configuration bit"]
pub type INMSEL_R = crate::FieldReader<u8, INMSEL_A>;
#[doc = "Comparator 1 Input Minus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INMSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: DAC Channel 1"]
    DacCh1 = 4,
    #[doc = "5: DAC Channel 2"]
    DacCh2 = 5,
    #[doc = "6: GPIO selected with INMESEL"]
    Gpiox = 6,
}
impl From<INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INMSEL_A) -> Self {
        variant as _
    }
}
impl INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INMSEL_A> {
        match self.bits {
            0 => Some(INMSEL_A::OneQuarterVref),
            1 => Some(INMSEL_A::OneHalfVref),
            2 => Some(INMSEL_A::ThreeQuarterVref),
            3 => Some(INMSEL_A::Vref),
            4 => Some(INMSEL_A::DacCh1),
            5 => Some(INMSEL_A::DacCh2),
            6 => Some(INMSEL_A::Gpiox),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OneQuarterVref`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == INMSEL_A::OneQuarterVref
    }
    #[doc = "Checks if the value of the field is `OneHalfVref`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == INMSEL_A::OneHalfVref
    }
    #[doc = "Checks if the value of the field is `ThreeQuarterVref`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == INMSEL_A::ThreeQuarterVref
    }
    #[doc = "Checks if the value of the field is `Vref`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == INMSEL_A::Vref
    }
    #[doc = "Checks if the value of the field is `DacCh1`"]
    #[inline(always)]
    pub fn is_dac_ch1(&self) -> bool {
        *self == INMSEL_A::DacCh1
    }
    #[doc = "Checks if the value of the field is `DacCh2`"]
    #[inline(always)]
    pub fn is_dac_ch2(&self) -> bool {
        *self == INMSEL_A::DacCh2
    }
    #[doc = "Checks if the value of the field is `Gpiox`"]
    #[inline(always)]
    pub fn is_gpiox(&self) -> bool {
        *self == INMSEL_A::Gpiox
    }
}
#[doc = "Field `INMSEL` writer - Comparator 1 Input Minus connection configuration bit"]
pub type INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, INMSEL_A, 3, O>;
impl<'a, const O: u8> INMSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(INMSEL_A::Vref)
    }
    #[doc = "DAC Channel 1"]
    #[inline(always)]
    pub fn dac_ch1(self) -> &'a mut W {
        self.variant(INMSEL_A::DacCh1)
    }
    #[doc = "DAC Channel 2"]
    #[inline(always)]
    pub fn dac_ch2(self) -> &'a mut W {
        self.variant(INMSEL_A::DacCh2)
    }
    #[doc = "GPIO selected with INMESEL"]
    #[inline(always)]
    pub fn gpiox(self) -> &'a mut W {
        self.variant(INMSEL_A::Gpiox)
    }
}
#[doc = "Field `INPSEL` reader - Comparator1 input plus selection bit"]
pub type INPSEL_R = crate::FieldReader<u8, INPSEL_A>;
#[doc = "Comparator1 input plus selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPSEL_A {
    #[doc = "0: PC5 connected to input plus"]
    Pc5 = 0,
    #[doc = "1: PB2 connected to input plus"]
    Pb2 = 1,
    #[doc = "2: PA3 connected to input plus"]
    Pa3 = 2,
}
impl From<INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as _
    }
}
impl INPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPSEL_A> {
        match self.bits {
            0 => Some(INPSEL_A::Pc5),
            1 => Some(INPSEL_A::Pb2),
            2 => Some(INPSEL_A::Pa3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pc5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == INPSEL_A::Pc5
    }
    #[doc = "Checks if the value of the field is `Pb2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == INPSEL_A::Pb2
    }
    #[doc = "Checks if the value of the field is `Pa3`"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == INPSEL_A::Pa3
    }
}
#[doc = "Field `INPSEL` writer - Comparator1 input plus selection bit"]
pub type INPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, INPSEL_A, 2, O>;
impl<'a, const O: u8> INPSEL_W<'a, O> {
    #[doc = "PC5 connected to input plus"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(INPSEL_A::Pc5)
    }
    #[doc = "PB2 connected to input plus"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(INPSEL_A::Pb2)
    }
    #[doc = "PA3 connected to input plus"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(INPSEL_A::Pa3)
    }
}
#[doc = "Field `POLARITY` reader - Comparator 1 polarity selection bit"]
pub type POLARITY_R = crate::BitReader<POLARITY_A>;
#[doc = "Comparator 1 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY_A {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            false => POLARITY_A::NotInverted,
            true => POLARITY_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == POLARITY_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLARITY_A::Inverted
    }
}
#[doc = "Field `POLARITY` writer - Comparator 1 polarity selection bit"]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, POLARITY_A, O>;
impl<'a, const O: u8> POLARITY_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(POLARITY_A::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(POLARITY_A::Inverted)
    }
}
#[doc = "Field `HYST` reader - Comparator 1 hysteresis selection bits"]
pub type HYST_R = crate::FieldReader<u8, HYST_A>;
#[doc = "Comparator 1 hysteresis selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST_A {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::NoHysteresis,
            1 => HYST_A::LowHysteresis,
            2 => HYST_A::MediumHysteresis,
            3 => HYST_A::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoHysteresis`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == HYST_A::NoHysteresis
    }
    #[doc = "Checks if the value of the field is `LowHysteresis`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == HYST_A::LowHysteresis
    }
    #[doc = "Checks if the value of the field is `MediumHysteresis`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == HYST_A::MediumHysteresis
    }
    #[doc = "Checks if the value of the field is `HighHysteresis`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == HYST_A::HighHysteresis
    }
}
#[doc = "Field `HYST` writer - Comparator 1 hysteresis selection bits"]
pub type HYST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP1_CSR_SPEC, u8, HYST_A, 2, O>;
impl<'a, const O: u8> HYST_W<'a, O> {
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::HighHysteresis)
    }
}
#[doc = "Field `BLANKING` reader - Comparator 1 blanking source selection bits"]
pub type BLANKING_R = crate::FieldReader<u8, BLANKING_A>;
#[doc = "Comparator 1 blanking source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLANKING_A {
    #[doc = "0: No blanking"]
    NoBlanking = 0,
    #[doc = "4: TIM15 OC1 selected as blanking source"]
    Tim1oc5 = 4,
}
impl From<BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: BLANKING_A) -> Self {
        variant as _
    }
}
impl BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLANKING_A> {
        match self.bits {
            0 => Some(BLANKING_A::NoBlanking),
            4 => Some(BLANKING_A::Tim1oc5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoBlanking`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == BLANKING_A::NoBlanking
    }
    #[doc = "Checks if the value of the field is `Tim1oc5`"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == BLANKING_A::Tim1oc5
    }
}
#[doc = "Field `BLANKING` writer - Comparator 1 blanking source selection bits"]
pub type BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, BLANKING_A, 3, O>;
impl<'a, const O: u8> BLANKING_W<'a, O> {
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(BLANKING_A::NoBlanking)
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut W {
        self.variant(BLANKING_A::Tim1oc5)
    }
}
#[doc = "Field `BRGEN` reader - Scaler bridge enable"]
pub type BRGEN_R = crate::BitReader<BRGEN_A>;
#[doc = "Scaler bridge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGEN_A {
    #[doc = "0: Scaler resistor bridge disabled"]
    Disabled = 0,
    #[doc = "1: Scaler resistor bridge enabled"]
    Enabled = 1,
}
impl From<BRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRGEN_A {
        match self.bits {
            false => BRGEN_A::Disabled,
            true => BRGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRGEN_A::Enabled
    }
}
#[doc = "Field `BRGEN` writer - Scaler bridge enable"]
pub type BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, BRGEN_A, O>;
impl<'a, const O: u8> BRGEN_W<'a, O> {
    #[doc = "Scaler resistor bridge disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BRGEN_A::Disabled)
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRGEN_A::Enabled)
    }
}
#[doc = "Field `SCALEN` reader - Voltage scaler enable bit"]
pub type SCALEN_R = crate::BitReader<SCALEN_A>;
#[doc = "Voltage scaler enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALEN_A {
    #[doc = "0: Voltage scaler disabled"]
    Disabled = 0,
    #[doc = "1: Voltage scaler enabled"]
    Enabled = 1,
}
impl From<SCALEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCALEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCALEN_A {
        match self.bits {
            false => SCALEN_A::Disabled,
            true => SCALEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCALEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCALEN_A::Enabled
    }
}
#[doc = "Field `SCALEN` writer - Voltage scaler enable bit"]
pub type SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, SCALEN_A, O>;
impl<'a, const O: u8> SCALEN_W<'a, O> {
    #[doc = "Voltage scaler disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCALEN_A::Disabled)
    }
    #[doc = "Voltage scaler enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCALEN_A::Enabled)
    }
}
#[doc = "Field `INMESEL` reader - comparator 1 input minus extended selection bits"]
pub type INMESEL_R = crate::FieldReader<u8, INMESEL_A>;
#[doc = "comparator 1 input minus extended selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INMESEL_A {
    #[doc = "0: PC4"]
    Pc4 = 0,
    #[doc = "1: PA0"]
    Pa0 = 1,
    #[doc = "2: PA4"]
    Pa4 = 2,
    #[doc = "3: PA5"]
    Pa5 = 3,
}
impl From<INMESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INMESEL_A) -> Self {
        variant as _
    }
}
impl INMESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INMESEL_A {
        match self.bits {
            0 => INMESEL_A::Pc4,
            1 => INMESEL_A::Pa0,
            2 => INMESEL_A::Pa4,
            3 => INMESEL_A::Pa5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pc4`"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == INMESEL_A::Pc4
    }
    #[doc = "Checks if the value of the field is `Pa0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == INMESEL_A::Pa0
    }
    #[doc = "Checks if the value of the field is `Pa4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == INMESEL_A::Pa4
    }
    #[doc = "Checks if the value of the field is `Pa5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == INMESEL_A::Pa5
    }
}
#[doc = "Field `INMESEL` writer - comparator 1 input minus extended selection bits"]
pub type INMESEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP1_CSR_SPEC, u8, INMESEL_A, 2, O>;
impl<'a, const O: u8> INMESEL_W<'a, O> {
    #[doc = "PC4"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut W {
        self.variant(INMESEL_A::Pc4)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(INMESEL_A::Pa0)
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(INMESEL_A::Pa4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(INMESEL_A::Pa5)
    }
}
#[doc = "Field `VALUE` reader - Comparator 1 output status bit"]
pub type VALUE_R = crate::BitReader<VALUE_A>;
#[doc = "Comparator 1 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALUE_A {
    #[doc = "0: Comparator output is low"]
    Low = 0,
    #[doc = "1: Comparator output is high"]
    High = 1,
}
impl From<VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: VALUE_A) -> Self {
        variant as u8 != 0
    }
}
impl VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALUE_A {
        match self.bits {
            false => VALUE_A::Low,
            true => VALUE_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VALUE_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VALUE_A::High
    }
}
#[doc = "Field `LOCK` writer - COMP1_CSR register lock bit"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 1"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Comparator1 input plus selection bit"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:26 - comparator 1 input minus extended selection bits"]
    #[inline(always)]
    pub fn inmesel(&self) -> INMESEL_R {
        INMESEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<2> {
        PWRMODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<4> {
        INMSEL_W::new(self)
    }
    #[doc = "Bits 7:8 - Comparator1 input plus selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<7> {
        INPSEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<15> {
        POLARITY_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<16> {
        HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn blanking(&mut self) -> BLANKING_W<18> {
        BLANKING_W::new(self)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<22> {
        BRGEN_W::new(self)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<23> {
        SCALEN_W::new(self)
    }
    #[doc = "Bits 25:26 - comparator 1 input minus extended selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn inmesel(&mut self) -> INMESEL_W<25> {
        INMESEL_W::new(self)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](index.html) module"]
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_csr::R](R) reader structure"]
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](W) writer structure"]
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
