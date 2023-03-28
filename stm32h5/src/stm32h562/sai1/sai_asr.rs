#[doc = "Register `SAI_ASR` reader"]
pub struct R(crate::R<SAI_ASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_ASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_ASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_ASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVRUDR` reader - Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register."]
pub type OVRUDR_R = crate::BitReader<OVRUDRR_A>;
#[doc = "Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRUDRR_A {
    #[doc = "0: No overrun/underrun error"]
    NoError = 0,
    #[doc = "1: Overrun/underrun error detection"]
    Overrun = 1,
}
impl From<OVRUDRR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRUDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRUDRR_A {
        match self.bits {
            false => OVRUDRR_A::NoError,
            true => OVRUDRR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OVRUDRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRUDRR_A::Overrun
    }
}
#[doc = "Field `MUTEDET` reader - Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register."]
pub type MUTEDET_R = crate::BitReader<MUTEDETR_A>;
#[doc = "Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEDETR_A {
    #[doc = "0: No MUTE detection on the SD input line"]
    NoMute = 0,
    #[doc = "1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame"]
    Mute = 1,
}
impl From<MUTEDETR_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETR_A) -> Self {
        variant as u8 != 0
    }
}
impl MUTEDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTEDETR_A {
        match self.bits {
            false => MUTEDETR_A::NoMute,
            true => MUTEDETR_A::Mute,
        }
    }
    #[doc = "Checks if the value of the field is `NoMute`"]
    #[inline(always)]
    pub fn is_no_mute(&self) -> bool {
        *self == MUTEDETR_A::NoMute
    }
    #[doc = "Checks if the value of the field is `Mute`"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == MUTEDETR_A::Mute
    }
}
#[doc = "Field `WCKCFG` reader - Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\\[1\\]
= 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register."]
pub type WCKCFG_R = crate::BitReader<WCKCFGR_A>;
#[doc = "Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\\[1\\]
= 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCKCFGR_A {
    #[doc = "0: Clock configuration is correct"]
    Correct = 0,
    #[doc = "1: Clock configuration does not respect the rule concerning the frame length specification"]
    Wrong = 1,
}
impl From<WCKCFGR_A> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGR_A) -> Self {
        variant as u8 != 0
    }
}
impl WCKCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCKCFGR_A {
        match self.bits {
            false => WCKCFGR_A::Correct,
            true => WCKCFGR_A::Wrong,
        }
    }
    #[doc = "Checks if the value of the field is `Correct`"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == WCKCFGR_A::Correct
    }
    #[doc = "Checks if the value of the field is `Wrong`"]
    #[inline(always)]
    pub fn is_wrong(&self) -> bool {
        *self == WCKCFGR_A::Wrong
    }
}
#[doc = "Field `FREQ` reader - FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register."]
pub type FREQ_R = crate::BitReader<FREQR_A>;
#[doc = "FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQR_A {
    #[doc = "0: No FIFO request"]
    NoRequest = 0,
    #[doc = "1: FIFO request to read or to write the SAI_xDR"]
    Request = 1,
}
impl From<FREQR_A> for bool {
    #[inline(always)]
    fn from(variant: FREQR_A) -> Self {
        variant as u8 != 0
    }
}
impl FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQR_A {
        match self.bits {
            false => FREQR_A::NoRequest,
            true => FREQR_A::Request,
        }
    }
    #[doc = "Checks if the value of the field is `NoRequest`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == FREQR_A::NoRequest
    }
    #[doc = "Checks if the value of the field is `Request`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == FREQR_A::Request
    }
}
#[doc = "Field `CNRDY` reader - Codec not ready. This bit is read only. This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register."]
pub type CNRDY_R = crate::BitReader<CNRDYR_A>;
#[doc = "Codec not ready. This bit is read only. This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNRDYR_A {
    #[doc = "0: External AC’97 Codec is ready"]
    Ready = 0,
    #[doc = "1: External AC’97 Codec is not ready"]
    NotReady = 1,
}
impl From<CNRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: CNRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl CNRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNRDYR_A {
        match self.bits {
            false => CNRDYR_A::Ready,
            true => CNRDYR_A::NotReady,
        }
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CNRDYR_A::Ready
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == CNRDYR_A::NotReady
    }
}
#[doc = "Field `AFSDET` reader - Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC’97 or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register."]
pub type AFSDET_R = crate::BitReader<AFSDETR_A>;
#[doc = "Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC’97 or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFSDETR_A {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Frame synchronization signal is detected earlier than expected"]
    EarlySync = 1,
}
impl From<AFSDETR_A> for bool {
    #[inline(always)]
    fn from(variant: AFSDETR_A) -> Self {
        variant as u8 != 0
    }
}
impl AFSDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFSDETR_A {
        match self.bits {
            false => AFSDETR_A::NoError,
            true => AFSDETR_A::EarlySync,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AFSDETR_A::NoError
    }
    #[doc = "Checks if the value of the field is `EarlySync`"]
    #[inline(always)]
    pub fn is_early_sync(&self) -> bool {
        *self == AFSDETR_A::EarlySync
    }
}
#[doc = "Field `LFSDET` reader - Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC’97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register"]
pub type LFSDET_R = crate::BitReader<LFSDETR_A>;
#[doc = "Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC’97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFSDETR_A {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Frame synchronization signal is not present at the right time"]
    NoSync = 1,
}
impl From<LFSDETR_A> for bool {
    #[inline(always)]
    fn from(variant: LFSDETR_A) -> Self {
        variant as u8 != 0
    }
}
impl LFSDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFSDETR_A {
        match self.bits {
            false => LFSDETR_A::NoError,
            true => LFSDETR_A::NoSync,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LFSDETR_A::NoError
    }
    #[doc = "Checks if the value of the field is `NoSync`"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == LFSDETR_A::NoSync
    }
}
#[doc = "Field `FLVL` reader - FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). Others: Reserved"]
pub type FLVL_R = crate::FieldReader<u8, FLVLR_A>;
#[doc = "FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLVLR_A {
    #[doc = "0: FIFO empty"]
    Empty = 0,
    #[doc = "1: FIFO <= 1⁄4 but not empty"]
    Quarter1 = 1,
    #[doc = "2: 1⁄4 < FIFO <= 1⁄2"]
    Quarter2 = 2,
    #[doc = "3: 1⁄2 < FIFO <= 3⁄4"]
    Quarter3 = 3,
    #[doc = "4: 3⁄4 < FIFO but not full"]
    Quarter4 = 4,
    #[doc = "5: FIFO full"]
    Full = 5,
}
impl From<FLVLR_A> for u8 {
    #[inline(always)]
    fn from(variant: FLVLR_A) -> Self {
        variant as _
    }
}
impl FLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLVLR_A> {
        match self.bits {
            0 => Some(FLVLR_A::Empty),
            1 => Some(FLVLR_A::Quarter1),
            2 => Some(FLVLR_A::Quarter2),
            3 => Some(FLVLR_A::Quarter3),
            4 => Some(FLVLR_A::Quarter4),
            5 => Some(FLVLR_A::Full),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FLVLR_A::Empty
    }
    #[doc = "Checks if the value of the field is `Quarter1`"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FLVLR_A::Quarter1
    }
    #[doc = "Checks if the value of the field is `Quarter2`"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FLVLR_A::Quarter2
    }
    #[doc = "Checks if the value of the field is `Quarter3`"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FLVLR_A::Quarter3
    }
    #[doc = "Checks if the value of the field is `Quarter4`"]
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        *self == FLVLR_A::Quarter4
    }
    #[doc = "Checks if the value of the field is `Full`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FLVLR_A::Full
    }
}
impl R {
    #[doc = "Bit 0 - Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register."]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register."]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\\[1\\]
= 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register."]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register."]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready. This bit is read only. This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register."]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC’97 or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register."]
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC’97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). Others: Reserved"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "SAI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_asr](index.html) module"]
pub struct SAI_ASR_SPEC;
impl crate::RegisterSpec for SAI_ASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_asr::R](R) reader structure"]
impl crate::Readable for SAI_ASR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAI_ASR to value 0x08"]
impl crate::Resettable for SAI_ASR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
