#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SITP` reader - SITP"]
pub type SITP_R = crate::FieldReader<u8, SITP_A>;
#[doc = "SITP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SITP_A {
    #[doc = "0: SPI with rising edge to strobe data"]
    SpirisingEdge = 0,
    #[doc = "1: SPI with falling edge to strobe data"]
    SpifallingEdge = 1,
    #[doc = "2: Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1"]
    Manchester = 2,
    #[doc = "3: Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0"]
    ManchesterInverted = 3,
}
impl From<SITP_A> for u8 {
    #[inline(always)]
    fn from(variant: SITP_A) -> Self {
        variant as _
    }
}
impl SITP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SITP_A {
        match self.bits {
            0 => SITP_A::SpirisingEdge,
            1 => SITP_A::SpifallingEdge,
            2 => SITP_A::Manchester,
            3 => SITP_A::ManchesterInverted,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SpirisingEdge`"]
    #[inline(always)]
    pub fn is_spirising_edge(&self) -> bool {
        *self == SITP_A::SpirisingEdge
    }
    #[doc = "Checks if the value of the field is `SpifallingEdge`"]
    #[inline(always)]
    pub fn is_spifalling_edge(&self) -> bool {
        *self == SITP_A::SpifallingEdge
    }
    #[doc = "Checks if the value of the field is `Manchester`"]
    #[inline(always)]
    pub fn is_manchester(&self) -> bool {
        *self == SITP_A::Manchester
    }
    #[doc = "Checks if the value of the field is `ManchesterInverted`"]
    #[inline(always)]
    pub fn is_manchester_inverted(&self) -> bool {
        *self == SITP_A::ManchesterInverted
    }
}
#[doc = "Field `SITP` writer - SITP"]
pub type SITP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, SITP_A, 2, O>;
impl<'a, const O: u8> SITP_W<'a, O> {
    #[doc = "SPI with rising edge to strobe data"]
    #[inline(always)]
    pub fn spirising_edge(self) -> &'a mut W {
        self.variant(SITP_A::SpirisingEdge)
    }
    #[doc = "SPI with falling edge to strobe data"]
    #[inline(always)]
    pub fn spifalling_edge(self) -> &'a mut W {
        self.variant(SITP_A::SpifallingEdge)
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1"]
    #[inline(always)]
    pub fn manchester(self) -> &'a mut W {
        self.variant(SITP_A::Manchester)
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0"]
    #[inline(always)]
    pub fn manchester_inverted(self) -> &'a mut W {
        self.variant(SITP_A::ManchesterInverted)
    }
}
#[doc = "Field `SPICKSEL` reader - SPICKSEL"]
pub type SPICKSEL_R = crate::FieldReader<u8, SPICKSEL_A>;
#[doc = "SPICKSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPICKSEL_A {
    #[doc = "0: Clock coming from external CKINy input - sampling point according SITP\\[1:0\\]"]
    Ckin = 0,
    #[doc = "1: Clock coming from internal CKOUT output - sampling point according SITP\\[1:0\\]"]
    Ckout = 1,
    #[doc = "2: Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)"]
    CkoutsecondFalling = 2,
    #[doc = "3: Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)"]
    CkoutsecondRising = 3,
}
impl From<SPICKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPICKSEL_A) -> Self {
        variant as _
    }
}
impl SPICKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPICKSEL_A {
        match self.bits {
            0 => SPICKSEL_A::Ckin,
            1 => SPICKSEL_A::Ckout,
            2 => SPICKSEL_A::CkoutsecondFalling,
            3 => SPICKSEL_A::CkoutsecondRising,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ckin`"]
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == SPICKSEL_A::Ckin
    }
    #[doc = "Checks if the value of the field is `Ckout`"]
    #[inline(always)]
    pub fn is_ckout(&self) -> bool {
        *self == SPICKSEL_A::Ckout
    }
    #[doc = "Checks if the value of the field is `CkoutsecondFalling`"]
    #[inline(always)]
    pub fn is_ckoutsecond_falling(&self) -> bool {
        *self == SPICKSEL_A::CkoutsecondFalling
    }
    #[doc = "Checks if the value of the field is `CkoutsecondRising`"]
    #[inline(always)]
    pub fn is_ckoutsecond_rising(&self) -> bool {
        *self == SPICKSEL_A::CkoutsecondRising
    }
}
#[doc = "Field `SPICKSEL` writer - SPICKSEL"]
pub type SPICKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, SPICKSEL_A, 2, O>;
impl<'a, const O: u8> SPICKSEL_W<'a, O> {
    #[doc = "Clock coming from external CKINy input - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn ckin(self) -> &'a mut W {
        self.variant(SPICKSEL_A::Ckin)
    }
    #[doc = "Clock coming from internal CKOUT output - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn ckout(self) -> &'a mut W {
        self.variant(SPICKSEL_A::Ckout)
    }
    #[doc = "Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)"]
    #[inline(always)]
    pub fn ckoutsecond_falling(self) -> &'a mut W {
        self.variant(SPICKSEL_A::CkoutsecondFalling)
    }
    #[doc = "Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)"]
    #[inline(always)]
    pub fn ckoutsecond_rising(self) -> &'a mut W {
        self.variant(SPICKSEL_A::CkoutsecondRising)
    }
}
#[doc = "Field `SCDEN` reader - SCDEN"]
pub type SCDEN_R = crate::BitReader<SCDEN_A>;
#[doc = "SCDEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCDEN_A {
    #[doc = "0: Input channel y will not be guarded by the short-circuit detector"]
    Disabled = 0,
    #[doc = "1: Input channel y will be continuously guarded by the short-circuit detector"]
    Enabled = 1,
}
impl From<SCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCDEN_A {
        match self.bits {
            false => SCDEN_A::Disabled,
            true => SCDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCDEN_A::Enabled
    }
}
#[doc = "Field `SCDEN` writer - SCDEN"]
pub type SCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, SCDEN_A, O>;
impl<'a, const O: u8> SCDEN_W<'a, O> {
    #[doc = "Input channel y will not be guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCDEN_A::Disabled)
    }
    #[doc = "Input channel y will be continuously guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCDEN_A::Enabled)
    }
}
#[doc = "Field `CKABEN` reader - CKABEN"]
pub type CKABEN_R = crate::BitReader<CKABEN_A>;
#[doc = "CKABEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKABEN_A {
    #[doc = "0: Clock absence detector disabled on channel y"]
    Disabled = 0,
    #[doc = "1: Clock absence detector enabled on channel y"]
    Enabled = 1,
}
impl From<CKABEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKABEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CKABEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKABEN_A {
        match self.bits {
            false => CKABEN_A::Disabled,
            true => CKABEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKABEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKABEN_A::Enabled
    }
}
#[doc = "Field `CKABEN` writer - CKABEN"]
pub type CKABEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, CKABEN_A, O>;
impl<'a, const O: u8> CKABEN_W<'a, O> {
    #[doc = "Clock absence detector disabled on channel y"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CKABEN_A::Disabled)
    }
    #[doc = "Clock absence detector enabled on channel y"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CKABEN_A::Enabled)
    }
}
#[doc = "Field `CHEN` reader - CHEN"]
pub type CHEN_R = crate::BitReader<CHEN_A>;
#[doc = "CHEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHEN_A {
    #[doc = "0: Channel y disabled"]
    Disabled = 0,
    #[doc = "1: Channel y enabled"]
    Enabled = 1,
}
impl From<CHEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN_A {
        match self.bits {
            false => CHEN_A::Disabled,
            true => CHEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHEN_A::Enabled
    }
}
#[doc = "Field `CHEN` writer - CHEN"]
pub type CHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, CHEN_A, O>;
impl<'a, const O: u8> CHEN_W<'a, O> {
    #[doc = "Channel y disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CHEN_A::Disabled)
    }
    #[doc = "Channel y enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CHEN_A::Enabled)
    }
}
#[doc = "Field `CHINSEL` reader - CHINSEL"]
pub type CHINSEL_R = crate::BitReader<CHINSEL_A>;
#[doc = "CHINSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHINSEL_A {
    #[doc = "0: Channel inputs are taken from pins of the same channel y"]
    SameChannel = 0,
    #[doc = "1: Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)"]
    FollowingChannel = 1,
}
impl From<CHINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CHINSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CHINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHINSEL_A {
        match self.bits {
            false => CHINSEL_A::SameChannel,
            true => CHINSEL_A::FollowingChannel,
        }
    }
    #[doc = "Checks if the value of the field is `SameChannel`"]
    #[inline(always)]
    pub fn is_same_channel(&self) -> bool {
        *self == CHINSEL_A::SameChannel
    }
    #[doc = "Checks if the value of the field is `FollowingChannel`"]
    #[inline(always)]
    pub fn is_following_channel(&self) -> bool {
        *self == CHINSEL_A::FollowingChannel
    }
}
#[doc = "Field `CHINSEL` writer - CHINSEL"]
pub type CHINSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, CHINSEL_A, O>;
impl<'a, const O: u8> CHINSEL_W<'a, O> {
    #[doc = "Channel inputs are taken from pins of the same channel y"]
    #[inline(always)]
    pub fn same_channel(self) -> &'a mut W {
        self.variant(CHINSEL_A::SameChannel)
    }
    #[doc = "Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)"]
    #[inline(always)]
    pub fn following_channel(self) -> &'a mut W {
        self.variant(CHINSEL_A::FollowingChannel)
    }
}
#[doc = "Field `DATMPX` reader - DATMPX"]
pub type DATMPX_R = crate::FieldReader<u8, DATMPX_A>;
#[doc = "DATMPX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATMPX_A {
    #[doc = "0: Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected"]
    External = 0,
    #[doc = "1: Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR register"]
    Adc = 1,
    #[doc = "2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting"]
    Internal = 2,
}
impl From<DATMPX_A> for u8 {
    #[inline(always)]
    fn from(variant: DATMPX_A) -> Self {
        variant as _
    }
}
impl DATMPX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATMPX_A> {
        match self.bits {
            0 => Some(DATMPX_A::External),
            1 => Some(DATMPX_A::Adc),
            2 => Some(DATMPX_A::Internal),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `External`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == DATMPX_A::External
    }
    #[doc = "Checks if the value of the field is `Adc`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == DATMPX_A::Adc
    }
    #[doc = "Checks if the value of the field is `Internal`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == DATMPX_A::Internal
    }
}
#[doc = "Field `DATMPX` writer - DATMPX"]
pub type DATMPX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, DATMPX_A, 2, O>;
impl<'a, const O: u8> DATMPX_W<'a, O> {
    #[doc = "Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(DATMPX_A::External)
    }
    #[doc = "Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR register"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(DATMPX_A::Adc)
    }
    #[doc = "Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(DATMPX_A::Internal)
    }
}
#[doc = "Field `DATPACK` reader - DATPACK"]
pub type DATPACK_R = crate::FieldReader<u8, DATPACK_A>;
#[doc = "DATPACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATPACK_A {
    #[doc = "0: Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y"]
    Standard = 0,
    #[doc = "1: : Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y)"]
    Interleaved = 1,
    #[doc = "2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y+1)"]
    Dual = 2,
}
impl From<DATPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: DATPACK_A) -> Self {
        variant as _
    }
}
impl DATPACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATPACK_A> {
        match self.bits {
            0 => Some(DATPACK_A::Standard),
            1 => Some(DATPACK_A::Interleaved),
            2 => Some(DATPACK_A::Dual),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DATPACK_A::Standard
    }
    #[doc = "Checks if the value of the field is `Interleaved`"]
    #[inline(always)]
    pub fn is_interleaved(&self) -> bool {
        *self == DATPACK_A::Interleaved
    }
    #[doc = "Checks if the value of the field is `Dual`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DATPACK_A::Dual
    }
}
#[doc = "Field `DATPACK` writer - DATPACK"]
pub type DATPACK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, DATPACK_A, 2, O>;
impl<'a, const O: u8> DATPACK_W<'a, O> {
    #[doc = "Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(DATPACK_A::Standard)
    }
    #[doc = ": Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y)"]
    #[inline(always)]
    pub fn interleaved(self) -> &'a mut W {
        self.variant(DATPACK_A::Interleaved)
    }
    #[doc = "Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y+1)"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DATPACK_A::Dual)
    }
}
#[doc = "Field `CKOUTDIV` reader - CKOUTDIV"]
pub type CKOUTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUTDIV` writer - CKOUTDIV"]
pub type CKOUTDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKOUTSRC` reader - CKOUTSRC"]
pub type CKOUTSRC_R = crate::BitReader<CKOUTSRC_A>;
#[doc = "CKOUTSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKOUTSRC_A {
    #[doc = "0: Source for output clock is from system clock"]
    Sysclk = 0,
    #[doc = "1: Source for output clock is from audio clock"]
    Audclk = 1,
}
impl From<CKOUTSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CKOUTSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CKOUTSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKOUTSRC_A {
        match self.bits {
            false => CKOUTSRC_A::Sysclk,
            true => CKOUTSRC_A::Audclk,
        }
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKOUTSRC_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Audclk`"]
    #[inline(always)]
    pub fn is_audclk(&self) -> bool {
        *self == CKOUTSRC_A::Audclk
    }
}
#[doc = "Field `CKOUTSRC` writer - CKOUTSRC"]
pub type CKOUTSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, CKOUTSRC_A, O>;
impl<'a, const O: u8> CKOUTSRC_W<'a, O> {
    #[doc = "Source for output clock is from system clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CKOUTSRC_A::Sysclk)
    }
    #[doc = "Source for output clock is from audio clock"]
    #[inline(always)]
    pub fn audclk(self) -> &'a mut W {
        self.variant(CKOUTSRC_A::Audclk)
    }
}
#[doc = "Field `DFSDMEN` reader - DFSDMEN"]
pub type DFSDMEN_R = crate::BitReader<DFSDMEN_A>;
#[doc = "DFSDMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDMEN_A {
    #[doc = "0: DFSDM interface disabled"]
    Disabled = 0,
    #[doc = "1: DFSDM interface enabled"]
    Enabled = 1,
}
impl From<DFSDMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DFSDMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDMEN_A {
        match self.bits {
            false => DFSDMEN_A::Disabled,
            true => DFSDMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFSDMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFSDMEN_A::Enabled
    }
}
#[doc = "Field `DFSDMEN` writer - DFSDMEN"]
pub type DFSDMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, DFSDMEN_A, O>;
impl<'a, const O: u8> DFSDMEN_W<'a, O> {
    #[doc = "DFSDM interface disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFSDMEN_A::Disabled)
    }
    #[doc = "DFSDM interface enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DFSDMEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    #[must_use]
    pub fn sitp(&mut self) -> SITP_W<0> {
        SITP_W::new(self)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn spicksel(&mut self) -> SPICKSEL_W<2> {
        SPICKSEL_W::new(self)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<5> {
        SCDEN_W::new(self)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckaben(&mut self) -> CKABEN_W<6> {
        CKABEN_W::new(self)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<7> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chinsel(&mut self) -> CHINSEL_W<8> {
        CHINSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    #[must_use]
    pub fn datmpx(&mut self) -> DATMPX_W<12> {
        DATMPX_W::new(self)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    #[must_use]
    pub fn datpack(&mut self) -> DATPACK_W<14> {
        DATPACK_W::new(self)
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W<16> {
        CKOUTDIV_W::new(self)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<30> {
        CKOUTSRC_W::new(self)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<31> {
        DFSDMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel configuration y register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
