#[doc = "Register `TIMBCR` reader"]
pub struct R(crate::R<TIMBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMBCR` writer"]
pub struct W(crate::W<TIMBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMBCR_SPEC>;
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
impl From<crate::W<TIMBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKPSCx` reader - HRTIM Timer x Clock prescaler"]
pub type CKPSCX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKPSCx` writer - HRTIM Timer x Clock prescaler"]
pub type CKPSCX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMBCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CONT` reader - Continuous mode"]
pub type CONT_R = crate::BitReader<CONT_A>;
#[doc = "Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    #[doc = "0: The timer operates in single-shot mode and stops when it reaches the MPER value"]
    SingleShot = 0,
    #[doc = "1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
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
            false => CONT_A::SingleShot,
            true => CONT_A::Continuous,
        }
    }
    #[doc = "Checks if the value of the field is `SingleShot`"]
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == CONT_A::SingleShot
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::Continuous
    }
}
#[doc = "Field `CONT` writer - Continuous mode"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    #[doc = "The timer operates in single-shot mode and stops when it reaches the MPER value"]
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut W {
        self.variant(CONT_A::SingleShot)
    }
    #[doc = "The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::Continuous)
    }
}
#[doc = "Field `RETRIG` reader - Re-triggerable mode"]
pub type RETRIG_R = crate::BitReader<RETRIG_A>;
#[doc = "Re-triggerable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETRIG_A {
    #[doc = "0: The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    Disabled = 0,
    #[doc = "1: The timer is retriggerable: a counter reset is done whatever the counter state"]
    Enabled = 1,
}
impl From<RETRIG_A> for bool {
    #[inline(always)]
    fn from(variant: RETRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl RETRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETRIG_A {
        match self.bits {
            false => RETRIG_A::Disabled,
            true => RETRIG_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RETRIG_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RETRIG_A::Enabled
    }
}
#[doc = "Field `RETRIG` writer - Re-triggerable mode"]
pub type RETRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, RETRIG_A, O>;
impl<'a, const O: u8> RETRIG_W<'a, O> {
    #[doc = "The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RETRIG_A::Disabled)
    }
    #[doc = "The timer is retriggerable: a counter reset is done whatever the counter state"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RETRIG_A::Enabled)
    }
}
#[doc = "Field `HALF` reader - Half mode enable"]
pub type HALF_R = crate::BitReader<HALF_A>;
#[doc = "Half mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALF_A {
    #[doc = "0: Half mode disabled"]
    Disabled = 0,
    #[doc = "1: Half mode enabled"]
    Enabled = 1,
}
impl From<HALF_A> for bool {
    #[inline(always)]
    fn from(variant: HALF_A) -> Self {
        variant as u8 != 0
    }
}
impl HALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALF_A {
        match self.bits {
            false => HALF_A::Disabled,
            true => HALF_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALF_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALF_A::Enabled
    }
}
#[doc = "Field `HALF` writer - Half mode enable"]
pub type HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, HALF_A, O>;
impl<'a, const O: u8> HALF_W<'a, O> {
    #[doc = "Half mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HALF_A::Disabled)
    }
    #[doc = "Half mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HALF_A::Enabled)
    }
}
#[doc = "Field `PSHPLL` reader - Push-Pull mode enable"]
pub type PSHPLL_R = crate::BitReader<PSHPLL_A>;
#[doc = "Push-Pull mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSHPLL_A {
    #[doc = "0: Push-pull mode disabled"]
    Disabled = 0,
    #[doc = "1: Push-pull mode enabled"]
    Enabled = 1,
}
impl From<PSHPLL_A> for bool {
    #[inline(always)]
    fn from(variant: PSHPLL_A) -> Self {
        variant as u8 != 0
    }
}
impl PSHPLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSHPLL_A {
        match self.bits {
            false => PSHPLL_A::Disabled,
            true => PSHPLL_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSHPLL_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PSHPLL_A::Enabled
    }
}
#[doc = "Field `PSHPLL` writer - Push-Pull mode enable"]
pub type PSHPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, PSHPLL_A, O>;
impl<'a, const O: u8> PSHPLL_W<'a, O> {
    #[doc = "Push-pull mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PSHPLL_A::Disabled)
    }
    #[doc = "Push-pull mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PSHPLL_A::Enabled)
    }
}
#[doc = "Field `SYNCRSTx` reader - Synchronization Resets Timer x"]
pub type SYNCRSTX_R = crate::BitReader<SYNCRSTX_A>;
#[doc = "Synchronization Resets Timer x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCRSTX_A {
    #[doc = "0: Synchronization event has no effect on Timer x"]
    Disabled = 0,
    #[doc = "1: Synchronization event resets Timer x"]
    Reset = 1,
}
impl From<SYNCRSTX_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCRSTX_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCRSTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCRSTX_A {
        match self.bits {
            false => SYNCRSTX_A::Disabled,
            true => SYNCRSTX_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCRSTX_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYNCRSTX_A::Reset
    }
}
#[doc = "Field `SYNCRSTx` writer - Synchronization Resets Timer x"]
pub type SYNCRSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, SYNCRSTX_A, O>;
impl<'a, const O: u8> SYNCRSTX_W<'a, O> {
    #[doc = "Synchronization event has no effect on Timer x"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCRSTX_A::Disabled)
    }
    #[doc = "Synchronization event resets Timer x"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYNCRSTX_A::Reset)
    }
}
#[doc = "Field `SYNCSTRTx` reader - Synchronization Starts Timer x"]
pub type SYNCSTRTX_R = crate::BitReader<SYNCSTRTX_A>;
#[doc = "Synchronization Starts Timer x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCSTRTX_A {
    #[doc = "0: Synchronization event has no effect on Timer x"]
    Disabled = 0,
    #[doc = "1: Synchronization event starts Timer x"]
    Start = 1,
}
impl From<SYNCSTRTX_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCSTRTX_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCSTRTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCSTRTX_A {
        match self.bits {
            false => SYNCSTRTX_A::Disabled,
            true => SYNCSTRTX_A::Start,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCSTRTX_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SYNCSTRTX_A::Start
    }
}
#[doc = "Field `SYNCSTRTx` writer - Synchronization Starts Timer x"]
pub type SYNCSTRTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, SYNCSTRTX_A, O>;
impl<'a, const O: u8> SYNCSTRTX_W<'a, O> {
    #[doc = "Synchronization event has no effect on Timer x"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCSTRTX_A::Disabled)
    }
    #[doc = "Synchronization event starts Timer x"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SYNCSTRTX_A::Start)
    }
}
#[doc = "Field `DELCMP2` reader - Delayed CMP2 mode"]
pub type DELCMP2_R = crate::FieldReader<u8, DELCMP2_A>;
#[doc = "Delayed CMP2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELCMP2_A {
    #[doc = "0: CMP2 register is always active (standard compare mode)"]
    Standard = 0,
    #[doc = "1: CMP2 is recomputed and is active following a capture 1 event"]
    Capture1 = 1,
    #[doc = "2: CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match"]
    Capture1Compare1 = 2,
    #[doc = "3: CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match"]
    Capture1Compare3 = 3,
}
impl From<DELCMP2_A> for u8 {
    #[inline(always)]
    fn from(variant: DELCMP2_A) -> Self {
        variant as _
    }
}
impl DELCMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELCMP2_A {
        match self.bits {
            0 => DELCMP2_A::Standard,
            1 => DELCMP2_A::Capture1,
            2 => DELCMP2_A::Capture1Compare1,
            3 => DELCMP2_A::Capture1Compare3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP2_A::Standard
    }
    #[doc = "Checks if the value of the field is `Capture1`"]
    #[inline(always)]
    pub fn is_capture1(&self) -> bool {
        *self == DELCMP2_A::Capture1
    }
    #[doc = "Checks if the value of the field is `Capture1Compare1`"]
    #[inline(always)]
    pub fn is_capture1_compare1(&self) -> bool {
        *self == DELCMP2_A::Capture1Compare1
    }
    #[doc = "Checks if the value of the field is `Capture1Compare3`"]
    #[inline(always)]
    pub fn is_capture1_compare3(&self) -> bool {
        *self == DELCMP2_A::Capture1Compare3
    }
}
#[doc = "Field `DELCMP2` writer - Delayed CMP2 mode"]
pub type DELCMP2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMBCR_SPEC, u8, DELCMP2_A, 2, O>;
impl<'a, const O: u8> DELCMP2_W<'a, O> {
    #[doc = "CMP2 register is always active (standard compare mode)"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(DELCMP2_A::Standard)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event"]
    #[inline(always)]
    pub fn capture1(self) -> &'a mut W {
        self.variant(DELCMP2_A::Capture1)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match"]
    #[inline(always)]
    pub fn capture1_compare1(self) -> &'a mut W {
        self.variant(DELCMP2_A::Capture1Compare1)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match"]
    #[inline(always)]
    pub fn capture1_compare3(self) -> &'a mut W {
        self.variant(DELCMP2_A::Capture1Compare3)
    }
}
#[doc = "Field `DELCMP4` reader - Delayed CMP4 mode"]
pub type DELCMP4_R = crate::FieldReader<u8, DELCMP4_A>;
#[doc = "Delayed CMP4 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELCMP4_A {
    #[doc = "0: CMP4 register is always active (standard compare mode)"]
    Standard = 0,
    #[doc = "1: CMP4 is recomputed and is active following a capture 2 event"]
    Capture2 = 1,
    #[doc = "2: CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match"]
    Capture2Compare1 = 2,
    #[doc = "3: CMP4 is recomputed and is active following a capture event or a Compare 3 match"]
    CaptureCompare3 = 3,
}
impl From<DELCMP4_A> for u8 {
    #[inline(always)]
    fn from(variant: DELCMP4_A) -> Self {
        variant as _
    }
}
impl DELCMP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELCMP4_A {
        match self.bits {
            0 => DELCMP4_A::Standard,
            1 => DELCMP4_A::Capture2,
            2 => DELCMP4_A::Capture2Compare1,
            3 => DELCMP4_A::CaptureCompare3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP4_A::Standard
    }
    #[doc = "Checks if the value of the field is `Capture2`"]
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        *self == DELCMP4_A::Capture2
    }
    #[doc = "Checks if the value of the field is `Capture2Compare1`"]
    #[inline(always)]
    pub fn is_capture2_compare1(&self) -> bool {
        *self == DELCMP4_A::Capture2Compare1
    }
    #[doc = "Checks if the value of the field is `CaptureCompare3`"]
    #[inline(always)]
    pub fn is_capture_compare3(&self) -> bool {
        *self == DELCMP4_A::CaptureCompare3
    }
}
#[doc = "Field `DELCMP4` writer - Delayed CMP4 mode"]
pub type DELCMP4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMBCR_SPEC, u8, DELCMP4_A, 2, O>;
impl<'a, const O: u8> DELCMP4_W<'a, O> {
    #[doc = "CMP4 register is always active (standard compare mode)"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(DELCMP4_A::Standard)
    }
    #[doc = "CMP4 is recomputed and is active following a capture 2 event"]
    #[inline(always)]
    pub fn capture2(self) -> &'a mut W {
        self.variant(DELCMP4_A::Capture2)
    }
    #[doc = "CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match"]
    #[inline(always)]
    pub fn capture2_compare1(self) -> &'a mut W {
        self.variant(DELCMP4_A::Capture2Compare1)
    }
    #[doc = "CMP4 is recomputed and is active following a capture event or a Compare 3 match"]
    #[inline(always)]
    pub fn capture_compare3(self) -> &'a mut W {
        self.variant(DELCMP4_A::CaptureCompare3)
    }
}
#[doc = "Field `TxREPU` reader - Timer x Repetition update"]
pub type TX_REPU_R = crate::BitReader<TX_REPU_A>;
#[doc = "Timer x Repetition update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_REPU_A {
    #[doc = "0: Update by timer x repetition disabled"]
    Disabled = 0,
    #[doc = "1: Update by timer x repetition enabled"]
    Enabled = 1,
}
impl From<TX_REPU_A> for bool {
    #[inline(always)]
    fn from(variant: TX_REPU_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_REPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_REPU_A {
        match self.bits {
            false => TX_REPU_A::Disabled,
            true => TX_REPU_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_REPU_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_REPU_A::Enabled
    }
}
#[doc = "Field `TxREPU` writer - Timer x Repetition update"]
pub type TX_REPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, TX_REPU_A, O>;
impl<'a, const O: u8> TX_REPU_W<'a, O> {
    #[doc = "Update by timer x repetition disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TX_REPU_A::Disabled)
    }
    #[doc = "Update by timer x repetition enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TX_REPU_A::Enabled)
    }
}
#[doc = "Field `TxRSTU` reader - Timerx reset update"]
pub type TX_RSTU_R = crate::BitReader<TX_RSTU_A>;
#[doc = "Timerx reset update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_RSTU_A {
    #[doc = "0: Update by timer x reset/roll-over disabled"]
    Disabled = 0,
    #[doc = "1: Update by timer x reset/roll-over enabled"]
    Enabled = 1,
}
impl From<TX_RSTU_A> for bool {
    #[inline(always)]
    fn from(variant: TX_RSTU_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_RSTU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_RSTU_A {
        match self.bits {
            false => TX_RSTU_A::Disabled,
            true => TX_RSTU_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_RSTU_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_RSTU_A::Enabled
    }
}
#[doc = "Field `TxRSTU` writer - Timerx reset update"]
pub type TX_RSTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, TX_RSTU_A, O>;
impl<'a, const O: u8> TX_RSTU_W<'a, O> {
    #[doc = "Update by timer x reset/roll-over disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TX_RSTU_A::Disabled)
    }
    #[doc = "Update by timer x reset/roll-over enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TX_RSTU_A::Enabled)
    }
}
#[doc = "Field `TBU` reader - TBU"]
pub type TBU_R = crate::BitReader<TBU_A>;
#[doc = "TBU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBU_A {
    #[doc = "0: Update by timer x disabled"]
    Disabled = 0,
    #[doc = "1: Update by timer x enabled"]
    Enabled = 1,
}
impl From<TBU_A> for bool {
    #[inline(always)]
    fn from(variant: TBU_A) -> Self {
        variant as u8 != 0
    }
}
impl TBU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBU_A {
        match self.bits {
            false => TBU_A::Disabled,
            true => TBU_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBU_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBU_A::Enabled
    }
}
#[doc = "Field `TBU` writer - TBU"]
pub type TBU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, TBU_A, O>;
impl<'a, const O: u8> TBU_W<'a, O> {
    #[doc = "Update by timer x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TBU_A::Disabled)
    }
    #[doc = "Update by timer x enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TBU_A::Enabled)
    }
}
#[doc = "Field `TCU` reader - TCU"]
pub use TBU_R as TCU_R;
#[doc = "Field `TDU` reader - TDU"]
pub use TBU_R as TDU_R;
#[doc = "Field `TEU` reader - TEU"]
pub use TBU_R as TEU_R;
#[doc = "Field `TCU` writer - TCU"]
pub use TBU_W as TCU_W;
#[doc = "Field `TDU` writer - TDU"]
pub use TBU_W as TDU_W;
#[doc = "Field `TEU` writer - TEU"]
pub use TBU_W as TEU_W;
#[doc = "Field `MSTU` reader - Master Timer update"]
pub type MSTU_R = crate::BitReader<MSTU_A>;
#[doc = "Master Timer update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTU_A {
    #[doc = "0: Update by master timer disabled"]
    Disabled = 0,
    #[doc = "1: Update by master timer enabled"]
    Enabled = 1,
}
impl From<MSTU_A> for bool {
    #[inline(always)]
    fn from(variant: MSTU_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTU_A {
        match self.bits {
            false => MSTU_A::Disabled,
            true => MSTU_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTU_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTU_A::Enabled
    }
}
#[doc = "Field `MSTU` writer - Master Timer update"]
pub type MSTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, MSTU_A, O>;
impl<'a, const O: u8> MSTU_W<'a, O> {
    #[doc = "Update by master timer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTU_A::Disabled)
    }
    #[doc = "Update by master timer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTU_A::Enabled)
    }
}
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub type DACSYNC_R = crate::FieldReader<u8, DACSYNC_A>;
#[doc = "AC Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACSYNC_A {
    #[doc = "0: No DAC trigger generated"]
    Disabled = 0,
    #[doc = "1: Trigger generated on DACSync1"]
    Dacsync1 = 1,
    #[doc = "2: Trigger generated on DACSync2"]
    Dacsync2 = 2,
    #[doc = "3: Trigger generated on DACSync3"]
    Dacsync3 = 3,
}
impl From<DACSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: DACSYNC_A) -> Self {
        variant as _
    }
}
impl DACSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACSYNC_A {
        match self.bits {
            0 => DACSYNC_A::Disabled,
            1 => DACSYNC_A::Dacsync1,
            2 => DACSYNC_A::Dacsync2,
            3 => DACSYNC_A::Dacsync3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACSYNC_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Dacsync1`"]
    #[inline(always)]
    pub fn is_dacsync1(&self) -> bool {
        *self == DACSYNC_A::Dacsync1
    }
    #[doc = "Checks if the value of the field is `Dacsync2`"]
    #[inline(always)]
    pub fn is_dacsync2(&self) -> bool {
        *self == DACSYNC_A::Dacsync2
    }
    #[doc = "Checks if the value of the field is `Dacsync3`"]
    #[inline(always)]
    pub fn is_dacsync3(&self) -> bool {
        *self == DACSYNC_A::Dacsync3
    }
}
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub type DACSYNC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMBCR_SPEC, u8, DACSYNC_A, 2, O>;
impl<'a, const O: u8> DACSYNC_W<'a, O> {
    #[doc = "No DAC trigger generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACSYNC_A::Disabled)
    }
    #[doc = "Trigger generated on DACSync1"]
    #[inline(always)]
    pub fn dacsync1(self) -> &'a mut W {
        self.variant(DACSYNC_A::Dacsync1)
    }
    #[doc = "Trigger generated on DACSync2"]
    #[inline(always)]
    pub fn dacsync2(self) -> &'a mut W {
        self.variant(DACSYNC_A::Dacsync2)
    }
    #[doc = "Trigger generated on DACSync3"]
    #[inline(always)]
    pub fn dacsync3(self) -> &'a mut W {
        self.variant(DACSYNC_A::Dacsync3)
    }
}
#[doc = "Field `PREEN` reader - Preload enable"]
pub type PREEN_R = crate::BitReader<PREEN_A>;
#[doc = "Preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREEN_A {
    #[doc = "0: Preload disabled: the write access is directly done into the active register"]
    Disabled = 0,
    #[doc = "1: Preload enabled: the write access is done into the preload register"]
    Enabled = 1,
}
impl From<PREEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PREEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREEN_A {
        match self.bits {
            false => PREEN_A::Disabled,
            true => PREEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREEN_A::Enabled
    }
}
#[doc = "Field `PREEN` writer - Preload enable"]
pub type PREEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMBCR_SPEC, PREEN_A, O>;
impl<'a, const O: u8> PREEN_W<'a, O> {
    #[doc = "Preload disabled: the write access is directly done into the active register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREEN_A::Disabled)
    }
    #[doc = "Preload enabled: the write access is done into the preload register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREEN_A::Enabled)
    }
}
#[doc = "Field `UPDGAT` reader - Update Gating"]
pub type UPDGAT_R = crate::FieldReader<u8, UPDGAT_A>;
#[doc = "Update Gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPDGAT_A {
    #[doc = "0: Update occurs independently from the DMA burst transfer"]
    Independent = 0,
    #[doc = "1: Update occurs when the DMA burst transfer is completed"]
    Dmaburst = 1,
    #[doc = "2: Update occurs on the update event following DMA burst transfer completion"]
    DmaburstUpdate = 2,
    #[doc = "3: Update occurs on a rising edge of HRTIM update enable input 1"]
    Input1 = 3,
    #[doc = "4: Update occurs on a rising edge of HRTIM update enable input 2"]
    Input2 = 4,
    #[doc = "5: Update occurs on a rising edge of HRTIM update enable input 3"]
    Input3 = 5,
    #[doc = "6: Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    Input1Update = 6,
    #[doc = "7: Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    Input2Update = 7,
    #[doc = "8: Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    Input3Update = 8,
}
impl From<UPDGAT_A> for u8 {
    #[inline(always)]
    fn from(variant: UPDGAT_A) -> Self {
        variant as _
    }
}
impl UPDGAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPDGAT_A> {
        match self.bits {
            0 => Some(UPDGAT_A::Independent),
            1 => Some(UPDGAT_A::Dmaburst),
            2 => Some(UPDGAT_A::DmaburstUpdate),
            3 => Some(UPDGAT_A::Input1),
            4 => Some(UPDGAT_A::Input2),
            5 => Some(UPDGAT_A::Input3),
            6 => Some(UPDGAT_A::Input1Update),
            7 => Some(UPDGAT_A::Input2Update),
            8 => Some(UPDGAT_A::Input3Update),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Independent`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == UPDGAT_A::Independent
    }
    #[doc = "Checks if the value of the field is `Dmaburst`"]
    #[inline(always)]
    pub fn is_dmaburst(&self) -> bool {
        *self == UPDGAT_A::Dmaburst
    }
    #[doc = "Checks if the value of the field is `DmaburstUpdate`"]
    #[inline(always)]
    pub fn is_dmaburst_update(&self) -> bool {
        *self == UPDGAT_A::DmaburstUpdate
    }
    #[doc = "Checks if the value of the field is `Input1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == UPDGAT_A::Input1
    }
    #[doc = "Checks if the value of the field is `Input2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == UPDGAT_A::Input2
    }
    #[doc = "Checks if the value of the field is `Input3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == UPDGAT_A::Input3
    }
    #[doc = "Checks if the value of the field is `Input1Update`"]
    #[inline(always)]
    pub fn is_input1_update(&self) -> bool {
        *self == UPDGAT_A::Input1Update
    }
    #[doc = "Checks if the value of the field is `Input2Update`"]
    #[inline(always)]
    pub fn is_input2_update(&self) -> bool {
        *self == UPDGAT_A::Input2Update
    }
    #[doc = "Checks if the value of the field is `Input3Update`"]
    #[inline(always)]
    pub fn is_input3_update(&self) -> bool {
        *self == UPDGAT_A::Input3Update
    }
}
#[doc = "Field `UPDGAT` writer - Update Gating"]
pub type UPDGAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMBCR_SPEC, u8, UPDGAT_A, 4, O>;
impl<'a, const O: u8> UPDGAT_W<'a, O> {
    #[doc = "Update occurs independently from the DMA burst transfer"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(UPDGAT_A::Independent)
    }
    #[doc = "Update occurs when the DMA burst transfer is completed"]
    #[inline(always)]
    pub fn dmaburst(self) -> &'a mut W {
        self.variant(UPDGAT_A::Dmaburst)
    }
    #[doc = "Update occurs on the update event following DMA burst transfer completion"]
    #[inline(always)]
    pub fn dmaburst_update(self) -> &'a mut W {
        self.variant(UPDGAT_A::DmaburstUpdate)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 1"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(UPDGAT_A::Input1)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 2"]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(UPDGAT_A::Input2)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 3"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(UPDGAT_A::Input3)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    #[inline(always)]
    pub fn input1_update(self) -> &'a mut W {
        self.variant(UPDGAT_A::Input1Update)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    #[inline(always)]
    pub fn input2_update(self) -> &'a mut W {
        self.variant(UPDGAT_A::Input2Update)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    #[inline(always)]
    pub fn input3_update(self) -> &'a mut W {
        self.variant(UPDGAT_A::Input3Update)
    }
}
impl R {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ckpscx(&self) -> CKPSCX_R {
        CKPSCX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&self) -> SYNCRSTX_R {
        SYNCRSTX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&self) -> SYNCSTRTX_R {
        SYNCSTRTX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&self) -> TX_REPU_R {
        TX_REPU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&self) -> TX_RSTU_R {
        TX_RSTU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    pub fn teu(&self) -> TEU_R {
        TEU_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ckpscx(&mut self) -> CKPSCX_W<0> {
        CKPSCX_W::new(self)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<3> {
        CONT_W::new(self)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RETRIG_W<4> {
        RETRIG_W::new(self)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<5> {
        HALF_W::new(self)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pshpll(&mut self) -> PSHPLL_W<6> {
        PSHPLL_W::new(self)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncrstx(&mut self) -> SYNCRSTX_W<10> {
        SYNCRSTX_W::new(self)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncstrtx(&mut self) -> SYNCSTRTX_W<11> {
        SYNCSTRTX_W::new(self)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp2(&mut self) -> DELCMP2_W<12> {
        DELCMP2_W::new(self)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp4(&mut self) -> DELCMP4_W<14> {
        DELCMP4_W::new(self)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_repu(&mut self) -> TX_REPU_W<17> {
        TX_REPU_W::new(self)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rstu(&mut self) -> TX_RSTU_W<18> {
        TX_RSTU_W::new(self)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<20> {
        TBU_W::new(self)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    #[must_use]
    pub fn tcu(&mut self) -> TCU_W<21> {
        TCU_W::new(self)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    #[must_use]
    pub fn tdu(&mut self) -> TDU_W<22> {
        TDU_W::new(self)
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    #[must_use]
    pub fn teu(&mut self) -> TEU_W<23> {
        TEU_W::new(self)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    #[must_use]
    pub fn mstu(&mut self) -> MSTU_W<24> {
        MSTU_W::new(self)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DACSYNC_W<25> {
        DACSYNC_W::new(self)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<27> {
        PREEN_W::new(self)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    #[must_use]
    pub fn updgat(&mut self) -> UPDGAT_W<28> {
        UPDGAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timbcr](index.html) module"]
pub struct TIMBCR_SPEC;
impl crate::RegisterSpec for TIMBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timbcr::R](R) reader structure"]
impl crate::Readable for TIMBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timbcr::W](W) writer structure"]
impl crate::Writable for TIMBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMBCR to value 0"]
impl crate::Resettable for TIMBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
