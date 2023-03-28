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
#[doc = "Field `ADEN` reader - ADC enable"]
pub type ADEN_R = crate::BitReader<ADENR_A>;
#[doc = "ADC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENR_A {
    #[doc = "0: ADC disabled"]
    Disabled = 0,
    #[doc = "1: ADC enabled"]
    Enabled = 1,
}
impl From<ADENR_A> for bool {
    #[inline(always)]
    fn from(variant: ADENR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADENR_A {
        match self.bits {
            false => ADENR_A::Disabled,
            true => ADENR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADENR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADENR_A::Enabled
    }
}
#[doc = "ADC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW_AW {
    #[doc = "1: Enable the ADC"]
    Enabled = 1,
}
impl From<ADENW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` writer - ADC enable"]
pub type ADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADENW_AW, O>;
impl<'a, const O: u8> ADEN_W<'a, O> {
    #[doc = "Enable the ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADENW_AW::Enabled)
    }
}
#[doc = "Field `ADDIS` reader - ADC disable"]
pub type ADDIS_R = crate::BitReader<ADDISR_A>;
#[doc = "ADC disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISR_A {
    #[doc = "0: No disable command active"]
    NotDisabling = 0,
    #[doc = "1: ADC disabling"]
    Disabling = 1,
}
impl From<ADDISR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDISR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDISR_A {
        match self.bits {
            false => ADDISR_A::NotDisabling,
            true => ADDISR_A::Disabling,
        }
    }
    #[doc = "Checks if the value of the field is `NotDisabling`"]
    #[inline(always)]
    pub fn is_not_disabling(&self) -> bool {
        *self == ADDISR_A::NotDisabling
    }
    #[doc = "Checks if the value of the field is `Disabling`"]
    #[inline(always)]
    pub fn is_disabling(&self) -> bool {
        *self == ADDISR_A::Disabling
    }
}
#[doc = "ADC disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW_AW {
    #[doc = "1: Disable the ADC"]
    Disable = 1,
}
impl From<ADDISW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDISW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` writer - ADC disable"]
pub type ADDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADDISW_AW, O>;
impl<'a, const O: u8> ADDIS_W<'a, O> {
    #[doc = "Disable the ADC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADDISW_AW::Disable)
    }
}
#[doc = "Field `ADSTART` reader - ADC group regular conversion start"]
pub type ADSTART_R = crate::BitReader<ADSTARTR_A>;
#[doc = "ADC group regular conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTR_A {
    #[doc = "0: No conversion ongoing"]
    NotActive = 0,
    #[doc = "1: ADC operating and may be converting"]
    Active = 1,
}
impl From<ADSTARTR_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTARTR_A {
        match self.bits {
            false => ADSTARTR_A::NotActive,
            true => ADSTARTR_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADSTARTR_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ADSTARTR_A::Active
    }
}
#[doc = "ADC group regular conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTW_AW {
    #[doc = "1: Start the ADC conversion (may be delayed for hardware triggers)"]
    StartConversion = 1,
}
impl From<ADSTARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` writer - ADC group regular conversion start"]
pub type ADSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADSTARTW_AW, O>;
impl<'a, const O: u8> ADSTART_W<'a, O> {
    #[doc = "Start the ADC conversion (may be delayed for hardware triggers)"]
    #[inline(always)]
    pub fn start_conversion(self) -> &'a mut W {
        self.variant(ADSTARTW_AW::StartConversion)
    }
}
#[doc = "Field `JADSTART` reader - ADC group injected conversion start"]
pub use ADSTART_R as JADSTART_R;
#[doc = "Field `JADSTART` writer - ADC group injected conversion start"]
pub use ADSTART_W as JADSTART_W;
#[doc = "Field `ADSTP` reader - ADC group regular conversion stop"]
pub type ADSTP_R = crate::BitReader<ADSTPR_A>;
#[doc = "ADC group regular conversion stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPR_A {
    #[doc = "0: No stop command active"]
    NotStopping = 0,
    #[doc = "1: ADC stopping conversion"]
    Stopping = 1,
}
impl From<ADSTPR_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTPR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTPR_A {
        match self.bits {
            false => ADSTPR_A::NotStopping,
            true => ADSTPR_A::Stopping,
        }
    }
    #[doc = "Checks if the value of the field is `NotStopping`"]
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        *self == ADSTPR_A::NotStopping
    }
    #[doc = "Checks if the value of the field is `Stopping`"]
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        *self == ADSTPR_A::Stopping
    }
}
#[doc = "ADC group regular conversion stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPW_AW {
    #[doc = "1: Stop the active conversion"]
    StopConversion = 1,
}
impl From<ADSTPW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSTPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` writer - ADC group regular conversion stop"]
pub type ADSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADSTPW_AW, O>;
impl<'a, const O: u8> ADSTP_W<'a, O> {
    #[doc = "Stop the active conversion"]
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut W {
        self.variant(ADSTPW_AW::StopConversion)
    }
}
#[doc = "Field `JADSTP` reader - ADC group injected conversion stop"]
pub use ADSTP_R as JADSTP_R;
#[doc = "Field `JADSTP` writer - ADC group injected conversion stop"]
pub use ADSTP_W as JADSTP_W;
#[doc = "Field `BOOST` reader - Boost mode control"]
pub type BOOST_R = crate::FieldReader<u8, BOOST_A>;
#[doc = "Boost mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOST_A {
    #[doc = "0: Boost mode used when ADC clock ≤ 6.25 MHz"]
    Lt625 = 0,
    #[doc = "1: Boost mode used when 6.25 MHz < ADC clock ≤ 12.5 MHz"]
    Lt125 = 1,
    #[doc = "2: Boost mode used when 12.5 MHz < ADC clock ≤ 25.0 MHz"]
    Lt25 = 2,
    #[doc = "3: Boost mode used when 25.0 MHz < ADC clock ≤ 50.0 MHz"]
    Lt50 = 3,
}
impl From<BOOST_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOST_A) -> Self {
        variant as _
    }
}
impl BOOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOST_A {
        match self.bits {
            0 => BOOST_A::Lt625,
            1 => BOOST_A::Lt125,
            2 => BOOST_A::Lt25,
            3 => BOOST_A::Lt50,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Lt625`"]
    #[inline(always)]
    pub fn is_lt6_25(&self) -> bool {
        *self == BOOST_A::Lt625
    }
    #[doc = "Checks if the value of the field is `Lt125`"]
    #[inline(always)]
    pub fn is_lt12_5(&self) -> bool {
        *self == BOOST_A::Lt125
    }
    #[doc = "Checks if the value of the field is `Lt25`"]
    #[inline(always)]
    pub fn is_lt25(&self) -> bool {
        *self == BOOST_A::Lt25
    }
    #[doc = "Checks if the value of the field is `Lt50`"]
    #[inline(always)]
    pub fn is_lt50(&self) -> bool {
        *self == BOOST_A::Lt50
    }
}
#[doc = "Field `BOOST` writer - Boost mode control"]
pub type BOOST_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, BOOST_A, 2, O>;
impl<'a, const O: u8> BOOST_W<'a, O> {
    #[doc = "Boost mode used when ADC clock ≤ 6.25 MHz"]
    #[inline(always)]
    pub fn lt6_25(self) -> &'a mut W {
        self.variant(BOOST_A::Lt625)
    }
    #[doc = "Boost mode used when 6.25 MHz < ADC clock ≤ 12.5 MHz"]
    #[inline(always)]
    pub fn lt12_5(self) -> &'a mut W {
        self.variant(BOOST_A::Lt125)
    }
    #[doc = "Boost mode used when 12.5 MHz < ADC clock ≤ 25.0 MHz"]
    #[inline(always)]
    pub fn lt25(self) -> &'a mut W {
        self.variant(BOOST_A::Lt25)
    }
    #[doc = "Boost mode used when 25.0 MHz < ADC clock ≤ 50.0 MHz"]
    #[inline(always)]
    pub fn lt50(self) -> &'a mut W {
        self.variant(BOOST_A::Lt50)
    }
}
#[doc = "Field `ADCALLIN` reader - Linearity calibration"]
pub type ADCALLIN_R = crate::BitReader<ADCALLIN_A>;
#[doc = "Linearity calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALLIN_A {
    #[doc = "0: ADC calibration without linearaity calibration"]
    NoLinearity = 0,
    #[doc = "1: ADC calibration with linearaity calibration"]
    Linearity = 1,
}
impl From<ADCALLIN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALLIN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCALLIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCALLIN_A {
        match self.bits {
            false => ADCALLIN_A::NoLinearity,
            true => ADCALLIN_A::Linearity,
        }
    }
    #[doc = "Checks if the value of the field is `NoLinearity`"]
    #[inline(always)]
    pub fn is_no_linearity(&self) -> bool {
        *self == ADCALLIN_A::NoLinearity
    }
    #[doc = "Checks if the value of the field is `Linearity`"]
    #[inline(always)]
    pub fn is_linearity(&self) -> bool {
        *self == ADCALLIN_A::Linearity
    }
}
#[doc = "Field `ADCALLIN` writer - Linearity calibration"]
pub type ADCALLIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADCALLIN_A, O>;
impl<'a, const O: u8> ADCALLIN_W<'a, O> {
    #[doc = "ADC calibration without linearaity calibration"]
    #[inline(always)]
    pub fn no_linearity(self) -> &'a mut W {
        self.variant(ADCALLIN_A::NoLinearity)
    }
    #[doc = "ADC calibration with linearaity calibration"]
    #[inline(always)]
    pub fn linearity(self) -> &'a mut W {
        self.variant(ADCALLIN_A::Linearity)
    }
}
#[doc = "Field `LINCALRDYW1` reader - Linearity calibration ready Word 1"]
pub type LINCALRDYW1_R = crate::BitReader<LINCALRDYW1_A>;
#[doc = "Linearity calibration ready Word 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINCALRDYW1_A {
    #[doc = "0: LINCALFACT Word Read"]
    Reset = 0,
    #[doc = "1: LINCALFACT Word Write"]
    Set = 1,
}
impl From<LINCALRDYW1_A> for bool {
    #[inline(always)]
    fn from(variant: LINCALRDYW1_A) -> Self {
        variant as u8 != 0
    }
}
impl LINCALRDYW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINCALRDYW1_A {
        match self.bits {
            false => LINCALRDYW1_A::Reset,
            true => LINCALRDYW1_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LINCALRDYW1_A::Reset
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == LINCALRDYW1_A::Set
    }
}
#[doc = "Field `LINCALRDYW1` writer - Linearity calibration ready Word 1"]
pub type LINCALRDYW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LINCALRDYW1_A, O>;
impl<'a, const O: u8> LINCALRDYW1_W<'a, O> {
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LINCALRDYW1_A::Reset)
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LINCALRDYW1_A::Set)
    }
}
#[doc = "Field `LINCALRDYW2` reader - Linearity calibration ready Word 2"]
pub use LINCALRDYW1_R as LINCALRDYW2_R;
#[doc = "Field `LINCALRDYW3` reader - Linearity calibration ready Word 3"]
pub use LINCALRDYW1_R as LINCALRDYW3_R;
#[doc = "Field `LINCALRDYW4` reader - Linearity calibration ready Word 4"]
pub use LINCALRDYW1_R as LINCALRDYW4_R;
#[doc = "Field `LINCALRDYW5` reader - Linearity calibration ready Word 5"]
pub use LINCALRDYW1_R as LINCALRDYW5_R;
#[doc = "Field `LINCALRDYW6` reader - Linearity calibration ready Word 6"]
pub use LINCALRDYW1_R as LINCALRDYW6_R;
#[doc = "Field `LINCALRDYW2` writer - Linearity calibration ready Word 2"]
pub use LINCALRDYW1_W as LINCALRDYW2_W;
#[doc = "Field `LINCALRDYW3` writer - Linearity calibration ready Word 3"]
pub use LINCALRDYW1_W as LINCALRDYW3_W;
#[doc = "Field `LINCALRDYW4` writer - Linearity calibration ready Word 4"]
pub use LINCALRDYW1_W as LINCALRDYW4_W;
#[doc = "Field `LINCALRDYW5` writer - Linearity calibration ready Word 5"]
pub use LINCALRDYW1_W as LINCALRDYW5_W;
#[doc = "Field `LINCALRDYW6` writer - Linearity calibration ready Word 6"]
pub use LINCALRDYW1_W as LINCALRDYW6_W;
#[doc = "Field `ADVREGEN` reader - ADC voltage regulator enable"]
pub type ADVREGEN_R = crate::BitReader<ADVREGEN_A>;
#[doc = "ADC voltage regulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVREGEN_A {
    #[doc = "0: ADC voltage regulator disabled"]
    Disabled = 0,
    #[doc = "1: ADC voltage regulator enabled"]
    Enabled = 1,
}
impl From<ADVREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADVREGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVREGEN_A {
        match self.bits {
            false => ADVREGEN_A::Disabled,
            true => ADVREGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN_A::Enabled
    }
}
#[doc = "Field `ADVREGEN` writer - ADC voltage regulator enable"]
pub type ADVREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADVREGEN_A, O>;
impl<'a, const O: u8> ADVREGEN_W<'a, O> {
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::Disabled)
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::Enabled)
    }
}
#[doc = "Field `DEEPPWD` reader - ADC deep power down enable"]
pub type DEEPPWD_R = crate::BitReader<DEEPPWD_A>;
#[doc = "ADC deep power down enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPPWD_A {
    #[doc = "0: ADC not in deep power down"]
    PowerUp = 0,
    #[doc = "1: ADC in deep power down"]
    PowerDown = 1,
}
impl From<DEEPPWD_A> for bool {
    #[inline(always)]
    fn from(variant: DEEPPWD_A) -> Self {
        variant as u8 != 0
    }
}
impl DEEPPWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPPWD_A {
        match self.bits {
            false => DEEPPWD_A::PowerUp,
            true => DEEPPWD_A::PowerDown,
        }
    }
    #[doc = "Checks if the value of the field is `PowerUp`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == DEEPPWD_A::PowerUp
    }
    #[doc = "Checks if the value of the field is `PowerDown`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == DEEPPWD_A::PowerDown
    }
}
#[doc = "Field `DEEPPWD` writer - ADC deep power down enable"]
pub type DEEPPWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DEEPPWD_A, O>;
impl<'a, const O: u8> DEEPPWD_W<'a, O> {
    #[doc = "ADC not in deep power down"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(DEEPPWD_A::PowerUp)
    }
    #[doc = "ADC in deep power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(DEEPPWD_A::PowerDown)
    }
}
#[doc = "Field `ADCALDIF` reader - ADC differential mode for calibration"]
pub type ADCALDIF_R = crate::BitReader<ADCALDIF_A>;
#[doc = "ADC differential mode for calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALDIF_A {
    #[doc = "0: Calibration for single-ended mode"]
    SingleEnded = 0,
    #[doc = "1: Calibration for differential mode"]
    Differential = 1,
}
impl From<ADCALDIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCALDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCALDIF_A {
        match self.bits {
            false => ADCALDIF_A::SingleEnded,
            true => ADCALDIF_A::Differential,
        }
    }
    #[doc = "Checks if the value of the field is `SingleEnded`"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == ADCALDIF_A::SingleEnded
    }
    #[doc = "Checks if the value of the field is `Differential`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == ADCALDIF_A::Differential
    }
}
#[doc = "Field `ADCALDIF` writer - ADC differential mode for calibration"]
pub type ADCALDIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADCALDIF_A, O>;
impl<'a, const O: u8> ADCALDIF_W<'a, O> {
    #[doc = "Calibration for single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(ADCALDIF_A::SingleEnded)
    }
    #[doc = "Calibration for differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(ADCALDIF_A::Differential)
    }
}
#[doc = "Field `ADCAL` reader - ADC calibration"]
pub type ADCAL_R = crate::BitReader<ADCAL_A>;
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCAL_A {
    #[doc = "0: Calibration complete"]
    Complete = 0,
    #[doc = "1: Start the calibration of the ADC"]
    Calibration = 1,
}
impl From<ADCAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADCAL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCAL_A {
        match self.bits {
            false => ADCAL_A::Complete,
            true => ADCAL_A::Calibration,
        }
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == ADCAL_A::Complete
    }
    #[doc = "Checks if the value of the field is `Calibration`"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == ADCAL_A::Calibration
    }
}
#[doc = "Field `ADCAL` writer - ADC calibration"]
pub type ADCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADCAL_A, O>;
impl<'a, const O: u8> ADCAL_W<'a, O> {
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(ADCAL_A::Complete)
    }
    #[doc = "Start the calibration of the ADC"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(ADCAL_A::Calibration)
    }
}
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Boost mode control"]
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Linearity calibration"]
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - Linearity calibration ready Word 1"]
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> LINCALRDYW1_R {
        LINCALRDYW1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Linearity calibration ready Word 2"]
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> LINCALRDYW2_R {
        LINCALRDYW2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Linearity calibration ready Word 3"]
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> LINCALRDYW3_R {
        LINCALRDYW3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Linearity calibration ready Word 4"]
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> LINCALRDYW4_R {
        LINCALRDYW4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Linearity calibration ready Word 5"]
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> LINCALRDYW5_R {
        LINCALRDYW5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Linearity calibration ready Word 6"]
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> LINCALRDYW6_R {
        LINCALRDYW6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<0> {
        ADEN_W::new(self)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<1> {
        ADDIS_W::new(self)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<2> {
        ADSTART_W::new(self)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<3> {
        JADSTART_W::new(self)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<4> {
        ADSTP_W::new(self)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<5> {
        JADSTP_W::new(self)
    }
    #[doc = "Bits 8:9 - Boost mode control"]
    #[inline(always)]
    #[must_use]
    pub fn boost(&mut self) -> BOOST_W<8> {
        BOOST_W::new(self)
    }
    #[doc = "Bit 16 - Linearity calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcallin(&mut self) -> ADCALLIN_W<16> {
        ADCALLIN_W::new(self)
    }
    #[doc = "Bit 22 - Linearity calibration ready Word 1"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw1(&mut self) -> LINCALRDYW1_W<22> {
        LINCALRDYW1_W::new(self)
    }
    #[doc = "Bit 23 - Linearity calibration ready Word 2"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw2(&mut self) -> LINCALRDYW2_W<23> {
        LINCALRDYW2_W::new(self)
    }
    #[doc = "Bit 24 - Linearity calibration ready Word 3"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw3(&mut self) -> LINCALRDYW3_W<24> {
        LINCALRDYW3_W::new(self)
    }
    #[doc = "Bit 25 - Linearity calibration ready Word 4"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw4(&mut self) -> LINCALRDYW4_W<25> {
        LINCALRDYW4_W::new(self)
    }
    #[doc = "Bit 26 - Linearity calibration ready Word 5"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw5(&mut self) -> LINCALRDYW5_W<26> {
        LINCALRDYW5_W::new(self)
    }
    #[doc = "Bit 27 - Linearity calibration ready Word 6"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw6(&mut self) -> LINCALRDYW6_W<27> {
        LINCALRDYW6_W::new(self)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<28> {
        ADVREGEN_W::new(self)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<29> {
        DEEPPWD_W::new(self)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<30> {
        ADCALDIF_W::new(self)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<31> {
        ADCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0x2000_0000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
