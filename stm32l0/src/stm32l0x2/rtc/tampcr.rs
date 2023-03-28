#[doc = "Register `TAMPCR` reader"]
pub struct R(crate::R<TAMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMPCR` writer"]
pub struct W(crate::W<TAMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMPCR_SPEC>;
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
impl From<crate::W<TAMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1E` reader - RTC_TAMP1 input detection enable"]
pub type TAMP1E_R = crate::BitReader<TAMP1E_A>;
#[doc = "RTC_TAMP1 input detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1E_A {
    #[doc = "0: RTC_TAMPx input detection disabled"]
    Disabled = 0,
    #[doc = "1: RTC_TAMPx input detection enabled"]
    Enabled = 1,
}
impl From<TAMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1E_A {
        match self.bits {
            false => TAMP1E_A::Disabled,
            true => TAMP1E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1E_A::Enabled
    }
}
#[doc = "Field `TAMP1E` writer - RTC_TAMP1 input detection enable"]
pub type TAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, TAMP1E_A, O>;
impl<'a, const O: u8> TAMP1E_W<'a, O> {
    #[doc = "RTC_TAMPx input detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP1E_A::Disabled)
    }
    #[doc = "RTC_TAMPx input detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP1E_A::Enabled)
    }
}
#[doc = "Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input"]
pub type TAMP1TRG_R = crate::BitReader<TAMP1TRG_A>;
#[doc = "Active level for RTC_TAMP1 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1TRG_A {
    #[doc = "0: If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    RisingEdge = 0,
    #[doc = "1: If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    FallingEdge = 1,
}
impl From<TAMP1TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1TRG_A {
        match self.bits {
            false => TAMP1TRG_A::RisingEdge,
            true => TAMP1TRG_A::FallingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TAMP1TRG_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TAMP1TRG_A::FallingEdge
    }
}
#[doc = "Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input"]
pub type TAMP1TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, TAMP1TRG_A, O>;
impl<'a, const O: u8> TAMP1TRG_W<'a, O> {
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::RisingEdge)
    }
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::FallingEdge)
    }
}
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TAMPIE_R = crate::BitReader<TAMPIE_A>;
#[doc = "Tamper interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPIE_A {
    #[doc = "0: Tamper interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Tamper interrupt enabled"]
    Enabled = 1,
}
impl From<TAMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPIE_A {
        match self.bits {
            false => TAMPIE_A::Disabled,
            true => TAMPIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPIE_A::Enabled
    }
}
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, TAMPIE_A, O>;
impl<'a, const O: u8> TAMPIE_W<'a, O> {
    #[doc = "Tamper interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPIE_A::Disabled)
    }
    #[doc = "Tamper interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPIE_A::Enabled)
    }
}
#[doc = "Field `TAMP2E` reader - RTC_TAMP2 input detection enable"]
pub use TAMP1E_R as TAMP2E_R;
#[doc = "Field `TAMP3E` reader - RTC_TAMP3 detection enable"]
pub use TAMP1E_R as TAMP3E_R;
#[doc = "Field `TAMP2E` writer - RTC_TAMP2 input detection enable"]
pub use TAMP1E_W as TAMP2E_W;
#[doc = "Field `TAMP3E` writer - RTC_TAMP3 detection enable"]
pub use TAMP1E_W as TAMP3E_W;
#[doc = "Field `TAMP2TRG` reader - Active level for RTC_TAMP2 input"]
pub use TAMP1TRG_R as TAMP2TRG_R;
#[doc = "Field `TAMP3TRG` reader - Active level for RTC_TAMP3 input"]
pub use TAMP1TRG_R as TAMP3TRG_R;
#[doc = "Field `TAMP2TRG` writer - Active level for RTC_TAMP2 input"]
pub use TAMP1TRG_W as TAMP2TRG_W;
#[doc = "Field `TAMP3TRG` writer - Active level for RTC_TAMP3 input"]
pub use TAMP1TRG_W as TAMP3TRG_W;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub type TAMPTS_R = crate::BitReader<TAMPTS_A>;
#[doc = "Activate timestamp on tamper detection event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPTS_A {
    #[doc = "0: Tamper detection event does not cause a timestamp to be saved"]
    NoSave = 0,
    #[doc = "1: Save timestamp on tamper detection event"]
    Save = 1,
}
impl From<TAMPTS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMPTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPTS_A {
        match self.bits {
            false => TAMPTS_A::NoSave,
            true => TAMPTS_A::Save,
        }
    }
    #[doc = "Checks if the value of the field is `NoSave`"]
    #[inline(always)]
    pub fn is_no_save(&self) -> bool {
        *self == TAMPTS_A::NoSave
    }
    #[doc = "Checks if the value of the field is `Save`"]
    #[inline(always)]
    pub fn is_save(&self) -> bool {
        *self == TAMPTS_A::Save
    }
}
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub type TAMPTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, TAMPTS_A, O>;
impl<'a, const O: u8> TAMPTS_W<'a, O> {
    #[doc = "Tamper detection event does not cause a timestamp to be saved"]
    #[inline(always)]
    pub fn no_save(self) -> &'a mut W {
        self.variant(TAMPTS_A::NoSave)
    }
    #[doc = "Save timestamp on tamper detection event"]
    #[inline(always)]
    pub fn save(self) -> &'a mut W {
        self.variant(TAMPTS_A::Save)
    }
}
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub type TAMPFREQ_R = crate::FieldReader<u8, TAMPFREQ_A>;
#[doc = "Tamper sampling frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFREQ_A {
    #[doc = "0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    Div32768 = 0,
    #[doc = "1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    Div16384 = 1,
    #[doc = "2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    Div8192 = 2,
    #[doc = "3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    Div4096 = 3,
    #[doc = "4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    Div2048 = 4,
    #[doc = "5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    Div1024 = 5,
    #[doc = "6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    Div512 = 6,
    #[doc = "7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    Div256 = 7,
}
impl From<TAMPFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ_A) -> Self {
        variant as _
    }
}
impl TAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPFREQ_A {
        match self.bits {
            0 => TAMPFREQ_A::Div32768,
            1 => TAMPFREQ_A::Div16384,
            2 => TAMPFREQ_A::Div8192,
            3 => TAMPFREQ_A::Div4096,
            4 => TAMPFREQ_A::Div2048,
            5 => TAMPFREQ_A::Div1024,
            6 => TAMPFREQ_A::Div512,
            7 => TAMPFREQ_A::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == TAMPFREQ_A::Div32768
    }
    #[doc = "Checks if the value of the field is `Div16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == TAMPFREQ_A::Div16384
    }
    #[doc = "Checks if the value of the field is `Div8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == TAMPFREQ_A::Div8192
    }
    #[doc = "Checks if the value of the field is `Div4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == TAMPFREQ_A::Div4096
    }
    #[doc = "Checks if the value of the field is `Div2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == TAMPFREQ_A::Div2048
    }
    #[doc = "Checks if the value of the field is `Div1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == TAMPFREQ_A::Div1024
    }
    #[doc = "Checks if the value of the field is `Div512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == TAMPFREQ_A::Div512
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == TAMPFREQ_A::Div256
    }
}
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub type TAMPFREQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAMPCR_SPEC, u8, TAMPFREQ_A, 3, O>;
impl<'a, const O: u8> TAMPFREQ_W<'a, O> {
    #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div32768)
    }
    #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div16384)
    }
    #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div8192)
    }
    #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div4096)
    }
    #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div2048)
    }
    #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div1024)
    }
    #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div512)
    }
    #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div256)
    }
}
#[doc = "Field `TAMPFLT` reader - RTC_TAMPx filter count"]
pub type TAMPFLT_R = crate::FieldReader<u8, TAMPFLT_A>;
#[doc = "RTC_TAMPx filter count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFLT_A {
    #[doc = "0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
    Immediate = 0,
    #[doc = "1: Tamper event is activated after 2 consecutive samples at the active level"]
    Samples2 = 1,
    #[doc = "2: Tamper event is activated after 4 consecutive samples at the active level"]
    Samples4 = 2,
    #[doc = "3: Tamper event is activated after 8 consecutive samples at the active level"]
    Samples8 = 3,
}
impl From<TAMPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT_A) -> Self {
        variant as _
    }
}
impl TAMPFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPFLT_A {
        match self.bits {
            0 => TAMPFLT_A::Immediate,
            1 => TAMPFLT_A::Samples2,
            2 => TAMPFLT_A::Samples4,
            3 => TAMPFLT_A::Samples8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Immediate`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TAMPFLT_A::Immediate
    }
    #[doc = "Checks if the value of the field is `Samples2`"]
    #[inline(always)]
    pub fn is_samples2(&self) -> bool {
        *self == TAMPFLT_A::Samples2
    }
    #[doc = "Checks if the value of the field is `Samples4`"]
    #[inline(always)]
    pub fn is_samples4(&self) -> bool {
        *self == TAMPFLT_A::Samples4
    }
    #[doc = "Checks if the value of the field is `Samples8`"]
    #[inline(always)]
    pub fn is_samples8(&self) -> bool {
        *self == TAMPFLT_A::Samples8
    }
}
#[doc = "Field `TAMPFLT` writer - RTC_TAMPx filter count"]
pub type TAMPFLT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAMPCR_SPEC, u8, TAMPFLT_A, 2, O>;
impl<'a, const O: u8> TAMPFLT_W<'a, O> {
    #[doc = "Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Immediate)
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples2(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Samples2)
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples4(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Samples4)
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples8(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Samples8)
    }
}
#[doc = "Field `TAMPPRCH` reader - RTC_TAMPx precharge duration"]
pub type TAMPPRCH_R = crate::FieldReader<u8, TAMPPRCH_A>;
#[doc = "RTC_TAMPx precharge duration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPPRCH_A {
    #[doc = "0: 1 RTCCLK cycle"]
    Cycles1 = 0,
    #[doc = "1: 2 RTCCLK cycles"]
    Cycles2 = 1,
    #[doc = "2: 4 RTCCLK cycles"]
    Cycles4 = 2,
    #[doc = "3: 8 RTCCLK cycles"]
    Cycles8 = 3,
}
impl From<TAMPPRCH_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH_A) -> Self {
        variant as _
    }
}
impl TAMPPRCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPPRCH_A {
        match self.bits {
            0 => TAMPPRCH_A::Cycles1,
            1 => TAMPPRCH_A::Cycles2,
            2 => TAMPPRCH_A::Cycles4,
            3 => TAMPPRCH_A::Cycles8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Cycles1`"]
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        *self == TAMPPRCH_A::Cycles1
    }
    #[doc = "Checks if the value of the field is `Cycles2`"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TAMPPRCH_A::Cycles2
    }
    #[doc = "Checks if the value of the field is `Cycles4`"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == TAMPPRCH_A::Cycles4
    }
    #[doc = "Checks if the value of the field is `Cycles8`"]
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == TAMPPRCH_A::Cycles8
    }
}
#[doc = "Field `TAMPPRCH` writer - RTC_TAMPx precharge duration"]
pub type TAMPPRCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAMPCR_SPEC, u8, TAMPPRCH_A, 2, O>;
impl<'a, const O: u8> TAMPPRCH_W<'a, O> {
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles1)
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles2)
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles4)
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles8)
    }
}
#[doc = "Field `TAMPPUDIS` reader - RTC_TAMPx pull-up disable"]
pub type TAMPPUDIS_R = crate::BitReader<TAMPPUDIS_A>;
#[doc = "RTC_TAMPx pull-up disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPPUDIS_A {
    #[doc = "0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
    Enabled = 0,
    #[doc = "1: Disable precharge of RTC_TAMPx pins"]
    Disabled = 1,
}
impl From<TAMPPUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMPPUDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPPUDIS_A {
        match self.bits {
            false => TAMPPUDIS_A::Enabled,
            true => TAMPPUDIS_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPPUDIS_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPPUDIS_A::Disabled
    }
}
#[doc = "Field `TAMPPUDIS` writer - RTC_TAMPx pull-up disable"]
pub type TAMPPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, TAMPPUDIS_A, O>;
impl<'a, const O: u8> TAMPPUDIS_W<'a, O> {
    #[doc = "Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::Enabled)
    }
    #[doc = "Disable precharge of RTC_TAMPx pins"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::Disabled)
    }
}
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub type TAMP1IE_R = crate::BitReader<TAMP1IE_A>;
#[doc = "Tamper 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1IE_A {
    #[doc = "0: Tamper x interrupt is disabled if TAMPIE = 0"]
    Disabled = 0,
    #[doc = "1: Tamper x interrupt enabled"]
    Enabled = 1,
}
impl From<TAMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1IE_A {
        match self.bits {
            false => TAMP1IE_A::Disabled,
            true => TAMP1IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1IE_A::Enabled
    }
}
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub type TAMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, TAMP1IE_A, O>;
impl<'a, const O: u8> TAMP1IE_W<'a, O> {
    #[doc = "Tamper x interrupt is disabled if TAMPIE = 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP1IE_A::Disabled)
    }
    #[doc = "Tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP1IE_A::Enabled)
    }
}
#[doc = "Field `TAMP1NOERASE` reader - Tamper 1 no erase"]
pub type TAMP1NOERASE_R = crate::BitReader<TAMP1NOERASE_A>;
#[doc = "Tamper 1 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1NOERASE_A {
    #[doc = "0: Tamper x event erases the backup registers"]
    Erase = 0,
    #[doc = "1: Tamper x event does not erase the backup registers"]
    NoErase = 1,
}
impl From<TAMP1NOERASE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1NOERASE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1NOERASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1NOERASE_A {
        match self.bits {
            false => TAMP1NOERASE_A::Erase,
            true => TAMP1NOERASE_A::NoErase,
        }
    }
    #[doc = "Checks if the value of the field is `Erase`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == TAMP1NOERASE_A::Erase
    }
    #[doc = "Checks if the value of the field is `NoErase`"]
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == TAMP1NOERASE_A::NoErase
    }
}
#[doc = "Field `TAMP1NOERASE` writer - Tamper 1 no erase"]
pub type TAMP1NOERASE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TAMPCR_SPEC, TAMP1NOERASE_A, O>;
impl<'a, const O: u8> TAMP1NOERASE_W<'a, O> {
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(TAMP1NOERASE_A::Erase)
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut W {
        self.variant(TAMP1NOERASE_A::NoErase)
    }
}
#[doc = "Field `TAMP1MF` reader - Tamper 1 mask flag"]
pub type TAMP1MF_R = crate::BitReader<TAMP1MF_A>;
#[doc = "Tamper 1 mask flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1MF_A {
    #[doc = "0: Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    NotMasked = 0,
    #[doc = "1: Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased."]
    Masked = 1,
}
impl From<TAMP1MF_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MF_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1MF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1MF_A {
        match self.bits {
            false => TAMP1MF_A::NotMasked,
            true => TAMP1MF_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `NotMasked`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TAMP1MF_A::NotMasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TAMP1MF_A::Masked
    }
}
#[doc = "Field `TAMP1MF` writer - Tamper 1 mask flag"]
pub type TAMP1MF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, TAMP1MF_A, O>;
impl<'a, const O: u8> TAMP1MF_W<'a, O> {
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TAMP1MF_A::NotMasked)
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TAMP1MF_A::Masked)
    }
}
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub use TAMP1IE_R as TAMP2IE_R;
#[doc = "Field `TAMP3IE` reader - Tamper 3 interrupt enable"]
pub use TAMP1IE_R as TAMP3IE_R;
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub use TAMP1IE_W as TAMP2IE_W;
#[doc = "Field `TAMP3IE` writer - Tamper 3 interrupt enable"]
pub use TAMP1IE_W as TAMP3IE_W;
#[doc = "Field `TAMP2MF` reader - Tamper 2 mask flag"]
pub use TAMP1MF_R as TAMP2MF_R;
#[doc = "Field `TAMP3MF` reader - Tamper 3 mask flag"]
pub use TAMP1MF_R as TAMP3MF_R;
#[doc = "Field `TAMP2MF` writer - Tamper 2 mask flag"]
pub use TAMP1MF_W as TAMP2MF_W;
#[doc = "Field `TAMP3MF` writer - Tamper 3 mask flag"]
pub use TAMP1MF_W as TAMP3MF_W;
#[doc = "Field `TAMP2NOERASE` reader - Tamper 2 no erase"]
pub use TAMP1NOERASE_R as TAMP2NOERASE_R;
#[doc = "Field `TAMP3NOERASE` reader - Tamper 3 no erase"]
pub use TAMP1NOERASE_R as TAMP3NOERASE_R;
#[doc = "Field `TAMP2NOERASE` writer - Tamper 2 no erase"]
pub use TAMP1NOERASE_W as TAMP2NOERASE_W;
#[doc = "Field `TAMP3NOERASE` writer - Tamper 3 no erase"]
pub use TAMP1NOERASE_W as TAMP3NOERASE_W;
impl R {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&self) -> TAMP1NOERASE_R {
        TAMP1NOERASE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&self) -> TAMP2NOERASE_R {
        TAMP2NOERASE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&self) -> TAMP3NOERASE_R {
        TAMP3NOERASE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> TAMP1E_W<0> {
        TAMP1E_W::new(self)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<1> {
        TAMP1TRG_W::new(self)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampie(&mut self) -> TAMPIE_W<2> {
        TAMPIE_W::new(self)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2e(&mut self) -> TAMP2E_W<3> {
        TAMP2E_W::new(self)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<4> {
        TAMP2TRG_W::new(self)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3e(&mut self) -> TAMP3E_W<5> {
        TAMP3E_W::new(self)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<6> {
        TAMP3TRG_W::new(self)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<7> {
        TAMPTS_W::new(self)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<8> {
        TAMPFREQ_W::new(self)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<11> {
        TAMPFLT_W::new(self)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<13> {
        TAMPPRCH_W::new(self)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<15> {
        TAMPPUDIS_W::new(self)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<16> {
        TAMP1IE_W::new(self)
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1noerase(&mut self) -> TAMP1NOERASE_W<17> {
        TAMP1NOERASE_W::new(self)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1mf(&mut self) -> TAMP1MF_W<18> {
        TAMP1MF_W::new(self)
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<19> {
        TAMP2IE_W::new(self)
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2noerase(&mut self) -> TAMP2NOERASE_W<20> {
        TAMP2NOERASE_W::new(self)
    }
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2mf(&mut self) -> TAMP2MF_W<21> {
        TAMP2MF_W::new(self)
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<22> {
        TAMP3IE_W::new(self)
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3noerase(&mut self) -> TAMP3NOERASE_W<23> {
        TAMP3NOERASE_W::new(self)
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3mf(&mut self) -> TAMP3MF_W<24> {
        TAMP3MF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC tamper configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampcr](index.html) module"]
pub struct TAMPCR_SPEC;
impl crate::RegisterSpec for TAMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tampcr::R](R) reader structure"]
impl crate::Readable for TAMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tampcr::W](W) writer structure"]
impl crate::Writable for TAMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMPCR to value 0"]
impl crate::Resettable for TAMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
