#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRAWF` reader - Alarm A write flag"]
pub type ALRAWF_R = crate::BitReader<ALRAWFR_A>;
#[doc = "Alarm A write flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAWFR_A {
    #[doc = "0: Alarm update not allowed"]
    UpdateNotAllowed = 0,
    #[doc = "1: Alarm update allowed"]
    UpdateAllowed = 1,
}
impl From<ALRAWFR_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAWFR_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAWFR_A {
        match self.bits {
            false => ALRAWFR_A::UpdateNotAllowed,
            true => ALRAWFR_A::UpdateAllowed,
        }
    }
    #[doc = "Checks if the value of the field is `UpdateNotAllowed`"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == ALRAWFR_A::UpdateNotAllowed
    }
    #[doc = "Checks if the value of the field is `UpdateAllowed`"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == ALRAWFR_A::UpdateAllowed
    }
}
#[doc = "Field `ALRBWF` reader - Alarm B write flag"]
pub use ALRAWF_R as ALRBWF_R;
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub type WUTWF_R = crate::BitReader<WUTWFR_A>;
#[doc = "Wakeup timer write flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTWFR_A {
    #[doc = "0: Wakeup timer configuration update not allowed"]
    UpdateNotAllowed = 0,
    #[doc = "1: Wakeup timer configuration update allowed"]
    UpdateAllowed = 1,
}
impl From<WUTWFR_A> for bool {
    #[inline(always)]
    fn from(variant: WUTWFR_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTWFR_A {
        match self.bits {
            false => WUTWFR_A::UpdateNotAllowed,
            true => WUTWFR_A::UpdateAllowed,
        }
    }
    #[doc = "Checks if the value of the field is `UpdateNotAllowed`"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == WUTWFR_A::UpdateNotAllowed
    }
    #[doc = "Checks if the value of the field is `UpdateAllowed`"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == WUTWFR_A::UpdateAllowed
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub type SHPF_R = crate::BitReader<SHPFR_A>;
#[doc = "Shift operation pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHPFR_A {
    #[doc = "0: No shift operation is pending"]
    NoShiftPending = 0,
    #[doc = "1: A shift operation is pending"]
    ShiftPending = 1,
}
impl From<SHPFR_A> for bool {
    #[inline(always)]
    fn from(variant: SHPFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SHPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHPFR_A {
        match self.bits {
            false => SHPFR_A::NoShiftPending,
            true => SHPFR_A::ShiftPending,
        }
    }
    #[doc = "Checks if the value of the field is `NoShiftPending`"]
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        *self == SHPFR_A::NoShiftPending
    }
    #[doc = "Checks if the value of the field is `ShiftPending`"]
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        *self == SHPFR_A::ShiftPending
    }
}
#[doc = "Field `SHPF` writer - Shift operation pending"]
pub type SHPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, SHPFR_A, O>;
impl<'a, const O: u8> SHPF_W<'a, O> {
    #[doc = "No shift operation is pending"]
    #[inline(always)]
    pub fn no_shift_pending(self) -> &'a mut W {
        self.variant(SHPFR_A::NoShiftPending)
    }
    #[doc = "A shift operation is pending"]
    #[inline(always)]
    pub fn shift_pending(self) -> &'a mut W {
        self.variant(SHPFR_A::ShiftPending)
    }
}
#[doc = "Field `INITS` reader - Initialization status flag"]
pub type INITS_R = crate::BitReader<INITSR_A>;
#[doc = "Initialization status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITSR_A {
    #[doc = "0: Calendar has not been initialized"]
    NotInitalized = 0,
    #[doc = "1: Calendar has been initialized"]
    Initalized = 1,
}
impl From<INITSR_A> for bool {
    #[inline(always)]
    fn from(variant: INITSR_A) -> Self {
        variant as u8 != 0
    }
}
impl INITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITSR_A {
        match self.bits {
            false => INITSR_A::NotInitalized,
            true => INITSR_A::Initalized,
        }
    }
    #[doc = "Checks if the value of the field is `NotInitalized`"]
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        *self == INITSR_A::NotInitalized
    }
    #[doc = "Checks if the value of the field is `Initalized`"]
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        *self == INITSR_A::Initalized
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub type RSF_R = crate::BitReader<RSFR_A>;
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFR_A {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    NotSynced = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    Synced = 1,
}
impl From<RSFR_A> for bool {
    #[inline(always)]
    fn from(variant: RSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSFR_A {
        match self.bits {
            false => RSFR_A::NotSynced,
            true => RSFR_A::Synced,
        }
    }
    #[doc = "Checks if the value of the field is `NotSynced`"]
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        *self == RSFR_A::NotSynced
    }
    #[doc = "Checks if the value of the field is `Synced`"]
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        *self == RSFR_A::Synced
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFW_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<RSFW_AW> for bool {
    #[inline(always)]
    fn from(variant: RSFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISR_SPEC, RSFW_AW, O>;
impl<'a, const O: u8> RSF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSFW_AW::Clear)
    }
}
#[doc = "Field `INITF` reader - Initialization flag"]
pub type INITF_R = crate::BitReader<INITFR_A>;
#[doc = "Initialization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITFR_A {
    #[doc = "0: Calendar registers update is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Calendar registers update is allowed"]
    Allowed = 1,
}
impl From<INITFR_A> for bool {
    #[inline(always)]
    fn from(variant: INITFR_A) -> Self {
        variant as u8 != 0
    }
}
impl INITF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITFR_A {
        match self.bits {
            false => INITFR_A::NotAllowed,
            true => INITFR_A::Allowed,
        }
    }
    #[doc = "Checks if the value of the field is `NotAllowed`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INITFR_A::NotAllowed
    }
    #[doc = "Checks if the value of the field is `Allowed`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INITFR_A::Allowed
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub type INIT_R = crate::BitReader<INIT_A>;
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    #[doc = "0: Free running mode"]
    FreeRunningMode = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    InitMode = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::FreeRunningMode,
            true => INIT_A::InitMode,
        }
    }
    #[doc = "Checks if the value of the field is `FreeRunningMode`"]
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        *self == INIT_A::FreeRunningMode
    }
    #[doc = "Checks if the value of the field is `InitMode`"]
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        *self == INIT_A::InitMode
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, INIT_A, O>;
impl<'a, const O: u8> INIT_W<'a, O> {
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut W {
        self.variant(INIT_A::FreeRunningMode)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut W {
        self.variant(INIT_A::InitMode)
    }
}
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub type ALRAF_R = crate::BitReader<ALRAFR_A>;
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAFR_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    Match = 1,
}
impl From<ALRAFR_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAFR_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRAFR_A> {
        match self.bits {
            true => Some(ALRAFR_A::Match),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAFR_A::Match
    }
}
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAFW_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<ALRAFW_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRAFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAF` writer - Alarm A flag"]
pub type ALRAF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISR_SPEC, ALRAFW_AW, O>;
impl<'a, const O: u8> ALRAF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRAFW_AW::Clear)
    }
}
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub type ALRBF_R = crate::BitReader<ALRBFR_A>;
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBFR_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    Match = 1,
}
impl From<ALRBFR_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBFR_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRBFR_A> {
        match self.bits {
            true => Some(ALRBFR_A::Match),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRBFR_A::Match
    }
}
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBFW_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<ALRBFW_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRBFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBF` writer - Alarm B flag"]
pub type ALRBF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISR_SPEC, ALRBFW_AW, O>;
impl<'a, const O: u8> ALRBF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRBFW_AW::Clear)
    }
}
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub type WUTF_R = crate::BitReader<WUTFR_A>;
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTFR_A {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    Zero = 1,
}
impl From<WUTFR_A> for bool {
    #[inline(always)]
    fn from(variant: WUTFR_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUTFR_A> {
        match self.bits {
            true => Some(WUTFR_A::Zero),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Zero`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTFR_A::Zero
    }
}
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTFW_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<WUTFW_AW> for bool {
    #[inline(always)]
    fn from(variant: WUTFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTF` writer - Wakeup timer flag"]
pub type WUTF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISR_SPEC, WUTFW_AW, O>;
impl<'a, const O: u8> WUTF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUTFW_AW::Clear)
    }
}
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub type TSF_R = crate::BitReader<TSFR_A>;
#[doc = "Time-stamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSFR_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TimestampEvent = 1,
}
impl From<TSFR_A> for bool {
    #[inline(always)]
    fn from(variant: TSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl TSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSFR_A> {
        match self.bits {
            true => Some(TSFR_A::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TimestampEvent`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSFR_A::TimestampEvent
    }
}
#[doc = "Time-stamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSFW_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<TSFW_AW> for bool {
    #[inline(always)]
    fn from(variant: TSFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub type TSF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISR_SPEC, TSFW_AW, O>;
impl<'a, const O: u8> TSF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSFW_AW::Clear)
    }
}
#[doc = "Field `TSOVF` reader - Time-stamp overflow flag"]
pub type TSOVF_R = crate::BitReader<TSOVFR_A>;
#[doc = "Time-stamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVFR_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    Overflow = 1,
}
impl From<TSOVFR_A> for bool {
    #[inline(always)]
    fn from(variant: TSOVFR_A) -> Self {
        variant as u8 != 0
    }
}
impl TSOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSOVFR_A> {
        match self.bits {
            true => Some(TSOVFR_A::Overflow),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Overflow`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVFR_A::Overflow
    }
}
#[doc = "Time-stamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVFW_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<TSOVFW_AW> for bool {
    #[inline(always)]
    fn from(variant: TSOVFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVF` writer - Time-stamp overflow flag"]
pub type TSOVF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISR_SPEC, TSOVFW_AW, O>;
impl<'a, const O: u8> TSOVF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSOVFW_AW::Clear)
    }
}
#[doc = "Field `TAMP1F` reader - Tamper detection flag"]
pub type TAMP1F_R = crate::BitReader<TAMP1FR_A>;
#[doc = "Tamper detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1FR_A {
    #[doc = "1: This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input"]
    Tampered = 1,
}
impl From<TAMP1FR_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1FR_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TAMP1FR_A> {
        match self.bits {
            true => Some(TAMP1FR_A::Tampered),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tampered`"]
    #[inline(always)]
    pub fn is_tampered(&self) -> bool {
        *self == TAMP1FR_A::Tampered
    }
}
#[doc = "Tamper detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1FW_AW {
    #[doc = "0: Flag cleared by software writing 0"]
    Clear = 0,
}
impl From<TAMP1FW_AW> for bool {
    #[inline(always)]
    fn from(variant: TAMP1FW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1F` writer - Tamper detection flag"]
pub type TAMP1F_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISR_SPEC, TAMP1FW_AW, O>;
impl<'a, const O: u8> TAMP1F_W<'a, O> {
    #[doc = "Flag cleared by software writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TAMP1FW_AW::Clear)
    }
}
#[doc = "Field `TAMP2F` reader - RTC_TAMP2 detection flag"]
pub use TAMP1F_R as TAMP2F_R;
#[doc = "Field `TAMP3F` reader - RTC_TAMP3 detection flag"]
pub use TAMP1F_R as TAMP3F_R;
#[doc = "Field `TAMP2F` writer - RTC_TAMP2 detection flag"]
pub use TAMP1F_W as TAMP2F_W;
#[doc = "Field `TAMP3F` writer - RTC_TAMP3 detection flag"]
pub use TAMP1F_W as TAMP3F_W;
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub type RECALPF_R = crate::BitReader<RECALPFR_A>;
#[doc = "Recalibration pending Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECALPFR_A {
    #[doc = "1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    Pending = 1,
}
impl From<RECALPFR_A> for bool {
    #[inline(always)]
    fn from(variant: RECALPFR_A) -> Self {
        variant as u8 != 0
    }
}
impl RECALPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RECALPFR_A> {
        match self.bits {
            true => Some(RECALPFR_A::Pending),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECALPFR_A::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC_TAMP2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC_TAMP3 detection flag"]
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    #[must_use]
    pub fn shpf(&mut self) -> SHPF_W<3> {
        SHPF_W::new(self)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<7> {
        INIT_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn alraf(&mut self) -> ALRAF_W<8> {
        ALRAF_W::new(self)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrbf(&mut self) -> ALRBF_W<9> {
        ALRBF_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    #[must_use]
    pub fn wutf(&mut self) -> WUTF_W<10> {
        WUTF_W::new(self)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<11> {
        TSF_W::new(self)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsovf(&mut self) -> TSOVF_W<12> {
        TSOVF_W::new(self)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1f(&mut self) -> TAMP1F_W<13> {
        TAMP1F_W::new(self)
    }
    #[doc = "Bit 14 - RTC_TAMP2 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2f(&mut self) -> TAMP2F_W<14> {
        TAMP2F_W::new(self)
    }
    #[doc = "Bit 15 - RTC_TAMP3 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3f(&mut self) -> TAMP3F_W<15> {
        TAMP3F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "initialization and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff20;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0x07"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
