#[doc = "Register `OUTAR` reader"]
pub struct R(crate::R<OUTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTAR` writer"]
pub struct W(crate::W<OUTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTAR_SPEC>;
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
impl From<crate::W<OUTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL1` reader - Output 1 polarity"]
pub type POL1_R = crate::BitReader<POL1_A>;
#[doc = "Output 1 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL1_A {
    #[doc = "0: Positive polarity (output active high)"]
    ActiveHigh = 0,
    #[doc = "1: Negative polarity (output active low)"]
    ActiveLow = 1,
}
impl From<POL1_A> for bool {
    #[inline(always)]
    fn from(variant: POL1_A) -> Self {
        variant as u8 != 0
    }
}
impl POL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL1_A {
        match self.bits {
            false => POL1_A::ActiveHigh,
            true => POL1_A::ActiveLow,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == POL1_A::ActiveHigh
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == POL1_A::ActiveLow
    }
}
#[doc = "Field `POL1` writer - Output 1 polarity"]
pub type POL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTAR_SPEC, POL1_A, O>;
impl<'a, const O: u8> POL1_W<'a, O> {
    #[doc = "Positive polarity (output active high)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(POL1_A::ActiveHigh)
    }
    #[doc = "Negative polarity (output active low)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(POL1_A::ActiveLow)
    }
}
#[doc = "Field `IDLEM1` reader - Output 1 Idle mode"]
pub type IDLEM1_R = crate::BitReader<IDLEM1_A>;
#[doc = "Output 1 Idle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEM1_A {
    #[doc = "0: No action: the output is not affected by the burst mode operation"]
    NoEffect = 0,
    #[doc = "1: The output is in idle state when requested by the burst mode controller"]
    SetIdle = 1,
}
impl From<IDLEM1_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEM1_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLEM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLEM1_A {
        match self.bits {
            false => IDLEM1_A::NoEffect,
            true => IDLEM1_A::SetIdle,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IDLEM1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetIdle`"]
    #[inline(always)]
    pub fn is_set_idle(&self) -> bool {
        *self == IDLEM1_A::SetIdle
    }
}
#[doc = "Field `IDLEM1` writer - Output 1 Idle mode"]
pub type IDLEM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTAR_SPEC, IDLEM1_A, O>;
impl<'a, const O: u8> IDLEM1_W<'a, O> {
    #[doc = "No action: the output is not affected by the burst mode operation"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(IDLEM1_A::NoEffect)
    }
    #[doc = "The output is in idle state when requested by the burst mode controller"]
    #[inline(always)]
    pub fn set_idle(self) -> &'a mut W {
        self.variant(IDLEM1_A::SetIdle)
    }
}
#[doc = "Field `IDLES1` reader - Output 1 Idle State"]
pub type IDLES1_R = crate::BitReader<IDLES1_A>;
#[doc = "Output 1 Idle State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLES1_A {
    #[doc = "0: Output idle state is inactive"]
    Inactive = 0,
    #[doc = "1: Output idle state is active"]
    Active = 1,
}
impl From<IDLES1_A> for bool {
    #[inline(always)]
    fn from(variant: IDLES1_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLES1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLES1_A {
        match self.bits {
            false => IDLES1_A::Inactive,
            true => IDLES1_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `Inactive`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IDLES1_A::Inactive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IDLES1_A::Active
    }
}
#[doc = "Field `IDLES1` writer - Output 1 Idle State"]
pub type IDLES1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTAR_SPEC, IDLES1_A, O>;
impl<'a, const O: u8> IDLES1_W<'a, O> {
    #[doc = "Output idle state is inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(IDLES1_A::Inactive)
    }
    #[doc = "Output idle state is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(IDLES1_A::Active)
    }
}
#[doc = "Field `FAULT1` reader - Output 1 Fault state"]
pub type FAULT1_R = crate::FieldReader<u8, FAULT1_A>;
#[doc = "Output 1 Fault state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAULT1_A {
    #[doc = "0: No action: the output is not affected by the fault input and stays in run mode"]
    Disabled = 0,
    #[doc = "1: Output goes to active state after a fault event"]
    SetActive = 1,
    #[doc = "2: Output goes to inactive state after a fault event"]
    SetInactive = 2,
    #[doc = "3: Output goes to high-z state after a fault event"]
    SetHighZ = 3,
}
impl From<FAULT1_A> for u8 {
    #[inline(always)]
    fn from(variant: FAULT1_A) -> Self {
        variant as _
    }
}
impl FAULT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT1_A {
        match self.bits {
            0 => FAULT1_A::Disabled,
            1 => FAULT1_A::SetActive,
            2 => FAULT1_A::SetInactive,
            3 => FAULT1_A::SetHighZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAULT1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `SetActive`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == FAULT1_A::SetActive
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == FAULT1_A::SetInactive
    }
    #[doc = "Checks if the value of the field is `SetHighZ`"]
    #[inline(always)]
    pub fn is_set_high_z(&self) -> bool {
        *self == FAULT1_A::SetHighZ
    }
}
#[doc = "Field `FAULT1` writer - Output 1 Fault state"]
pub type FAULT1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OUTAR_SPEC, u8, FAULT1_A, 2, O>;
impl<'a, const O: u8> FAULT1_W<'a, O> {
    #[doc = "No action: the output is not affected by the fault input and stays in run mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FAULT1_A::Disabled)
    }
    #[doc = "Output goes to active state after a fault event"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(FAULT1_A::SetActive)
    }
    #[doc = "Output goes to inactive state after a fault event"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(FAULT1_A::SetInactive)
    }
    #[doc = "Output goes to high-z state after a fault event"]
    #[inline(always)]
    pub fn set_high_z(self) -> &'a mut W {
        self.variant(FAULT1_A::SetHighZ)
    }
}
#[doc = "Field `CHP1` reader - Output 1 Chopper enable"]
pub type CHP1_R = crate::BitReader<CHP1_A>;
#[doc = "Output 1 Chopper enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHP1_A {
    #[doc = "0: Output signal not altered"]
    Disabled = 0,
    #[doc = "1: Output signal is chopped by a carrier signal"]
    Enabled = 1,
}
impl From<CHP1_A> for bool {
    #[inline(always)]
    fn from(variant: CHP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHP1_A {
        match self.bits {
            false => CHP1_A::Disabled,
            true => CHP1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHP1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHP1_A::Enabled
    }
}
#[doc = "Field `CHP1` writer - Output 1 Chopper enable"]
pub type CHP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTAR_SPEC, CHP1_A, O>;
impl<'a, const O: u8> CHP1_W<'a, O> {
    #[doc = "Output signal not altered"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CHP1_A::Disabled)
    }
    #[doc = "Output signal is chopped by a carrier signal"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CHP1_A::Enabled)
    }
}
#[doc = "Field `DIDL1` reader - Output 1 Deadtime upon burst mode Idle entry"]
pub type DIDL1_R = crate::BitReader<DIDL1_A>;
#[doc = "Output 1 Deadtime upon burst mode Idle entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIDL1_A {
    #[doc = "0: The programmed idle state is applied immediately to the output"]
    Disabled = 0,
    #[doc = "1: Deadtime (inactive level) is inserted on output before entering the idle mode"]
    Enabled = 1,
}
impl From<DIDL1_A> for bool {
    #[inline(always)]
    fn from(variant: DIDL1_A) -> Self {
        variant as u8 != 0
    }
}
impl DIDL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIDL1_A {
        match self.bits {
            false => DIDL1_A::Disabled,
            true => DIDL1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIDL1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIDL1_A::Enabled
    }
}
#[doc = "Field `DIDL1` writer - Output 1 Deadtime upon burst mode Idle entry"]
pub type DIDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTAR_SPEC, DIDL1_A, O>;
impl<'a, const O: u8> DIDL1_W<'a, O> {
    #[doc = "The programmed idle state is applied immediately to the output"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIDL1_A::Disabled)
    }
    #[doc = "Deadtime (inactive level) is inserted on output before entering the idle mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIDL1_A::Enabled)
    }
}
#[doc = "Field `DTEN` reader - Deadtime enable"]
pub type DTEN_R = crate::BitReader<DTEN_A>;
#[doc = "Deadtime enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN_A {
    #[doc = "0: Output 1 and 2 signals are independent"]
    Disabled = 0,
    #[doc = "1: Deadtime is inserted between output 1 and output 2"]
    Enabled = 1,
}
impl From<DTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::Disabled,
            true => DTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN_A::Enabled
    }
}
#[doc = "Field `DTEN` writer - Deadtime enable"]
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTAR_SPEC, DTEN_A, O>;
impl<'a, const O: u8> DTEN_W<'a, O> {
    #[doc = "Output 1 and 2 signals are independent"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTEN_A::Disabled)
    }
    #[doc = "Deadtime is inserted between output 1 and output 2"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTEN_A::Enabled)
    }
}
#[doc = "Field `DLYPRTEN` reader - Delayed Protection Enable"]
pub type DLYPRTEN_R = crate::BitReader<DLYPRTEN_A>;
#[doc = "Delayed Protection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTEN_A {
    #[doc = "0: No action"]
    Disabled = 0,
    #[doc = "1: Delayed protection is enabled, as per DLYPRT bits"]
    Enabled = 1,
}
impl From<DLYPRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYPRTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYPRTEN_A {
        match self.bits {
            false => DLYPRTEN_A::Disabled,
            true => DLYPRTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTEN_A::Enabled
    }
}
#[doc = "Field `DLYPRTEN` writer - Delayed Protection Enable"]
pub type DLYPRTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTAR_SPEC, DLYPRTEN_A, O>;
impl<'a, const O: u8> DLYPRTEN_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLYPRTEN_A::Disabled)
    }
    #[doc = "Delayed protection is enabled, as per DLYPRT bits"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLYPRTEN_A::Enabled)
    }
}
#[doc = "Field `DLYPRT` reader - Delayed Protection"]
pub type DLYPRT_R = crate::FieldReader<u8, DLYPRT_A>;
#[doc = "Delayed Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYPRT_A {
    #[doc = "0: Output 1 delayed idle on external event 6"]
    Output1Ee6 = 0,
    #[doc = "1: Output 2 delayed idle on external event 6"]
    Output2Ee6 = 1,
    #[doc = "2: Output 1 and 2 delayed idle on external event 6"]
    Output12Ee6 = 2,
    #[doc = "3: Balanced idle on external event 6"]
    BalancedEe6 = 3,
    #[doc = "4: Output 1 delayed idle on external event 7"]
    Output1Ee7 = 4,
    #[doc = "5: Output 2 delayed idle on external event 7"]
    Output2Ee7 = 5,
    #[doc = "6: Output 1 and 2 delayed idle on external event 7"]
    Output12Ee7 = 6,
    #[doc = "7: Balanced idle on external event 7"]
    BalancedEe7 = 7,
}
impl From<DLYPRT_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYPRT_A) -> Self {
        variant as _
    }
}
impl DLYPRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYPRT_A {
        match self.bits {
            0 => DLYPRT_A::Output1Ee6,
            1 => DLYPRT_A::Output2Ee6,
            2 => DLYPRT_A::Output12Ee6,
            3 => DLYPRT_A::BalancedEe6,
            4 => DLYPRT_A::Output1Ee7,
            5 => DLYPRT_A::Output2Ee7,
            6 => DLYPRT_A::Output12Ee7,
            7 => DLYPRT_A::BalancedEe7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Output1Ee6`"]
    #[inline(always)]
    pub fn is_output1_ee6(&self) -> bool {
        *self == DLYPRT_A::Output1Ee6
    }
    #[doc = "Checks if the value of the field is `Output2Ee6`"]
    #[inline(always)]
    pub fn is_output2_ee6(&self) -> bool {
        *self == DLYPRT_A::Output2Ee6
    }
    #[doc = "Checks if the value of the field is `Output12Ee6`"]
    #[inline(always)]
    pub fn is_output1_2_ee6(&self) -> bool {
        *self == DLYPRT_A::Output12Ee6
    }
    #[doc = "Checks if the value of the field is `BalancedEe6`"]
    #[inline(always)]
    pub fn is_balanced_ee6(&self) -> bool {
        *self == DLYPRT_A::BalancedEe6
    }
    #[doc = "Checks if the value of the field is `Output1Ee7`"]
    #[inline(always)]
    pub fn is_output1_ee7(&self) -> bool {
        *self == DLYPRT_A::Output1Ee7
    }
    #[doc = "Checks if the value of the field is `Output2Ee7`"]
    #[inline(always)]
    pub fn is_output2_ee7(&self) -> bool {
        *self == DLYPRT_A::Output2Ee7
    }
    #[doc = "Checks if the value of the field is `Output12Ee7`"]
    #[inline(always)]
    pub fn is_output1_2_ee7(&self) -> bool {
        *self == DLYPRT_A::Output12Ee7
    }
    #[doc = "Checks if the value of the field is `BalancedEe7`"]
    #[inline(always)]
    pub fn is_balanced_ee7(&self) -> bool {
        *self == DLYPRT_A::BalancedEe7
    }
}
#[doc = "Field `DLYPRT` writer - Delayed Protection"]
pub type DLYPRT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OUTAR_SPEC, u8, DLYPRT_A, 3, O>;
impl<'a, const O: u8> DLYPRT_W<'a, O> {
    #[doc = "Output 1 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output1_ee6(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output1Ee6)
    }
    #[doc = "Output 2 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output2_ee6(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output2Ee6)
    }
    #[doc = "Output 1 and 2 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output1_2_ee6(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output12Ee6)
    }
    #[doc = "Balanced idle on external event 6"]
    #[inline(always)]
    pub fn balanced_ee6(self) -> &'a mut W {
        self.variant(DLYPRT_A::BalancedEe6)
    }
    #[doc = "Output 1 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output1_ee7(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output1Ee7)
    }
    #[doc = "Output 2 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output2_ee7(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output2Ee7)
    }
    #[doc = "Output 1 and 2 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output1_2_ee7(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output12Ee7)
    }
    #[doc = "Balanced idle on external event 7"]
    #[inline(always)]
    pub fn balanced_ee7(self) -> &'a mut W {
        self.variant(DLYPRT_A::BalancedEe7)
    }
}
#[doc = "Field `CHP2` reader - Output 2 Chopper enable"]
pub use CHP1_R as CHP2_R;
#[doc = "Field `CHP2` writer - Output 2 Chopper enable"]
pub use CHP1_W as CHP2_W;
#[doc = "Field `DIDL2` reader - Output 2 Deadtime upon burst mode Idle entry"]
pub use DIDL1_R as DIDL2_R;
#[doc = "Field `DIDL2` writer - Output 2 Deadtime upon burst mode Idle entry"]
pub use DIDL1_W as DIDL2_W;
#[doc = "Field `FAULT2` reader - Output 2 Fault state"]
pub use FAULT1_R as FAULT2_R;
#[doc = "Field `FAULT2` writer - Output 2 Fault state"]
pub use FAULT1_W as FAULT2_W;
#[doc = "Field `IDLEM2` reader - Output 2 Idle mode"]
pub use IDLEM1_R as IDLEM2_R;
#[doc = "Field `IDLEM2` writer - Output 2 Idle mode"]
pub use IDLEM1_W as IDLEM2_W;
#[doc = "Field `IDLES2` reader - Output 2 Idle State"]
pub use IDLES1_R as IDLES2_R;
#[doc = "Field `IDLES2` writer - Output 2 Idle State"]
pub use IDLES1_W as IDLES2_W;
#[doc = "Field `POL2` reader - Output 2 polarity"]
pub use POL1_R as POL2_R;
#[doc = "Field `POL2` writer - Output 2 polarity"]
pub use POL1_W as POL2_W;
impl R {
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&self) -> IDLEM1_R {
        IDLEM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&self) -> IDLES1_R {
        IDLES1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&self) -> CHP1_R {
        CHP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&self) -> DIDL1_R {
        DIDL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&self) -> DLYPRTEN_R {
        DLYPRTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&self) -> IDLEM2_R {
        IDLEM2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&self) -> IDLES2_R {
        IDLES2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&self) -> CHP2_R {
        CHP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&self) -> DIDL2_R {
        DIDL2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> POL1_W<1> {
        POL1_W::new(self)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn idlem1(&mut self) -> IDLEM1_W<2> {
        IDLEM1_W::new(self)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    #[must_use]
    pub fn idles1(&mut self) -> IDLES1_W<3> {
        IDLES1_W::new(self)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<4> {
        FAULT1_W::new(self)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    #[must_use]
    pub fn chp1(&mut self) -> CHP1_W<6> {
        CHP1_W::new(self)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    #[must_use]
    pub fn didl1(&mut self) -> DIDL1_W<7> {
        DIDL1_W::new(self)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<8> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprten(&mut self) -> DLYPRTEN_W<9> {
        DLYPRTEN_W::new(self)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprt(&mut self) -> DLYPRT_W<10> {
        DLYPRT_W::new(self)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> POL2_W<17> {
        POL2_W::new(self)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn idlem2(&mut self) -> IDLEM2_W<18> {
        IDLEM2_W::new(self)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    #[must_use]
    pub fn idles2(&mut self) -> IDLES2_W<19> {
        IDLES2_W::new(self)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    #[must_use]
    pub fn fault2(&mut self) -> FAULT2_W<20> {
        FAULT2_W::new(self)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    #[must_use]
    pub fn chp2(&mut self) -> CHP2_W<22> {
        CHP2_W::new(self)
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    #[must_use]
    pub fn didl2(&mut self) -> DIDL2_W<23> {
        DIDL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outar](index.html) module"]
pub struct OUTAR_SPEC;
impl crate::RegisterSpec for OUTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outar::R](R) reader structure"]
impl crate::Readable for OUTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outar::W](W) writer structure"]
impl crate::Writable for OUTAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTAR to value 0"]
impl crate::Resettable for OUTAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
