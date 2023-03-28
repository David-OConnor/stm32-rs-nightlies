#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR1` writer"]
pub struct W(crate::W<SR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR1_SPEC>;
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
impl From<crate::W<SR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB` reader - Start bit (Master mode)"]
pub type SB_R = crate::BitReader<SB_A>;
#[doc = "Start bit (Master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SB_A {
    #[doc = "0: No Start condition"]
    NoStart = 0,
    #[doc = "1: Start condition generated"]
    Start = 1,
}
impl From<SB_A> for bool {
    #[inline(always)]
    fn from(variant: SB_A) -> Self {
        variant as u8 != 0
    }
}
impl SB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SB_A {
        match self.bits {
            false => SB_A::NoStart,
            true => SB_A::Start,
        }
    }
    #[doc = "Checks if the value of the field is `NoStart`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == SB_A::NoStart
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SB_A::Start
    }
}
#[doc = "Field `ADDR` reader - Address sent (master mode)/matched (slave mode)"]
pub type ADDR_R = crate::BitReader<ADDR_A>;
#[doc = "Address sent (master mode)/matched (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_A {
    #[doc = "0: Adress mismatched or not received"]
    NotMatch = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    Match = 1,
}
impl From<ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_A {
        match self.bits {
            false => ADDR_A::NotMatch,
            true => ADDR_A::Match,
        }
    }
    #[doc = "Checks if the value of the field is `NotMatch`"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR_A::NotMatch
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDR_A::Match
    }
}
#[doc = "Field `BTF` reader - Byte transfer finished"]
pub type BTF_R = crate::BitReader<BTF_A>;
#[doc = "Byte transfer finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BTF_A {
    #[doc = "0: Data byte transfer not done"]
    NotFinished = 0,
    #[doc = "1: Data byte transfer successful"]
    Finished = 1,
}
impl From<BTF_A> for bool {
    #[inline(always)]
    fn from(variant: BTF_A) -> Self {
        variant as u8 != 0
    }
}
impl BTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTF_A {
        match self.bits {
            false => BTF_A::NotFinished,
            true => BTF_A::Finished,
        }
    }
    #[doc = "Checks if the value of the field is `NotFinished`"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == BTF_A::NotFinished
    }
    #[doc = "Checks if the value of the field is `Finished`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == BTF_A::Finished
    }
}
#[doc = "Field `ADD10` reader - 10-bit header sent (Master mode)"]
pub type ADD10_R = crate::BitReader<bool>;
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type STOPF_R = crate::BitReader<STOPF_A>;
#[doc = "Stop detection (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF_A {
    #[doc = "0: No Stop condition detected"]
    NoStop = 0,
    #[doc = "1: Stop condition detected"]
    Stop = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NoStop,
            true => STOPF_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `NoStop`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF_A::NoStop
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF_A::Stop
    }
}
#[doc = "Field `RxNE` reader - Data register not empty (receivers)"]
pub type RX_NE_R = crate::BitReader<RX_NE_A>;
#[doc = "Data register not empty (receivers)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_NE_A {
    #[doc = "0: Data register empty"]
    Empty = 0,
    #[doc = "1: Data register not empty"]
    NotEmpty = 1,
}
impl From<RX_NE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_NE_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_NE_A {
        match self.bits {
            false => RX_NE_A::Empty,
            true => RX_NE_A::NotEmpty,
        }
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RX_NE_A::Empty
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RX_NE_A::NotEmpty
    }
}
#[doc = "Field `TxE` reader - Data register empty (transmitters)"]
pub type TX_E_R = crate::BitReader<TX_E_A>;
#[doc = "Data register empty (transmitters)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_E_A {
    #[doc = "0: Data register not empty"]
    NotEmpty = 0,
    #[doc = "1: Data register empty"]
    Empty = 1,
}
impl From<TX_E_A> for bool {
    #[inline(always)]
    fn from(variant: TX_E_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_E_A {
        match self.bits {
            false => TX_E_A::NotEmpty,
            true => TX_E_A::Empty,
        }
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TX_E_A::NotEmpty
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TX_E_A::Empty
    }
}
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader<BERRR_A>;
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRR_A {
    #[doc = "0: No misplaced Start or Stop condition"]
    NoError = 0,
    #[doc = "1: Misplaced Start or Stop condition"]
    Error = 1,
}
impl From<BERRR_A> for bool {
    #[inline(always)]
    fn from(variant: BERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl BERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BERRR_A {
        match self.bits {
            false => BERRR_A::NoError,
            true => BERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERRR_A::Error
    }
}
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<BERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: BERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERR` writer - Bus error"]
pub type BERR_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR1_SPEC, BERRW_AW, O>;
impl<'a, const O: u8> BERR_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BERRW_AW::Clear)
    }
}
#[doc = "Field `ARLO` reader - Arbitration lost (master mode)"]
pub type ARLO_R = crate::BitReader<ARLOR_A>;
#[doc = "Arbitration lost (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOR_A {
    #[doc = "0: No Arbitration Lost detected"]
    NoLost = 0,
    #[doc = "1: Arbitration Lost detected"]
    Lost = 1,
}
impl From<ARLOR_A> for bool {
    #[inline(always)]
    fn from(variant: ARLOR_A) -> Self {
        variant as u8 != 0
    }
}
impl ARLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARLOR_A {
        match self.bits {
            false => ARLOR_A::NoLost,
            true => ARLOR_A::Lost,
        }
    }
    #[doc = "Checks if the value of the field is `NoLost`"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == ARLOR_A::NoLost
    }
    #[doc = "Checks if the value of the field is `Lost`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLOR_A::Lost
    }
}
#[doc = "Arbitration lost (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<ARLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: ARLOW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLO` writer - Arbitration lost (master mode)"]
pub type ARLO_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR1_SPEC, ARLOW_AW, O>;
impl<'a, const O: u8> ARLO_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARLOW_AW::Clear)
    }
}
#[doc = "Field `AF` reader - Acknowledge failure"]
pub type AF_R = crate::BitReader<AFR_A>;
#[doc = "Acknowledge failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFR_A {
    #[doc = "0: No acknowledge failure"]
    NoFailure = 0,
    #[doc = "1: Acknowledge failure"]
    Failure = 1,
}
impl From<AFR_A> for bool {
    #[inline(always)]
    fn from(variant: AFR_A) -> Self {
        variant as u8 != 0
    }
}
impl AF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFR_A {
        match self.bits {
            false => AFR_A::NoFailure,
            true => AFR_A::Failure,
        }
    }
    #[doc = "Checks if the value of the field is `NoFailure`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == AFR_A::NoFailure
    }
    #[doc = "Checks if the value of the field is `Failure`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == AFR_A::Failure
    }
}
#[doc = "Acknowledge failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<AFW_AW> for bool {
    #[inline(always)]
    fn from(variant: AFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AF` writer - Acknowledge failure"]
pub type AF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR1_SPEC, AFW_AW, O>;
impl<'a, const O: u8> AF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AFW_AW::Clear)
    }
}
#[doc = "Field `OVR` reader - Overrun/Underrun"]
pub type OVR_R = crate::BitReader<OVRR_A>;
#[doc = "Overrun/Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR_A {
    #[doc = "0: No overrun/underrun occured"]
    NoOverrun = 0,
    #[doc = "1: Overrun/underrun occured"]
    Overrun = 1,
}
impl From<OVRR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRR_A {
        match self.bits {
            false => OVRR_A::NoOverrun,
            true => OVRR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR_A::Overrun
    }
}
#[doc = "Overrun/Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<OVRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` writer - Overrun/Underrun"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR1_SPEC, OVRW_AW, O>;
impl<'a, const O: u8> OVR_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRW_AW::Clear)
    }
}
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PECERR_R = crate::BitReader<PECERRR_A>;
#[doc = "PEC Error in reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERRR_A {
    #[doc = "0: no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    NoError = 0,
    #[doc = "1: PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    Error = 1,
}
impl From<PECERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECERRR_A {
        match self.bits {
            false => PECERRR_A::NoError,
            true => PECERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PECERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PECERRR_A::Error
    }
}
#[doc = "PEC Error in reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERRW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<PECERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PECERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` writer - PEC Error in reception"]
pub type PECERR_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR1_SPEC, PECERRW_AW, O>;
impl<'a, const O: u8> PECERR_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECERRW_AW::Clear)
    }
}
#[doc = "Field `TIMEOUT` reader - Timeout or Tlow error"]
pub type TIMEOUT_R = crate::BitReader<TIMEOUTR_A>;
#[doc = "Timeout or Tlow error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTR_A {
    #[doc = "0: No Timeout error"]
    NoTimeout = 0,
    #[doc = "1: SCL remained LOW for 25 ms"]
    Timeout = 1,
}
impl From<TIMEOUTR_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTR_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUTR_A {
        match self.bits {
            false => TIMEOUTR_A::NoTimeout,
            true => TIMEOUTR_A::Timeout,
        }
    }
    #[doc = "Checks if the value of the field is `NoTimeout`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUTR_A::NoTimeout
    }
    #[doc = "Checks if the value of the field is `Timeout`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUTR_A::Timeout
    }
}
#[doc = "Timeout or Tlow error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<TIMEOUTW_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` writer - Timeout or Tlow error"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR1_SPEC, TIMEOUTW_AW, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIMEOUTW_AW::Clear)
    }
}
#[doc = "Field `SMBALERT` reader - SMBus alert"]
pub type SMBALERT_R = crate::BitReader<SMBALERTR_A>;
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBALERTR_A {
    #[doc = "0: No SMBALERT occured"]
    NoAlert = 0,
    #[doc = "1: SMBALERT occurred"]
    Alert = 1,
}
impl From<SMBALERTR_A> for bool {
    #[inline(always)]
    fn from(variant: SMBALERTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBALERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBALERTR_A {
        match self.bits {
            false => SMBALERTR_A::NoAlert,
            true => SMBALERTR_A::Alert,
        }
    }
    #[doc = "Checks if the value of the field is `NoAlert`"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == SMBALERTR_A::NoAlert
    }
    #[doc = "Checks if the value of the field is `Alert`"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == SMBALERTR_A::Alert
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBALERTW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<SMBALERTW_AW> for bool {
    #[inline(always)]
    fn from(variant: SMBALERTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBALERT` writer - SMBus alert"]
pub type SMBALERT_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR1_SPEC, SMBALERTW_AW, O>;
impl<'a, const O: u8> SMBALERT_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SMBALERTW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 10-bit header sent (Master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RX_NE_R {
        RX_NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline(always)]
    pub fn tx_e(&self) -> TX_E_R {
        TX_E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<8> {
        BERR_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn arlo(&mut self) -> ARLO_W<9> {
        ARLO_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    #[must_use]
    pub fn af(&mut self) -> AF_W<10> {
        AF_W::new(self)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<11> {
        OVR_W::new(self)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PECERR_W<12> {
        PECERR_W::new(self)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<14> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn smbalert(&mut self) -> SMBALERT_W<15> {
        SMBALERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr1::W](W) writer structure"]
impl crate::Writable for SR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xdf00;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
