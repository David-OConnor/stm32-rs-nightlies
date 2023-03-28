#[doc = "Register `ISTR` reader"]
pub struct R(crate::R<ISTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISTR` writer"]
pub struct W(crate::W<ISTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTR_SPEC>;
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
impl From<crate::W<ISTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_ID` reader - Endpoint Identifier"]
pub type EP_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIR` reader - Direction of transaction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Direction of transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: data transmitted by the USB peripheral to the host PC"]
    To = 0,
    #[doc = "1: data received by the USB peripheral from the host PC"]
    From = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::To,
            true => DIR_A::From,
        }
    }
    #[doc = "Checks if the value of the field is `To`"]
    #[inline(always)]
    pub fn is_to(&self) -> bool {
        *self == DIR_A::To
    }
    #[doc = "Checks if the value of the field is `From`"]
    #[inline(always)]
    pub fn is_from(&self) -> bool {
        *self == DIR_A::From
    }
}
#[doc = "Field `L1REQ` reader - LPM L1 state request"]
pub type L1REQ_R = crate::BitReader<L1REQR_A>;
#[doc = "LPM L1 state request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQR_A {
    #[doc = "0: LPM command to enter the L1 state is not received"]
    NotReceived = 0,
    #[doc = "1: LPM command to enter the L1 state is successfully received and acknowledged"]
    Received = 1,
}
impl From<L1REQR_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQR_A) -> Self {
        variant as u8 != 0
    }
}
impl L1REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1REQR_A {
        match self.bits {
            false => L1REQR_A::NotReceived,
            true => L1REQR_A::Received,
        }
    }
    #[doc = "Checks if the value of the field is `NotReceived`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == L1REQR_A::NotReceived
    }
    #[doc = "Checks if the value of the field is `Received`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == L1REQR_A::Received
    }
}
#[doc = "LPM L1 state request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<L1REQW_AW> for bool {
    #[inline(always)]
    fn from(variant: L1REQW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1REQ` writer - LPM L1 state request"]
pub type L1REQ_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISTR_SPEC, L1REQW_AW, O>;
impl<'a, const O: u8> L1REQ_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(L1REQW_AW::Clear)
    }
}
#[doc = "Field `ESOF` reader - Expected start frame"]
pub type ESOF_R = crate::BitReader<ESOFR_A>;
#[doc = "Expected start frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFR_A {
    #[doc = "0: NotExpectedStartOfFrame"]
    NotExpectedStartOfFrame = 0,
    #[doc = "1: an SOF packet is expected but not received"]
    ExpectedStartOfFrame = 1,
}
impl From<ESOFR_A> for bool {
    #[inline(always)]
    fn from(variant: ESOFR_A) -> Self {
        variant as u8 != 0
    }
}
impl ESOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESOFR_A {
        match self.bits {
            false => ESOFR_A::NotExpectedStartOfFrame,
            true => ESOFR_A::ExpectedStartOfFrame,
        }
    }
    #[doc = "Checks if the value of the field is `NotExpectedStartOfFrame`"]
    #[inline(always)]
    pub fn is_not_expected_start_of_frame(&self) -> bool {
        *self == ESOFR_A::NotExpectedStartOfFrame
    }
    #[doc = "Checks if the value of the field is `ExpectedStartOfFrame`"]
    #[inline(always)]
    pub fn is_expected_start_of_frame(&self) -> bool {
        *self == ESOFR_A::ExpectedStartOfFrame
    }
}
#[doc = "Expected start frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<ESOFW_AW> for bool {
    #[inline(always)]
    fn from(variant: ESOFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOF` writer - Expected start frame"]
pub type ESOF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISTR_SPEC, ESOFW_AW, O>;
impl<'a, const O: u8> ESOF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ESOFW_AW::Clear)
    }
}
#[doc = "Field `SOF` reader - start of frame"]
pub type SOF_R = crate::BitReader<SOFR_A>;
#[doc = "start of frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFR_A {
    #[doc = "0: NotStartOfFrame"]
    NotStartOfFrame = 0,
    #[doc = "1: beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    StartOfFrame = 1,
}
impl From<SOFR_A> for bool {
    #[inline(always)]
    fn from(variant: SOFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFR_A {
        match self.bits {
            false => SOFR_A::NotStartOfFrame,
            true => SOFR_A::StartOfFrame,
        }
    }
    #[doc = "Checks if the value of the field is `NotStartOfFrame`"]
    #[inline(always)]
    pub fn is_not_start_of_frame(&self) -> bool {
        *self == SOFR_A::NotStartOfFrame
    }
    #[doc = "Checks if the value of the field is `StartOfFrame`"]
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == SOFR_A::StartOfFrame
    }
}
#[doc = "start of frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<SOFW_AW> for bool {
    #[inline(always)]
    fn from(variant: SOFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` writer - start of frame"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISTR_SPEC, SOFW_AW, O>;
impl<'a, const O: u8> SOF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SOFW_AW::Clear)
    }
}
#[doc = "Field `RESET` reader - reset request"]
pub type RESET_R = crate::BitReader<RESETR_A>;
#[doc = "reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETR_A {
    #[doc = "0: NotReset"]
    NotReset = 0,
    #[doc = "1: peripheral detects an active USB RESET signal at its inputs"]
    Reset = 1,
}
impl From<RESETR_A> for bool {
    #[inline(always)]
    fn from(variant: RESETR_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETR_A {
        match self.bits {
            false => RESETR_A::NotReset,
            true => RESETR_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NotReset`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == RESETR_A::NotReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETR_A::Reset
    }
}
#[doc = "reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<RESETW_AW> for bool {
    #[inline(always)]
    fn from(variant: RESETW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - reset request"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISTR_SPEC, RESETW_AW, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RESETW_AW::Clear)
    }
}
#[doc = "Field `SUSP` reader - Suspend mode request"]
pub type SUSP_R = crate::BitReader<SUSPR_A>;
#[doc = "Suspend mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPR_A {
    #[doc = "0: NotSuspend"]
    NotSuspend = 0,
    #[doc = "1: no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    Suspend = 1,
}
impl From<SUSPR_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPR_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPR_A {
        match self.bits {
            false => SUSPR_A::NotSuspend,
            true => SUSPR_A::Suspend,
        }
    }
    #[doc = "Checks if the value of the field is `NotSuspend`"]
    #[inline(always)]
    pub fn is_not_suspend(&self) -> bool {
        *self == SUSPR_A::NotSuspend
    }
    #[doc = "Checks if the value of the field is `Suspend`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SUSPR_A::Suspend
    }
}
#[doc = "Suspend mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<SUSPW_AW> for bool {
    #[inline(always)]
    fn from(variant: SUSPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` writer - Suspend mode request"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISTR_SPEC, SUSPW_AW, O>;
impl<'a, const O: u8> SUSP_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SUSPW_AW::Clear)
    }
}
#[doc = "Field `WKUP` reader - Wakeup"]
pub type WKUP_R = crate::BitReader<WKUPR_A>;
#[doc = "Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPR_A {
    #[doc = "0: NotWakeup"]
    NotWakeup = 0,
    #[doc = "1: activity is detected that wakes up the USB peripheral"]
    Wakeup = 1,
}
impl From<WKUPR_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPR_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPR_A {
        match self.bits {
            false => WKUPR_A::NotWakeup,
            true => WKUPR_A::Wakeup,
        }
    }
    #[doc = "Checks if the value of the field is `NotWakeup`"]
    #[inline(always)]
    pub fn is_not_wakeup(&self) -> bool {
        *self == WKUPR_A::NotWakeup
    }
    #[doc = "Checks if the value of the field is `Wakeup`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WKUPR_A::Wakeup
    }
}
#[doc = "Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<WKUPW_AW> for bool {
    #[inline(always)]
    fn from(variant: WKUPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUP` writer - Wakeup"]
pub type WKUP_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISTR_SPEC, WKUPW_AW, O>;
impl<'a, const O: u8> WKUP_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WKUPW_AW::Clear)
    }
}
#[doc = "Field `ERR` reader - Error"]
pub type ERR_R = crate::BitReader<ERRR_A>;
#[doc = "Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRR_A {
    #[doc = "0: Errors are not occurred"]
    NotOverrun = 0,
    #[doc = "1: One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    Error = 1,
}
impl From<ERRR_A> for bool {
    #[inline(always)]
    fn from(variant: ERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRR_A {
        match self.bits {
            false => ERRR_A::NotOverrun,
            true => ERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NotOverrun`"]
    #[inline(always)]
    pub fn is_not_overrun(&self) -> bool {
        *self == ERRR_A::NotOverrun
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERRR_A::Error
    }
}
#[doc = "Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<ERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` writer - Error"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISTR_SPEC, ERRW_AW, O>;
impl<'a, const O: u8> ERR_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRW_AW::Clear)
    }
}
#[doc = "Field `PMAOVR` reader - Packet memory area over / underrun"]
pub type PMAOVR_R = crate::BitReader<PMAOVRR_A>;
#[doc = "Packet memory area over / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRR_A {
    #[doc = "0: Overrun is not occurred"]
    NotOverrun = 0,
    #[doc = "1: microcontroller has not been able to respond in time to an USB memory request"]
    Overrun = 1,
}
impl From<PMAOVRR_A> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PMAOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMAOVRR_A {
        match self.bits {
            false => PMAOVRR_A::NotOverrun,
            true => PMAOVRR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NotOverrun`"]
    #[inline(always)]
    pub fn is_not_overrun(&self) -> bool {
        *self == PMAOVRR_A::NotOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == PMAOVRR_A::Overrun
    }
}
#[doc = "Packet memory area over / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<PMAOVRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVR` writer - Packet memory area over / underrun"]
pub type PMAOVR_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, ISTR_SPEC, PMAOVRW_AW, O>;
impl<'a, const O: u8> PMAOVR_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PMAOVRW_AW::Clear)
    }
}
#[doc = "Field `CTR` reader - Correct transfer"]
pub type CTR_R = crate::BitReader<CTR_A>;
#[doc = "Correct transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTR_A {
    #[doc = "1: endpoint has successfully completed a transaction"]
    Completed = 1,
}
impl From<CTR_A> for bool {
    #[inline(always)]
    fn from(variant: CTR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTR_A> {
        match self.bits {
            true => Some(CTR_A::Completed),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Completed`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CTR_A::Completed
    }
}
impl R {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - LPM L1 state request"]
    #[inline(always)]
    #[must_use]
    pub fn l1req(&mut self) -> L1REQ_W<7> {
        L1REQ_W::new(self)
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    #[must_use]
    pub fn esof(&mut self) -> ESOF_W<8> {
        ESOF_W::new(self)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<9> {
        SOF_W::new(self)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<10> {
        RESET_W::new(self)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<11> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<12> {
        WKUP_W::new(self)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<13> {
        ERR_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovr(&mut self) -> PMAOVR_W<14> {
        PMAOVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istr](index.html) module"]
pub struct ISTR_SPEC;
impl crate::RegisterSpec for ISTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [istr::R](R) reader structure"]
impl crate::Readable for ISTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [istr::W](W) writer structure"]
impl crate::Writable for ISTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x7f80;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for ISTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
