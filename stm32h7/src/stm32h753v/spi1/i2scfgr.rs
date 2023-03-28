#[doc = "Register `I2SCFGR` reader"]
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCFGR` writer"]
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub type I2SMOD_R = crate::BitReader<I2SMOD_A>;
#[doc = "I2S mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD_A {
    #[doc = "0: SPI mode selected"]
    Spi = 0,
    #[doc = "1: I2S/PCM mode selected"]
    I2s = 1,
}
impl From<I2SMOD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SMOD_A {
        match self.bits {
            false => I2SMOD_A::Spi,
            true => I2SMOD_A::I2s,
        }
    }
    #[doc = "Checks if the value of the field is `Spi`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == I2SMOD_A::Spi
    }
    #[doc = "Checks if the value of the field is `I2s`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == I2SMOD_A::I2s
    }
}
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub type I2SMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, I2SMOD_A, O>;
impl<'a, const O: u8> I2SMOD_W<'a, O> {
    #[doc = "SPI mode selected"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(I2SMOD_A::Spi)
    }
    #[doc = "I2S/PCM mode selected"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(I2SMOD_A::I2s)
    }
}
#[doc = "Field `I2SCFG` reader - I2S configuration mode"]
pub type I2SCFG_R = crate::FieldReader<u8, I2SCFG_A>;
#[doc = "I2S configuration mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG_A {
    #[doc = "0: Slave, transmit"]
    SlaveTransmit = 0,
    #[doc = "1: Slave, recteive"]
    SlaveReceive = 1,
    #[doc = "2: Master, transmit"]
    MasterTransmit = 2,
    #[doc = "3: Master, receive"]
    MasterReceive = 3,
    #[doc = "4: Slave, full duplex"]
    SlaveFullDuplex = 4,
    #[doc = "5: Master, full duplex"]
    MasterFullDuplex = 5,
}
impl From<I2SCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG_A) -> Self {
        variant as _
    }
}
impl I2SCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2SCFG_A> {
        match self.bits {
            0 => Some(I2SCFG_A::SlaveTransmit),
            1 => Some(I2SCFG_A::SlaveReceive),
            2 => Some(I2SCFG_A::MasterTransmit),
            3 => Some(I2SCFG_A::MasterReceive),
            4 => Some(I2SCFG_A::SlaveFullDuplex),
            5 => Some(I2SCFG_A::MasterFullDuplex),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SlaveTransmit`"]
    #[inline(always)]
    pub fn is_slave_transmit(&self) -> bool {
        *self == I2SCFG_A::SlaveTransmit
    }
    #[doc = "Checks if the value of the field is `SlaveReceive`"]
    #[inline(always)]
    pub fn is_slave_receive(&self) -> bool {
        *self == I2SCFG_A::SlaveReceive
    }
    #[doc = "Checks if the value of the field is `MasterTransmit`"]
    #[inline(always)]
    pub fn is_master_transmit(&self) -> bool {
        *self == I2SCFG_A::MasterTransmit
    }
    #[doc = "Checks if the value of the field is `MasterReceive`"]
    #[inline(always)]
    pub fn is_master_receive(&self) -> bool {
        *self == I2SCFG_A::MasterReceive
    }
    #[doc = "Checks if the value of the field is `SlaveFullDuplex`"]
    #[inline(always)]
    pub fn is_slave_full_duplex(&self) -> bool {
        *self == I2SCFG_A::SlaveFullDuplex
    }
    #[doc = "Checks if the value of the field is `MasterFullDuplex`"]
    #[inline(always)]
    pub fn is_master_full_duplex(&self) -> bool {
        *self == I2SCFG_A::MasterFullDuplex
    }
}
#[doc = "Field `I2SCFG` writer - I2S configuration mode"]
pub type I2SCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, I2SCFG_A, 3, O>;
impl<'a, const O: u8> I2SCFG_W<'a, O> {
    #[doc = "Slave, transmit"]
    #[inline(always)]
    pub fn slave_transmit(self) -> &'a mut W {
        self.variant(I2SCFG_A::SlaveTransmit)
    }
    #[doc = "Slave, recteive"]
    #[inline(always)]
    pub fn slave_receive(self) -> &'a mut W {
        self.variant(I2SCFG_A::SlaveReceive)
    }
    #[doc = "Master, transmit"]
    #[inline(always)]
    pub fn master_transmit(self) -> &'a mut W {
        self.variant(I2SCFG_A::MasterTransmit)
    }
    #[doc = "Master, receive"]
    #[inline(always)]
    pub fn master_receive(self) -> &'a mut W {
        self.variant(I2SCFG_A::MasterReceive)
    }
    #[doc = "Slave, full duplex"]
    #[inline(always)]
    pub fn slave_full_duplex(self) -> &'a mut W {
        self.variant(I2SCFG_A::SlaveFullDuplex)
    }
    #[doc = "Master, full duplex"]
    #[inline(always)]
    pub fn master_full_duplex(self) -> &'a mut W {
        self.variant(I2SCFG_A::MasterFullDuplex)
    }
}
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2SSTD_R = crate::FieldReader<u8, I2SSTD_A>;
#[doc = "I2S standard selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD_A {
    #[doc = "0: I2S Philips standard"]
    Philips = 0,
    #[doc = "1: MSB/left justified standard"]
    LeftAligned = 1,
    #[doc = "2: LSB/right justified standard"]
    RightAligned = 2,
    #[doc = "3: PCM standard"]
    Pcm = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
impl I2SSTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::Philips,
            1 => I2SSTD_A::LeftAligned,
            2 => I2SSTD_A::RightAligned,
            3 => I2SSTD_A::Pcm,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Philips`"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD_A::Philips
    }
    #[doc = "Checks if the value of the field is `LeftAligned`"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        *self == I2SSTD_A::LeftAligned
    }
    #[doc = "Checks if the value of the field is `RightAligned`"]
    #[inline(always)]
    pub fn is_right_aligned(&self) -> bool {
        *self == I2SSTD_A::RightAligned
    }
    #[doc = "Checks if the value of the field is `Pcm`"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD_A::Pcm
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2SSTD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I2SCFGR_SPEC, u8, I2SSTD_A, 2, O>;
impl<'a, const O: u8> I2SSTD_W<'a, O> {
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn philips(self) -> &'a mut W {
        self.variant(I2SSTD_A::Philips)
    }
    #[doc = "MSB/left justified standard"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut W {
        self.variant(I2SSTD_A::LeftAligned)
    }
    #[doc = "LSB/right justified standard"]
    #[inline(always)]
    pub fn right_aligned(self) -> &'a mut W {
        self.variant(I2SSTD_A::RightAligned)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut W {
        self.variant(I2SSTD_A::Pcm)
    }
}
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
pub type PCMSYNC_R = crate::BitReader<PCMSYNC_A>;
#[doc = "PCM frame synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC_A {
    #[doc = "0: Short PCM frame synchronization"]
    Short = 0,
    #[doc = "1: Long PCM frame synchronization"]
    Long = 1,
}
impl From<PCMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl PCMSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCMSYNC_A {
        match self.bits {
            false => PCMSYNC_A::Short,
            true => PCMSYNC_A::Long,
        }
    }
    #[doc = "Checks if the value of the field is `Short`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNC_A::Short
    }
    #[doc = "Checks if the value of the field is `Long`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNC_A::Long
    }
}
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub type PCMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, PCMSYNC_A, O>;
impl<'a, const O: u8> PCMSYNC_W<'a, O> {
    #[doc = "Short PCM frame synchronization"]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSYNC_A::Short)
    }
    #[doc = "Long PCM frame synchronization"]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(PCMSYNC_A::Long)
    }
}
#[doc = "Field `DATLEN` reader - Data length to be transferred"]
pub type DATLEN_R = crate::FieldReader<u8, DATLEN_A>;
#[doc = "Data length to be transferred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN_A {
    #[doc = "0: 16 bit data length"]
    Bits16 = 0,
    #[doc = "1: 24 bit data length"]
    Bits24 = 1,
    #[doc = "2: 32 bit data length"]
    Bits32 = 2,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
impl DATLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATLEN_A> {
        match self.bits {
            0 => Some(DATLEN_A::Bits16),
            1 => Some(DATLEN_A::Bits24),
            2 => Some(DATLEN_A::Bits32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == DATLEN_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits24`"]
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == DATLEN_A::Bits24
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == DATLEN_A::Bits32
    }
}
#[doc = "Field `DATLEN` writer - Data length to be transferred"]
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, DATLEN_A, 2, O>;
impl<'a, const O: u8> DATLEN_W<'a, O> {
    #[doc = "16 bit data length"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(DATLEN_A::Bits16)
    }
    #[doc = "24 bit data length"]
    #[inline(always)]
    pub fn bits24(self) -> &'a mut W {
        self.variant(DATLEN_A::Bits24)
    }
    #[doc = "32 bit data length"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(DATLEN_A::Bits32)
    }
}
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader<CHLEN_A>;
#[doc = "Channel length (number of bits per audio channel)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN_A {
    #[doc = "0: 16 bit per channel"]
    Bits16 = 0,
    #[doc = "1: 32 bit per channel"]
    Bits32 = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::Bits16,
            true => CHLEN_A::Bits32,
        }
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == CHLEN_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == CHLEN_A::Bits32
    }
}
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, CHLEN_A, O>;
impl<'a, const O: u8> CHLEN_W<'a, O> {
    #[doc = "16 bit per channel"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(CHLEN_A::Bits16)
    }
    #[doc = "32 bit per channel"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(CHLEN_A::Bits32)
    }
}
#[doc = "Field `CKPOL` reader - Serial audio clock polarity"]
pub type CKPOL_R = crate::BitReader<CKPOL_A>;
#[doc = "Serial audio clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL_A {
    #[doc = "0: Signals are sampled on rising and changed on falling clock edges"]
    SampleOnRising = 0,
    #[doc = "1: Signals are sampled on falling and changed on rising clock edges"]
    SampleOnFalling = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::SampleOnRising,
            true => CKPOL_A::SampleOnFalling,
        }
    }
    #[doc = "Checks if the value of the field is `SampleOnRising`"]
    #[inline(always)]
    pub fn is_sample_on_rising(&self) -> bool {
        *self == CKPOL_A::SampleOnRising
    }
    #[doc = "Checks if the value of the field is `SampleOnFalling`"]
    #[inline(always)]
    pub fn is_sample_on_falling(&self) -> bool {
        *self == CKPOL_A::SampleOnFalling
    }
}
#[doc = "Field `CKPOL` writer - Serial audio clock polarity"]
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, CKPOL_A, O>;
impl<'a, const O: u8> CKPOL_W<'a, O> {
    #[doc = "Signals are sampled on rising and changed on falling clock edges"]
    #[inline(always)]
    pub fn sample_on_rising(self) -> &'a mut W {
        self.variant(CKPOL_A::SampleOnRising)
    }
    #[doc = "Signals are sampled on falling and changed on rising clock edges"]
    #[inline(always)]
    pub fn sample_on_falling(self) -> &'a mut W {
        self.variant(CKPOL_A::SampleOnFalling)
    }
}
#[doc = "Field `FIXCH` reader - Word select inversion"]
pub type FIXCH_R = crate::BitReader<FIXCH_A>;
#[doc = "Word select inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXCH_A {
    #[doc = "0: The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    NotFixed = 0,
    #[doc = "1: The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    Fixed = 1,
}
impl From<FIXCH_A> for bool {
    #[inline(always)]
    fn from(variant: FIXCH_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXCH_A {
        match self.bits {
            false => FIXCH_A::NotFixed,
            true => FIXCH_A::Fixed,
        }
    }
    #[doc = "Checks if the value of the field is `NotFixed`"]
    #[inline(always)]
    pub fn is_not_fixed(&self) -> bool {
        *self == FIXCH_A::NotFixed
    }
    #[doc = "Checks if the value of the field is `Fixed`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == FIXCH_A::Fixed
    }
}
#[doc = "Field `FIXCH` writer - Word select inversion"]
pub type FIXCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, FIXCH_A, O>;
impl<'a, const O: u8> FIXCH_W<'a, O> {
    #[doc = "The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    #[inline(always)]
    pub fn not_fixed(self) -> &'a mut W {
        self.variant(FIXCH_A::NotFixed)
    }
    #[doc = "The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(FIXCH_A::Fixed)
    }
}
#[doc = "Field `WSINV` reader - Fixed channel length in SLAVE"]
pub type WSINV_R = crate::BitReader<WSINV_A>;
#[doc = "Fixed channel length in SLAVE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSINV_A {
    #[doc = "0: Word select inversion disabled"]
    Disabled = 0,
    #[doc = "1: Word select inversion enabled"]
    Enabled = 1,
}
impl From<WSINV_A> for bool {
    #[inline(always)]
    fn from(variant: WSINV_A) -> Self {
        variant as u8 != 0
    }
}
impl WSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSINV_A {
        match self.bits {
            false => WSINV_A::Disabled,
            true => WSINV_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WSINV_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WSINV_A::Enabled
    }
}
#[doc = "Field `WSINV` writer - Fixed channel length in SLAVE"]
pub type WSINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, WSINV_A, O>;
impl<'a, const O: u8> WSINV_W<'a, O> {
    #[doc = "Word select inversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WSINV_A::Disabled)
    }
    #[doc = "Word select inversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WSINV_A::Enabled)
    }
}
#[doc = "Field `DATFMT` reader - Data format"]
pub type DATFMT_R = crate::BitReader<DATFMT_A>;
#[doc = "Data format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATFMT_A {
    #[doc = "0: The data inside RXDR and TXDR are right aligned"]
    RightAligned = 0,
    #[doc = "1: The data inside RXDR and TXDR are left aligned"]
    LeftAligned = 1,
}
impl From<DATFMT_A> for bool {
    #[inline(always)]
    fn from(variant: DATFMT_A) -> Self {
        variant as u8 != 0
    }
}
impl DATFMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATFMT_A {
        match self.bits {
            false => DATFMT_A::RightAligned,
            true => DATFMT_A::LeftAligned,
        }
    }
    #[doc = "Checks if the value of the field is `RightAligned`"]
    #[inline(always)]
    pub fn is_right_aligned(&self) -> bool {
        *self == DATFMT_A::RightAligned
    }
    #[doc = "Checks if the value of the field is `LeftAligned`"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        *self == DATFMT_A::LeftAligned
    }
}
#[doc = "Field `DATFMT` writer - Data format"]
pub type DATFMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, DATFMT_A, O>;
impl<'a, const O: u8> DATFMT_W<'a, O> {
    #[doc = "The data inside RXDR and TXDR are right aligned"]
    #[inline(always)]
    pub fn right_aligned(self) -> &'a mut W {
        self.variant(DATFMT_A::RightAligned)
    }
    #[doc = "The data inside RXDR and TXDR are left aligned"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut W {
        self.variant(DATFMT_A::LeftAligned)
    }
}
#[doc = "Field `I2SDIV` reader - I2S linear prescaler"]
pub type I2SDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SDIV` writer - I2S linear prescaler"]
pub type I2SDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, I2SCFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `ODD` reader - Odd factor for the prescaler"]
pub type ODD_R = crate::BitReader<ODD_A>;
#[doc = "Odd factor for the prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD_A {
    #[doc = "0: Real divider value is I2SDIV*2"]
    Even = 0,
    #[doc = "1: Real divider value is I2SDIV*2 + 1"]
    Odd = 1,
}
impl From<ODD_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_A) -> Self {
        variant as u8 != 0
    }
}
impl ODD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODD_A {
        match self.bits {
            false => ODD_A::Even,
            true => ODD_A::Odd,
        }
    }
    #[doc = "Checks if the value of the field is `Even`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == ODD_A::Even
    }
    #[doc = "Checks if the value of the field is `Odd`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == ODD_A::Odd
    }
}
#[doc = "Field `ODD` writer - Odd factor for the prescaler"]
pub type ODD_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, ODD_A, O>;
impl<'a, const O: u8> ODD_W<'a, O> {
    #[doc = "Real divider value is I2SDIV*2"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(ODD_A::Even)
    }
    #[doc = "Real divider value is I2SDIV*2 + 1"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(ODD_A::Odd)
    }
}
#[doc = "Field `MCKOE` reader - Master clock output enable"]
pub type MCKOE_R = crate::BitReader<MCKOE_A>;
#[doc = "Master clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOE_A {
    #[doc = "0: Master clock output disabled"]
    Disabled = 0,
    #[doc = "1: Master clock output enabled"]
    Enabled = 1,
}
impl From<MCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOE_A) -> Self {
        variant as u8 != 0
    }
}
impl MCKOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKOE_A {
        match self.bits {
            false => MCKOE_A::Disabled,
            true => MCKOE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOE_A::Enabled
    }
}
#[doc = "Field `MCKOE` writer - Master clock output enable"]
pub type MCKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, MCKOE_A, O>;
impl<'a, const O: u8> MCKOE_W<'a, O> {
    #[doc = "Master clock output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKOE_A::Disabled)
    }
    #[doc = "Master clock output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKOE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Serial audio clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word select inversion"]
    #[inline(always)]
    pub fn fixch(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fixed channel length in SLAVE"]
    #[inline(always)]
    pub fn wsinv(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Data format"]
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<0> {
        I2SMOD_W::new(self)
    }
    #[doc = "Bits 1:3 - I2S configuration mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<1> {
        I2SCFG_W::new(self)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<4> {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<7> {
        PCMSYNC_W::new(self)
    }
    #[doc = "Bits 8:9 - Data length to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<8> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 10 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<10> {
        CHLEN_W::new(self)
    }
    #[doc = "Bit 11 - Serial audio clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<11> {
        CKPOL_W::new(self)
    }
    #[doc = "Bit 12 - Word select inversion"]
    #[inline(always)]
    #[must_use]
    pub fn fixch(&mut self) -> FIXCH_W<12> {
        FIXCH_W::new(self)
    }
    #[doc = "Bit 13 - Fixed channel length in SLAVE"]
    #[inline(always)]
    #[must_use]
    pub fn wsinv(&mut self) -> WSINV_W<13> {
        WSINV_W::new(self)
    }
    #[doc = "Bit 14 - Data format"]
    #[inline(always)]
    #[must_use]
    pub fn datfmt(&mut self) -> DATFMT_W<14> {
        DATFMT_W::new(self)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<16> {
        I2SDIV_W::new(self)
    }
    #[doc = "Bit 24 - Odd factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<24> {
        ODD_W::new(self)
    }
    #[doc = "Bit 25 - Master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<25> {
        MCKOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scfgr](index.html) module"]
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2scfgr::R](R) reader structure"]
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2scfgr::W](W) writer structure"]
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
