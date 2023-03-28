#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFEN` reader - DFSDM enable"]
pub type DFEN_R = crate::BitReader<DFEN_A>;
#[doc = "DFSDM enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFEN_A {
    #[doc = "0: DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped"]
    Disabled = 0,
    #[doc = "1: DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting"]
    Enabled = 1,
}
impl From<DFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFEN_A {
        match self.bits {
            false => DFEN_A::Disabled,
            true => DFEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFEN_A::Enabled
    }
}
#[doc = "Field `DFEN` writer - DFSDM enable"]
pub type DFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, DFEN_A, O>;
impl<'a, const O: u8> DFEN_W<'a, O> {
    #[doc = "DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFEN_A::Disabled)
    }
    #[doc = "DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DFEN_A::Enabled)
    }
}
#[doc = "Field `JSWSTART` reader - Start a conversion of the injected group of channels"]
pub type JSWSTART_R = crate::BitReader<JSWSTARTW_A>;
#[doc = "Start a conversion of the injected group of channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW_A {
    #[doc = "1: Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1"]
    Start = 1,
}
impl From<JSWSTARTW_A> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW_A) -> Self {
        variant as u8 != 0
    }
}
impl JSWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<JSWSTARTW_A> {
        match self.bits {
            true => Some(JSWSTARTW_A::Start),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == JSWSTARTW_A::Start
    }
}
#[doc = "Field `JSWSTART` writer - Start a conversion of the injected group of channels"]
pub type JSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, JSWSTARTW_A, O>;
impl<'a, const O: u8> JSWSTART_W<'a, O> {
    #[doc = "Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JSWSTARTW_A::Start)
    }
}
#[doc = "Field `JSYNC` reader - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
pub type JSYNC_R = crate::BitReader<JSYNC_A>;
#[doc = "Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSYNC_A {
    #[doc = "0: Do not launch an injected conversion synchronously with DFSDM_FLT0"]
    Disabled = 0,
    #[doc = "1: Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger"]
    Enabled = 1,
}
impl From<JSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: JSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl JSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSYNC_A {
        match self.bits {
            false => JSYNC_A::Disabled,
            true => JSYNC_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JSYNC_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JSYNC_A::Enabled
    }
}
#[doc = "Field `JSYNC` writer - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
pub type JSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, JSYNC_A, O>;
impl<'a, const O: u8> JSYNC_W<'a, O> {
    #[doc = "Do not launch an injected conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JSYNC_A::Disabled)
    }
    #[doc = "Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JSYNC_A::Enabled)
    }
}
#[doc = "Field `JSCAN` reader - Scanning conversion mode for injected conversions"]
pub type JSCAN_R = crate::BitReader<JSCAN_A>;
#[doc = "Scanning conversion mode for injected conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSCAN_A {
    #[doc = "0: One channel conversion is performed from the injected channel group and next the selected channel from this group is selected"]
    Single = 0,
    #[doc = "1: The series of conversions for the injected group channels is executed, starting over with the lowest selected channel"]
    Series = 1,
}
impl From<JSCAN_A> for bool {
    #[inline(always)]
    fn from(variant: JSCAN_A) -> Self {
        variant as u8 != 0
    }
}
impl JSCAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSCAN_A {
        match self.bits {
            false => JSCAN_A::Single,
            true => JSCAN_A::Series,
        }
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == JSCAN_A::Single
    }
    #[doc = "Checks if the value of the field is `Series`"]
    #[inline(always)]
    pub fn is_series(&self) -> bool {
        *self == JSCAN_A::Series
    }
}
#[doc = "Field `JSCAN` writer - Scanning conversion mode for injected conversions"]
pub type JSCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, JSCAN_A, O>;
impl<'a, const O: u8> JSCAN_W<'a, O> {
    #[doc = "One channel conversion is performed from the injected channel group and next the selected channel from this group is selected"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(JSCAN_A::Single)
    }
    #[doc = "The series of conversions for the injected group channels is executed, starting over with the lowest selected channel"]
    #[inline(always)]
    pub fn series(self) -> &'a mut W {
        self.variant(JSCAN_A::Series)
    }
}
#[doc = "Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group"]
pub type JDMAEN_R = crate::BitReader<JDMAEN_A>;
#[doc = "DMA channel enabled to read data for the injected channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDMAEN_A {
    #[doc = "0: The DMA channel is not enabled to read injected data"]
    Disabled = 0,
    #[doc = "1: The DMA channel is enabled to read injected data"]
    Enabled = 1,
}
impl From<JDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl JDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JDMAEN_A {
        match self.bits {
            false => JDMAEN_A::Disabled,
            true => JDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDMAEN_A::Enabled
    }
}
#[doc = "Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group"]
pub type JDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, JDMAEN_A, O>;
impl<'a, const O: u8> JDMAEN_W<'a, O> {
    #[doc = "The DMA channel is not enabled to read injected data"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDMAEN_A::Disabled)
    }
    #[doc = "The DMA channel is enabled to read injected data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDMAEN_A::Enabled)
    }
}
#[doc = "Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_R = crate::FieldReader<u8, JEXTEN_A>;
#[doc = "Trigger enable and trigger edge selection for injected conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN_A {
    #[doc = "0: Trigger detection is disabled"]
    Disabled = 0,
    #[doc = "1: Each rising edge on the selected trigger makes a request to launch an injected conversion"]
    RisingEdge = 1,
    #[doc = "2: Each falling edge on the selected trigger makes a request to launch an injected conversion"]
    FallingEdge = 2,
    #[doc = "3: Both rising edges and falling edges on the selected trigger make requests to launch injected conversions"]
    BothEdges = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
impl JEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::Disabled,
            1 => JEXTEN_A::RisingEdge,
            2 => JEXTEN_A::FallingEdge,
            3 => JEXTEN_A::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `BothEdges`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN_A::BothEdges
    }
}
#[doc = "Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, JEXTEN_A, 2, O>;
impl<'a, const O: u8> JEXTEN_W<'a, O> {
    #[doc = "Trigger detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTEN_A::Disabled)
    }
    #[doc = "Each rising edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::RisingEdge)
    }
    #[doc = "Each falling edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::FallingEdge)
    }
    #[doc = "Both rising edges and falling edges on the selected trigger make requests to launch injected conversions"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(JEXTEN_A::BothEdges)
    }
}
#[doc = "Field `RSWSTART` reader - Software start of a conversion on the regular channel"]
pub type RSWSTART_R = crate::BitReader<RSWSTARTW_A>;
#[doc = "Software start of a conversion on the regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSWSTARTW_A {
    #[doc = "1: Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1"]
    Start = 1,
}
impl From<RSWSTARTW_A> for bool {
    #[inline(always)]
    fn from(variant: RSWSTARTW_A) -> Self {
        variant as u8 != 0
    }
}
impl RSWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSWSTARTW_A> {
        match self.bits {
            true => Some(RSWSTARTW_A::Start),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RSWSTARTW_A::Start
    }
}
#[doc = "Field `RSWSTART` writer - Software start of a conversion on the regular channel"]
pub type RSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RSWSTARTW_A, O>;
impl<'a, const O: u8> RSWSTART_W<'a, O> {
    #[doc = "Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RSWSTARTW_A::Start)
    }
}
#[doc = "Field `RCONT` reader - Continuous mode selection for regular conversions"]
pub type RCONT_R = crate::BitReader<RCONT_A>;
#[doc = "Continuous mode selection for regular conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCONT_A {
    #[doc = "0: The regular channel is converted just once for each conversion request"]
    Once = 0,
    #[doc = "1: The regular channel is converted repeatedly after each conversion request"]
    Continuous = 1,
}
impl From<RCONT_A> for bool {
    #[inline(always)]
    fn from(variant: RCONT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCONT_A {
        match self.bits {
            false => RCONT_A::Once,
            true => RCONT_A::Continuous,
        }
    }
    #[doc = "Checks if the value of the field is `Once`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == RCONT_A::Once
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == RCONT_A::Continuous
    }
}
#[doc = "Field `RCONT` writer - Continuous mode selection for regular conversions"]
pub type RCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RCONT_A, O>;
impl<'a, const O: u8> RCONT_W<'a, O> {
    #[doc = "The regular channel is converted just once for each conversion request"]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(RCONT_A::Once)
    }
    #[doc = "The regular channel is converted repeatedly after each conversion request"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(RCONT_A::Continuous)
    }
}
#[doc = "Field `RSYNC` reader - Launch regular conversion synchronously with DFSDM0"]
pub type RSYNC_R = crate::BitReader<RSYNC_A>;
#[doc = "Launch regular conversion synchronously with DFSDM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSYNC_A {
    #[doc = "0: Do not launch a regular conversion synchronously with DFSDM_FLT0"]
    NoLaunch = 0,
    #[doc = "1: Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0"]
    Launch = 1,
}
impl From<RSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl RSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSYNC_A {
        match self.bits {
            false => RSYNC_A::NoLaunch,
            true => RSYNC_A::Launch,
        }
    }
    #[doc = "Checks if the value of the field is `NoLaunch`"]
    #[inline(always)]
    pub fn is_no_launch(&self) -> bool {
        *self == RSYNC_A::NoLaunch
    }
    #[doc = "Checks if the value of the field is `Launch`"]
    #[inline(always)]
    pub fn is_launch(&self) -> bool {
        *self == RSYNC_A::Launch
    }
}
#[doc = "Field `RSYNC` writer - Launch regular conversion synchronously with DFSDM0"]
pub type RSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RSYNC_A, O>;
impl<'a, const O: u8> RSYNC_W<'a, O> {
    #[doc = "Do not launch a regular conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn no_launch(self) -> &'a mut W {
        self.variant(RSYNC_A::NoLaunch)
    }
    #[doc = "Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0"]
    #[inline(always)]
    pub fn launch(self) -> &'a mut W {
        self.variant(RSYNC_A::Launch)
    }
}
#[doc = "Field `RDMAEN` reader - DMA channel enabled to read data for the regular conversion"]
pub type RDMAEN_R = crate::BitReader<RDMAEN_A>;
#[doc = "DMA channel enabled to read data for the regular conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDMAEN_A {
    #[doc = "0: The DMA channel is not enabled to read regular data"]
    Disabled = 0,
    #[doc = "1: The DMA channel is enabled to read regular data"]
    Enabled = 1,
}
impl From<RDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAEN_A {
        match self.bits {
            false => RDMAEN_A::Disabled,
            true => RDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDMAEN_A::Enabled
    }
}
#[doc = "Field `RDMAEN` writer - DMA channel enabled to read data for the regular conversion"]
pub type RDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RDMAEN_A, O>;
impl<'a, const O: u8> RDMAEN_W<'a, O> {
    #[doc = "The DMA channel is not enabled to read regular data"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDMAEN_A::Disabled)
    }
    #[doc = "The DMA channel is enabled to read regular data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDMAEN_A::Enabled)
    }
}
#[doc = "Field `RCH` reader - Regular channel selection"]
pub type RCH_R = crate::FieldReader<u8, RCH_A>;
#[doc = "Regular channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCH_A {
    #[doc = "0: Channel 0 is selected as regular channel"]
    Channel0 = 0,
    #[doc = "1: Channel 1 is selected as regular channel"]
    Channel1 = 1,
    #[doc = "2: Channel 2 is selected as regular channel"]
    Channel2 = 2,
    #[doc = "3: Channel 3 is selected as regular channel"]
    Channel3 = 3,
    #[doc = "4: Channel 4 is selected as regular channel"]
    Channel4 = 4,
    #[doc = "5: Channel 5 is selected as regular channel"]
    Channel5 = 5,
    #[doc = "6: Channel 6 is selected as regular channel"]
    Channel6 = 6,
    #[doc = "7: Channel 7 is selected as regular channel"]
    Channel7 = 7,
}
impl From<RCH_A> for u8 {
    #[inline(always)]
    fn from(variant: RCH_A) -> Self {
        variant as _
    }
}
impl RCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCH_A {
        match self.bits {
            0 => RCH_A::Channel0,
            1 => RCH_A::Channel1,
            2 => RCH_A::Channel2,
            3 => RCH_A::Channel3,
            4 => RCH_A::Channel4,
            5 => RCH_A::Channel5,
            6 => RCH_A::Channel6,
            7 => RCH_A::Channel7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Channel0`"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == RCH_A::Channel0
    }
    #[doc = "Checks if the value of the field is `Channel1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == RCH_A::Channel1
    }
    #[doc = "Checks if the value of the field is `Channel2`"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == RCH_A::Channel2
    }
    #[doc = "Checks if the value of the field is `Channel3`"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == RCH_A::Channel3
    }
    #[doc = "Checks if the value of the field is `Channel4`"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == RCH_A::Channel4
    }
    #[doc = "Checks if the value of the field is `Channel5`"]
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        *self == RCH_A::Channel5
    }
    #[doc = "Checks if the value of the field is `Channel6`"]
    #[inline(always)]
    pub fn is_channel6(&self) -> bool {
        *self == RCH_A::Channel6
    }
    #[doc = "Checks if the value of the field is `Channel7`"]
    #[inline(always)]
    pub fn is_channel7(&self) -> bool {
        *self == RCH_A::Channel7
    }
}
#[doc = "Field `RCH` writer - Regular channel selection"]
pub type RCH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, RCH_A, 3, O>;
impl<'a, const O: u8> RCH_W<'a, O> {
    #[doc = "Channel 0 is selected as regular channel"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(RCH_A::Channel0)
    }
    #[doc = "Channel 1 is selected as regular channel"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(RCH_A::Channel1)
    }
    #[doc = "Channel 2 is selected as regular channel"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut W {
        self.variant(RCH_A::Channel2)
    }
    #[doc = "Channel 3 is selected as regular channel"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut W {
        self.variant(RCH_A::Channel3)
    }
    #[doc = "Channel 4 is selected as regular channel"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut W {
        self.variant(RCH_A::Channel4)
    }
    #[doc = "Channel 5 is selected as regular channel"]
    #[inline(always)]
    pub fn channel5(self) -> &'a mut W {
        self.variant(RCH_A::Channel5)
    }
    #[doc = "Channel 6 is selected as regular channel"]
    #[inline(always)]
    pub fn channel6(self) -> &'a mut W {
        self.variant(RCH_A::Channel6)
    }
    #[doc = "Channel 7 is selected as regular channel"]
    #[inline(always)]
    pub fn channel7(self) -> &'a mut W {
        self.variant(RCH_A::Channel7)
    }
}
#[doc = "Field `FAST` reader - Fast conversion mode selection for regular conversions"]
pub type FAST_R = crate::BitReader<FAST_A>;
#[doc = "Fast conversion mode selection for regular conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAST_A {
    #[doc = "0: Fast conversion mode disabled"]
    Disabled = 0,
    #[doc = "1: Fast conversion mode enabled"]
    Enabled = 1,
}
impl From<FAST_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_A) -> Self {
        variant as u8 != 0
    }
}
impl FAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_A {
        match self.bits {
            false => FAST_A::Disabled,
            true => FAST_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAST_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FAST_A::Enabled
    }
}
#[doc = "Field `FAST` writer - Fast conversion mode selection for regular conversions"]
pub type FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, FAST_A, O>;
impl<'a, const O: u8> FAST_W<'a, O> {
    #[doc = "Fast conversion mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FAST_A::Disabled)
    }
    #[doc = "Fast conversion mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FAST_A::Enabled)
    }
}
#[doc = "Field `AWFSEL` reader - Analog watchdog fast mode select"]
pub type AWFSEL_R = crate::BitReader<AWFSEL_A>;
#[doc = "Analog watchdog fast mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWFSEL_A {
    #[doc = "0: Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift"]
    Output = 0,
    #[doc = "1: Analog watchdog on channel transceivers value (after watchdog filter)"]
    Transceiver = 1,
}
impl From<AWFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: AWFSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl AWFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWFSEL_A {
        match self.bits {
            false => AWFSEL_A::Output,
            true => AWFSEL_A::Transceiver,
        }
    }
    #[doc = "Checks if the value of the field is `Output`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == AWFSEL_A::Output
    }
    #[doc = "Checks if the value of the field is `Transceiver`"]
    #[inline(always)]
    pub fn is_transceiver(&self) -> bool {
        *self == AWFSEL_A::Transceiver
    }
}
#[doc = "Field `AWFSEL` writer - Analog watchdog fast mode select"]
pub type AWFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, AWFSEL_A, O>;
impl<'a, const O: u8> AWFSEL_W<'a, O> {
    #[doc = "Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(AWFSEL_A::Output)
    }
    #[doc = "Analog watchdog on channel transceivers value (after watchdog filter)"]
    #[inline(always)]
    pub fn transceiver(self) -> &'a mut W {
        self.variant(AWFSEL_A::Transceiver)
    }
}
impl R {
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfen(&mut self) -> DFEN_W<0> {
        DFEN_W::new(self)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<1> {
        JSWSTART_W::new(self)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    #[must_use]
    pub fn jsync(&mut self) -> JSYNC_W<3> {
        JSYNC_W::new(self)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jscan(&mut self) -> JSCAN_W<4> {
        JSCAN_W::new(self)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    #[must_use]
    pub fn jdmaen(&mut self) -> JDMAEN_W<5> {
        JDMAEN_W::new(self)
    }
    #[doc = "Bits 8:10 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<8> {
        JEXTSEL_W::new(self)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<13> {
        JEXTEN_W::new(self)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn rswstart(&mut self) -> RSWSTART_W<17> {
        RSWSTART_W::new(self)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<18> {
        RCONT_W::new(self)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    #[must_use]
    pub fn rsync(&mut self) -> RSYNC_W<19> {
        RSYNC_W::new(self)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    #[must_use]
    pub fn rdmaen(&mut self) -> RDMAEN_W<21> {
        RDMAEN_W::new(self)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn rch(&mut self) -> RCH_W<24> {
        RCH_W::new(self)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<29> {
        FAST_W::new(self)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    #[must_use]
    pub fn awfsel(&mut self) -> AWFSEL_W<30> {
        AWFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
