#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIZE` reader - Number of bits in at single SPI data frame"]
pub type DSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSIZE` writer - Number of bits in at single SPI data frame"]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, u8, 5, O>;
#[doc = "Field `FTHLV` reader - threshold level"]
pub type FTHLV_R = crate::FieldReader<u8, FTHLV_A>;
#[doc = "threshold level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTHLV_A {
    #[doc = "0: 1 frame"]
    OneFrame = 0,
    #[doc = "1: 2 frames"]
    TwoFrames = 1,
    #[doc = "2: 3 frames"]
    ThreeFrames = 2,
    #[doc = "3: 4 frames"]
    FourFrames = 3,
    #[doc = "4: 5 frames"]
    FiveFrames = 4,
    #[doc = "5: 6 frames"]
    SixFrames = 5,
    #[doc = "6: 7 frames"]
    SevenFrames = 6,
    #[doc = "7: 8 frames"]
    EightFrames = 7,
    #[doc = "8: 9 frames"]
    NineFrames = 8,
    #[doc = "9: 10 frames"]
    TenFrames = 9,
    #[doc = "10: 11 frames"]
    ElevenFrames = 10,
    #[doc = "11: 12 frames"]
    TwelveFrames = 11,
    #[doc = "12: 13 frames"]
    ThirteenFrames = 12,
    #[doc = "13: 14 frames"]
    FourteenFrames = 13,
    #[doc = "14: 15 frames"]
    FifteenFrames = 14,
    #[doc = "15: 16 frames"]
    SixteenFrames = 15,
}
impl From<FTHLV_A> for u8 {
    #[inline(always)]
    fn from(variant: FTHLV_A) -> Self {
        variant as _
    }
}
impl FTHLV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTHLV_A {
        match self.bits {
            0 => FTHLV_A::OneFrame,
            1 => FTHLV_A::TwoFrames,
            2 => FTHLV_A::ThreeFrames,
            3 => FTHLV_A::FourFrames,
            4 => FTHLV_A::FiveFrames,
            5 => FTHLV_A::SixFrames,
            6 => FTHLV_A::SevenFrames,
            7 => FTHLV_A::EightFrames,
            8 => FTHLV_A::NineFrames,
            9 => FTHLV_A::TenFrames,
            10 => FTHLV_A::ElevenFrames,
            11 => FTHLV_A::TwelveFrames,
            12 => FTHLV_A::ThirteenFrames,
            13 => FTHLV_A::FourteenFrames,
            14 => FTHLV_A::FifteenFrames,
            15 => FTHLV_A::SixteenFrames,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OneFrame`"]
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == FTHLV_A::OneFrame
    }
    #[doc = "Checks if the value of the field is `TwoFrames`"]
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == FTHLV_A::TwoFrames
    }
    #[doc = "Checks if the value of the field is `ThreeFrames`"]
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == FTHLV_A::ThreeFrames
    }
    #[doc = "Checks if the value of the field is `FourFrames`"]
    #[inline(always)]
    pub fn is_four_frames(&self) -> bool {
        *self == FTHLV_A::FourFrames
    }
    #[doc = "Checks if the value of the field is `FiveFrames`"]
    #[inline(always)]
    pub fn is_five_frames(&self) -> bool {
        *self == FTHLV_A::FiveFrames
    }
    #[doc = "Checks if the value of the field is `SixFrames`"]
    #[inline(always)]
    pub fn is_six_frames(&self) -> bool {
        *self == FTHLV_A::SixFrames
    }
    #[doc = "Checks if the value of the field is `SevenFrames`"]
    #[inline(always)]
    pub fn is_seven_frames(&self) -> bool {
        *self == FTHLV_A::SevenFrames
    }
    #[doc = "Checks if the value of the field is `EightFrames`"]
    #[inline(always)]
    pub fn is_eight_frames(&self) -> bool {
        *self == FTHLV_A::EightFrames
    }
    #[doc = "Checks if the value of the field is `NineFrames`"]
    #[inline(always)]
    pub fn is_nine_frames(&self) -> bool {
        *self == FTHLV_A::NineFrames
    }
    #[doc = "Checks if the value of the field is `TenFrames`"]
    #[inline(always)]
    pub fn is_ten_frames(&self) -> bool {
        *self == FTHLV_A::TenFrames
    }
    #[doc = "Checks if the value of the field is `ElevenFrames`"]
    #[inline(always)]
    pub fn is_eleven_frames(&self) -> bool {
        *self == FTHLV_A::ElevenFrames
    }
    #[doc = "Checks if the value of the field is `TwelveFrames`"]
    #[inline(always)]
    pub fn is_twelve_frames(&self) -> bool {
        *self == FTHLV_A::TwelveFrames
    }
    #[doc = "Checks if the value of the field is `ThirteenFrames`"]
    #[inline(always)]
    pub fn is_thirteen_frames(&self) -> bool {
        *self == FTHLV_A::ThirteenFrames
    }
    #[doc = "Checks if the value of the field is `FourteenFrames`"]
    #[inline(always)]
    pub fn is_fourteen_frames(&self) -> bool {
        *self == FTHLV_A::FourteenFrames
    }
    #[doc = "Checks if the value of the field is `FifteenFrames`"]
    #[inline(always)]
    pub fn is_fifteen_frames(&self) -> bool {
        *self == FTHLV_A::FifteenFrames
    }
    #[doc = "Checks if the value of the field is `SixteenFrames`"]
    #[inline(always)]
    pub fn is_sixteen_frames(&self) -> bool {
        *self == FTHLV_A::SixteenFrames
    }
}
#[doc = "Field `FTHLV` writer - threshold level"]
pub type FTHLV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, FTHLV_A, 4, O>;
impl<'a, const O: u8> FTHLV_W<'a, O> {
    #[doc = "1 frame"]
    #[inline(always)]
    pub fn one_frame(self) -> &'a mut W {
        self.variant(FTHLV_A::OneFrame)
    }
    #[doc = "2 frames"]
    #[inline(always)]
    pub fn two_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TwoFrames)
    }
    #[doc = "3 frames"]
    #[inline(always)]
    pub fn three_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::ThreeFrames)
    }
    #[doc = "4 frames"]
    #[inline(always)]
    pub fn four_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FourFrames)
    }
    #[doc = "5 frames"]
    #[inline(always)]
    pub fn five_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FiveFrames)
    }
    #[doc = "6 frames"]
    #[inline(always)]
    pub fn six_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SixFrames)
    }
    #[doc = "7 frames"]
    #[inline(always)]
    pub fn seven_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SevenFrames)
    }
    #[doc = "8 frames"]
    #[inline(always)]
    pub fn eight_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::EightFrames)
    }
    #[doc = "9 frames"]
    #[inline(always)]
    pub fn nine_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::NineFrames)
    }
    #[doc = "10 frames"]
    #[inline(always)]
    pub fn ten_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TenFrames)
    }
    #[doc = "11 frames"]
    #[inline(always)]
    pub fn eleven_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::ElevenFrames)
    }
    #[doc = "12 frames"]
    #[inline(always)]
    pub fn twelve_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TwelveFrames)
    }
    #[doc = "13 frames"]
    #[inline(always)]
    pub fn thirteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::ThirteenFrames)
    }
    #[doc = "14 frames"]
    #[inline(always)]
    pub fn fourteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FourteenFrames)
    }
    #[doc = "15 frames"]
    #[inline(always)]
    pub fn fifteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FifteenFrames)
    }
    #[doc = "16 frames"]
    #[inline(always)]
    pub fn sixteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SixteenFrames)
    }
}
#[doc = "Field `UDRCFG` reader - Behavior of slave transmitter at underrun condition"]
pub type UDRCFG_R = crate::FieldReader<u8, UDRCFG_A>;
#[doc = "Behavior of slave transmitter at underrun condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDRCFG_A {
    #[doc = "0: Slave sends a constant underrun pattern"]
    Constant = 0,
    #[doc = "1: Slave repeats last received data frame from master"]
    RepeatReceived = 1,
    #[doc = "2: Slave repeats last transmitted data frame"]
    RepeatTransmitted = 2,
}
impl From<UDRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: UDRCFG_A) -> Self {
        variant as _
    }
}
impl UDRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UDRCFG_A> {
        match self.bits {
            0 => Some(UDRCFG_A::Constant),
            1 => Some(UDRCFG_A::RepeatReceived),
            2 => Some(UDRCFG_A::RepeatTransmitted),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Constant`"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == UDRCFG_A::Constant
    }
    #[doc = "Checks if the value of the field is `RepeatReceived`"]
    #[inline(always)]
    pub fn is_repeat_received(&self) -> bool {
        *self == UDRCFG_A::RepeatReceived
    }
    #[doc = "Checks if the value of the field is `RepeatTransmitted`"]
    #[inline(always)]
    pub fn is_repeat_transmitted(&self) -> bool {
        *self == UDRCFG_A::RepeatTransmitted
    }
}
#[doc = "Field `UDRCFG` writer - Behavior of slave transmitter at underrun condition"]
pub type UDRCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, UDRCFG_A, 2, O>;
impl<'a, const O: u8> UDRCFG_W<'a, O> {
    #[doc = "Slave sends a constant underrun pattern"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(UDRCFG_A::Constant)
    }
    #[doc = "Slave repeats last received data frame from master"]
    #[inline(always)]
    pub fn repeat_received(self) -> &'a mut W {
        self.variant(UDRCFG_A::RepeatReceived)
    }
    #[doc = "Slave repeats last transmitted data frame"]
    #[inline(always)]
    pub fn repeat_transmitted(self) -> &'a mut W {
        self.variant(UDRCFG_A::RepeatTransmitted)
    }
}
#[doc = "Field `UDRDET` reader - Detection of underrun condition at slave transmitter"]
pub type UDRDET_R = crate::FieldReader<u8, UDRDET_A>;
#[doc = "Detection of underrun condition at slave transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDRDET_A {
    #[doc = "0: Underrun is detected at begin of data frame"]
    StartOfFrame = 0,
    #[doc = "1: Underrun is detected at end of last data frame"]
    EndOfFrame = 1,
    #[doc = "2: Underrun is detected at begin of active SS signal"]
    StartOfSlaveSelect = 2,
}
impl From<UDRDET_A> for u8 {
    #[inline(always)]
    fn from(variant: UDRDET_A) -> Self {
        variant as _
    }
}
impl UDRDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UDRDET_A> {
        match self.bits {
            0 => Some(UDRDET_A::StartOfFrame),
            1 => Some(UDRDET_A::EndOfFrame),
            2 => Some(UDRDET_A::StartOfSlaveSelect),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `StartOfFrame`"]
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == UDRDET_A::StartOfFrame
    }
    #[doc = "Checks if the value of the field is `EndOfFrame`"]
    #[inline(always)]
    pub fn is_end_of_frame(&self) -> bool {
        *self == UDRDET_A::EndOfFrame
    }
    #[doc = "Checks if the value of the field is `StartOfSlaveSelect`"]
    #[inline(always)]
    pub fn is_start_of_slave_select(&self) -> bool {
        *self == UDRDET_A::StartOfSlaveSelect
    }
}
#[doc = "Field `UDRDET` writer - Detection of underrun condition at slave transmitter"]
pub type UDRDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, UDRDET_A, 2, O>;
impl<'a, const O: u8> UDRDET_W<'a, O> {
    #[doc = "Underrun is detected at begin of data frame"]
    #[inline(always)]
    pub fn start_of_frame(self) -> &'a mut W {
        self.variant(UDRDET_A::StartOfFrame)
    }
    #[doc = "Underrun is detected at end of last data frame"]
    #[inline(always)]
    pub fn end_of_frame(self) -> &'a mut W {
        self.variant(UDRDET_A::EndOfFrame)
    }
    #[doc = "Underrun is detected at begin of active SS signal"]
    #[inline(always)]
    pub fn start_of_slave_select(self) -> &'a mut W {
        self.variant(UDRDET_A::StartOfSlaveSelect)
    }
}
#[doc = "Field `RXDMAEN` reader - Rx DMA stream enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
#[doc = "Rx DMA stream enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    #[doc = "0: Rx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    Enabled = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::Disabled,
            true => RXDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::Enabled
    }
}
#[doc = "Field `RXDMAEN` writer - Rx DMA stream enable"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, RXDMAEN_A, O>;
impl<'a, const O: u8> RXDMAEN_W<'a, O> {
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Disabled)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Enabled)
    }
}
#[doc = "Field `TXDMAEN` reader - Tx DMA stream enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
#[doc = "Tx DMA stream enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    #[doc = "0: Tx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    Enabled = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::Disabled,
            true => TXDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::Enabled
    }
}
#[doc = "Field `TXDMAEN` writer - Tx DMA stream enable"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, TXDMAEN_A, O>;
impl<'a, const O: u8> TXDMAEN_W<'a, O> {
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Disabled)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Enabled)
    }
}
#[doc = "Field `CRCSIZE` reader - Length of CRC frame to be transacted and compared"]
pub type CRCSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRCSIZE` writer - Length of CRC frame to be transacted and compared"]
pub type CRCSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CRCEN` reader - Hardware CRC computation enable"]
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
#[doc = "Hardware CRC computation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    #[doc = "0: CRC calculation disabled"]
    Disabled = 0,
    #[doc = "1: CRC calculation enabled"]
    Enabled = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::Disabled,
            true => CRCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::Enabled
    }
}
#[doc = "Field `CRCEN` writer - Hardware CRC computation enable"]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, CRCEN_A, O>;
impl<'a, const O: u8> CRCEN_W<'a, O> {
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::Disabled)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::Enabled)
    }
}
#[doc = "Field `MBR` reader - Master baud rate"]
pub type MBR_R = crate::FieldReader<u8, MBR_A>;
#[doc = "Master baud rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBR_A {
    #[doc = "0: f_spi_ker_ck / 2"]
    Div2 = 0,
    #[doc = "1: f_spi_ker_ck / 4"]
    Div4 = 1,
    #[doc = "2: f_spi_ker_ck / 8"]
    Div8 = 2,
    #[doc = "3: f_spi_ker_ck / 16"]
    Div16 = 3,
    #[doc = "4: f_spi_ker_ck / 32"]
    Div32 = 4,
    #[doc = "5: f_spi_ker_ck / 64"]
    Div64 = 5,
    #[doc = "6: f_spi_ker_ck / 128"]
    Div128 = 6,
    #[doc = "7: f_spi_ker_ck / 256"]
    Div256 = 7,
}
impl From<MBR_A> for u8 {
    #[inline(always)]
    fn from(variant: MBR_A) -> Self {
        variant as _
    }
}
impl MBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBR_A {
        match self.bits {
            0 => MBR_A::Div2,
            1 => MBR_A::Div4,
            2 => MBR_A::Div8,
            3 => MBR_A::Div16,
            4 => MBR_A::Div32,
            5 => MBR_A::Div64,
            6 => MBR_A::Div128,
            7 => MBR_A::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MBR_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MBR_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MBR_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MBR_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MBR_A::Div32
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MBR_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MBR_A::Div128
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == MBR_A::Div256
    }
}
#[doc = "Field `MBR` writer - Master baud rate"]
pub type MBR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, MBR_A, 3, O>;
impl<'a, const O: u8> MBR_W<'a, O> {
    #[doc = "f_spi_ker_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MBR_A::Div2)
    }
    #[doc = "f_spi_ker_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MBR_A::Div4)
    }
    #[doc = "f_spi_ker_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MBR_A::Div8)
    }
    #[doc = "f_spi_ker_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MBR_A::Div16)
    }
    #[doc = "f_spi_ker_ck / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(MBR_A::Div32)
    }
    #[doc = "f_spi_ker_ck / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(MBR_A::Div64)
    }
    #[doc = "f_spi_ker_ck / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(MBR_A::Div128)
    }
    #[doc = "f_spi_ker_ck / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(MBR_A::Div256)
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    pub fn udrdet(&self) -> UDRDET_R {
        UDRDET_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<0> {
        DSIZE_W::new(self)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn fthlv(&mut self) -> FTHLV_W<5> {
        FTHLV_W::new(self)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<9> {
        UDRCFG_W::new(self)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn udrdet(&mut self) -> UDRDET_W<11> {
        UDRDET_W::new(self)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<14> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<15> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<16> {
        CRCSIZE_W::new(self)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<22> {
        CRCEN_W::new(self)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<28> {
        MBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0x0007_0007"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0007;
}
