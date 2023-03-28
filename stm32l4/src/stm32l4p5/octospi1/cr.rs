#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: OCTOSPI disabled"]
    Disabled = 0,
    #[doc = "1: OCTOSPI enabled"]
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "OCTOSPI disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    #[doc = "OCTOSPI enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
#[doc = "Field `ABORT` reader - Abort request"]
pub type ABORT_R = crate::BitReader<ABORT_A>;
#[doc = "Abort request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_A {
    #[doc = "0: No abort requested"]
    NotRequested = 0,
    #[doc = "1: Abort requested"]
    Requested = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABORT_A {
        match self.bits {
            false => ABORT_A::NotRequested,
            true => ABORT_A::Requested,
        }
    }
    #[doc = "Checks if the value of the field is `NotRequested`"]
    #[inline(always)]
    pub fn is_not_requested(&self) -> bool {
        *self == ABORT_A::NotRequested
    }
    #[doc = "Checks if the value of the field is `Requested`"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == ABORT_A::Requested
    }
}
#[doc = "Field `ABORT` writer - Abort request"]
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ABORT_A, O>;
impl<'a, const O: u8> ABORT_W<'a, O> {
    #[doc = "No abort requested"]
    #[inline(always)]
    pub fn not_requested(self) -> &'a mut W {
        self.variant(ABORT_A::NotRequested)
    }
    #[doc = "Abort requested"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(ABORT_A::Requested)
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled for Indirect mode"]
    Disabled = 0,
    #[doc = "1: DMA enabled for Indirect mode"]
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA disabled for Indirect mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    #[doc = "DMA enabled for Indirect mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
#[doc = "Field `TCEN` reader - Timeout counter enable"]
pub type TCEN_R = crate::BitReader<TCEN_A>;
#[doc = "Timeout counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCEN_A {
    #[doc = "0: Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode"]
    Disabled = 0,
    #[doc = "1: Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\\[15:0\\]
cycles of external device inactivity"]
    Enabled = 1,
}
impl From<TCEN_A> for bool {
    #[inline(always)]
    fn from(variant: TCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCEN_A {
        match self.bits {
            false => TCEN_A::Disabled,
            true => TCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCEN_A::Enabled
    }
}
#[doc = "Field `TCEN` writer - Timeout counter enable"]
pub type TCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TCEN_A, O>;
impl<'a, const O: u8> TCEN_W<'a, O> {
    #[doc = "Timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely after an access in Memory-mapped mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCEN_A::Disabled)
    }
    #[doc = "Timeout counter is enabled, and thus the chip-select is released in the Memory-mapped mode after TIMEOUT\\[15:0\\]
cycles of external device inactivity"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCEN_A::Enabled)
    }
}
#[doc = "Field `DMM` reader - Dual-memory configuration"]
pub type DMM_R = crate::BitReader<DMM_A>;
#[doc = "Dual-memory configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMM_A {
    #[doc = "0: Dual-quad configuration disabled"]
    Disabled = 0,
    #[doc = "1: Dual-quad configuration enabled"]
    Enabled = 1,
}
impl From<DMM_A> for bool {
    #[inline(always)]
    fn from(variant: DMM_A) -> Self {
        variant as u8 != 0
    }
}
impl DMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMM_A {
        match self.bits {
            false => DMM_A::Disabled,
            true => DMM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMM_A::Enabled
    }
}
#[doc = "Field `DMM` writer - Dual-memory configuration"]
pub type DMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMM_A, O>;
impl<'a, const O: u8> DMM_W<'a, O> {
    #[doc = "Dual-quad configuration disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMM_A::Disabled)
    }
    #[doc = "Dual-quad configuration enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMM_A::Enabled)
    }
}
#[doc = "Field `FSEL` reader - FLASH memory selection"]
pub type FSEL_R = crate::BitReader<FSEL_A>;
#[doc = "FLASH memory selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSEL_A {
    #[doc = "0: FLASH 1 selected (data exchanged over IO\\[3:0\\])"]
    Flash1 = 0,
    #[doc = "1: FLASH 2 selected (data exchanged over IO\\[7:4\\])"]
    Flash2 = 1,
}
impl From<FSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL_A {
        match self.bits {
            false => FSEL_A::Flash1,
            true => FSEL_A::Flash2,
        }
    }
    #[doc = "Checks if the value of the field is `Flash1`"]
    #[inline(always)]
    pub fn is_flash1(&self) -> bool {
        *self == FSEL_A::Flash1
    }
    #[doc = "Checks if the value of the field is `Flash2`"]
    #[inline(always)]
    pub fn is_flash2(&self) -> bool {
        *self == FSEL_A::Flash2
    }
}
#[doc = "Field `FSEL` writer - FLASH memory selection"]
pub type FSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FSEL_A, O>;
impl<'a, const O: u8> FSEL_W<'a, O> {
    #[doc = "FLASH 1 selected (data exchanged over IO\\[3:0\\])"]
    #[inline(always)]
    pub fn flash1(self) -> &'a mut W {
        self.variant(FSEL_A::Flash1)
    }
    #[doc = "FLASH 2 selected (data exchanged over IO\\[7:4\\])"]
    #[inline(always)]
    pub fn flash2(self) -> &'a mut W {
        self.variant(FSEL_A::Flash2)
    }
}
#[doc = "Field `FTHRES` reader - IFO threshold level"]
pub type FTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTHRES` writer - IFO threshold level"]
pub type FTHRES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, u8, 5, O>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader<TEIE_A>;
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::Disabled,
            true => TEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE_A::Enabled
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TEIE_A, O>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::Enabled)
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::Disabled,
            true => TCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
#[doc = "Field `FTIE` reader - FIFO threshold interrupt enable"]
pub type FTIE_R = crate::BitReader<FTIE_A>;
#[doc = "FIFO threshold interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<FTIE_A> for bool {
    #[inline(always)]
    fn from(variant: FTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTIE_A {
        match self.bits {
            false => FTIE_A::Disabled,
            true => FTIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FTIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FTIE_A::Enabled
    }
}
#[doc = "Field `FTIE` writer - FIFO threshold interrupt enable"]
pub type FTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FTIE_A, O>;
impl<'a, const O: u8> FTIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FTIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FTIE_A::Enabled)
    }
}
#[doc = "Field `SMIE` reader - Status match interrupt enable"]
pub type SMIE_R = crate::BitReader<SMIE_A>;
#[doc = "Status match interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<SMIE_A> for bool {
    #[inline(always)]
    fn from(variant: SMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMIE_A {
        match self.bits {
            false => SMIE_A::Disabled,
            true => SMIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMIE_A::Enabled
    }
}
#[doc = "Field `SMIE` writer - Status match interrupt enable"]
pub type SMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SMIE_A, O>;
impl<'a, const O: u8> SMIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMIE_A::Enabled)
    }
}
#[doc = "Field `TOIE` reader - TimeOut interrupt enable"]
pub type TOIE_R = crate::BitReader<TOIE_A>;
#[doc = "TimeOut interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIE_A {
        match self.bits {
            false => TOIE_A::Disabled,
            true => TOIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TOIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TOIE_A::Enabled
    }
}
#[doc = "Field `TOIE` writer - TimeOut interrupt enable"]
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TOIE_A, O>;
impl<'a, const O: u8> TOIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TOIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TOIE_A::Enabled)
    }
}
#[doc = "Field `APMS` reader - Automatic poll mode stop"]
pub type APMS_R = crate::BitReader<APMS_A>;
#[doc = "Automatic poll mode stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APMS_A {
    #[doc = "0: Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI"]
    Running = 0,
    #[doc = "1: Automatic status-polling mode stops as soon as there is a match"]
    StopMatch = 1,
}
impl From<APMS_A> for bool {
    #[inline(always)]
    fn from(variant: APMS_A) -> Self {
        variant as u8 != 0
    }
}
impl APMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APMS_A {
        match self.bits {
            false => APMS_A::Running,
            true => APMS_A::StopMatch,
        }
    }
    #[doc = "Checks if the value of the field is `Running`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == APMS_A::Running
    }
    #[doc = "Checks if the value of the field is `StopMatch`"]
    #[inline(always)]
    pub fn is_stop_match(&self) -> bool {
        *self == APMS_A::StopMatch
    }
}
#[doc = "Field `APMS` writer - Automatic poll mode stop"]
pub type APMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, APMS_A, O>;
impl<'a, const O: u8> APMS_W<'a, O> {
    #[doc = "Automatic status-polling mode is stopped only by abort or by disabling the OCTOSPI"]
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(APMS_A::Running)
    }
    #[doc = "Automatic status-polling mode stops as soon as there is a match"]
    #[inline(always)]
    pub fn stop_match(self) -> &'a mut W {
        self.variant(APMS_A::StopMatch)
    }
}
#[doc = "Field `PMM` reader - Polling match mode"]
pub type PMM_R = crate::BitReader<PMM_A>;
#[doc = "Polling match mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMM_A {
    #[doc = "0: AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register"]
    AndmatchMode = 0,
    #[doc = "1: OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register"]
    Ormatchmode = 1,
}
impl From<PMM_A> for bool {
    #[inline(always)]
    fn from(variant: PMM_A) -> Self {
        variant as u8 != 0
    }
}
impl PMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMM_A {
        match self.bits {
            false => PMM_A::AndmatchMode,
            true => PMM_A::Ormatchmode,
        }
    }
    #[doc = "Checks if the value of the field is `AndmatchMode`"]
    #[inline(always)]
    pub fn is_andmatch_mode(&self) -> bool {
        *self == PMM_A::AndmatchMode
    }
    #[doc = "Checks if the value of the field is `Ormatchmode`"]
    #[inline(always)]
    pub fn is_ormatchmode(&self) -> bool {
        *self == PMM_A::Ormatchmode
    }
}
#[doc = "Field `PMM` writer - Polling match mode"]
pub type PMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PMM_A, O>;
impl<'a, const O: u8> PMM_W<'a, O> {
    #[doc = "AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register"]
    #[inline(always)]
    pub fn andmatch_mode(self) -> &'a mut W {
        self.variant(PMM_A::AndmatchMode)
    }
    #[doc = "OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register"]
    #[inline(always)]
    pub fn ormatchmode(self) -> &'a mut W {
        self.variant(PMM_A::Ormatchmode)
    }
}
#[doc = "Field `FMODE` reader - Functional mode"]
pub type FMODE_R = crate::FieldReader<u8, FMODE_A>;
#[doc = "Functional mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMODE_A {
    #[doc = "0: Indirect-write mode"]
    IndirectWrite = 0,
    #[doc = "1: Indirect-read mode"]
    IndirectRead = 1,
    #[doc = "2: Automatic status-polling mode"]
    AutomaticPolling = 2,
    #[doc = "3: Memory-mapped mode"]
    MemoryMapped = 3,
}
impl From<FMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FMODE_A) -> Self {
        variant as _
    }
}
impl FMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMODE_A {
        match self.bits {
            0 => FMODE_A::IndirectWrite,
            1 => FMODE_A::IndirectRead,
            2 => FMODE_A::AutomaticPolling,
            3 => FMODE_A::MemoryMapped,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IndirectWrite`"]
    #[inline(always)]
    pub fn is_indirect_write(&self) -> bool {
        *self == FMODE_A::IndirectWrite
    }
    #[doc = "Checks if the value of the field is `IndirectRead`"]
    #[inline(always)]
    pub fn is_indirect_read(&self) -> bool {
        *self == FMODE_A::IndirectRead
    }
    #[doc = "Checks if the value of the field is `AutomaticPolling`"]
    #[inline(always)]
    pub fn is_automatic_polling(&self) -> bool {
        *self == FMODE_A::AutomaticPolling
    }
    #[doc = "Checks if the value of the field is `MemoryMapped`"]
    #[inline(always)]
    pub fn is_memory_mapped(&self) -> bool {
        *self == FMODE_A::MemoryMapped
    }
}
#[doc = "Field `FMODE` writer - Functional mode"]
pub type FMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, FMODE_A, 2, O>;
impl<'a, const O: u8> FMODE_W<'a, O> {
    #[doc = "Indirect-write mode"]
    #[inline(always)]
    pub fn indirect_write(self) -> &'a mut W {
        self.variant(FMODE_A::IndirectWrite)
    }
    #[doc = "Indirect-read mode"]
    #[inline(always)]
    pub fn indirect_read(self) -> &'a mut W {
        self.variant(FMODE_A::IndirectRead)
    }
    #[doc = "Automatic status-polling mode"]
    #[inline(always)]
    pub fn automatic_polling(self) -> &'a mut W {
        self.variant(FMODE_A::AutomaticPolling)
    }
    #[doc = "Memory-mapped mode"]
    #[inline(always)]
    pub fn memory_mapped(self) -> &'a mut W {
        self.variant(FMODE_A::MemoryMapped)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort request"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout counter enable"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Dual-memory configuration"]
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLASH memory selection"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - IFO threshold level"]
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Status match interrupt enable"]
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Automatic poll mode stop"]
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Polling match mode"]
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Functional mode"]
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Abort request"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<1> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 2 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<2> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 3 - Timeout counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<3> {
        TCEN_W::new(self)
    }
    #[doc = "Bit 6 - Dual-memory configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dmm(&mut self) -> DMM_W<6> {
        DMM_W::new(self)
    }
    #[doc = "Bit 7 - FLASH memory selection"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<7> {
        FSEL_W::new(self)
    }
    #[doc = "Bits 8:12 - IFO threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn fthres(&mut self) -> FTHRES_W<8> {
        FTHRES_W::new(self)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<16> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<17> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<18> {
        FTIE_W::new(self)
    }
    #[doc = "Bit 19 - Status match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SMIE_W<19> {
        SMIE_W::new(self)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<20> {
        TOIE_W::new(self)
    }
    #[doc = "Bit 22 - Automatic poll mode stop"]
    #[inline(always)]
    #[must_use]
    pub fn apms(&mut self) -> APMS_W<22> {
        APMS_W::new(self)
    }
    #[doc = "Bit 23 - Polling match mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmm(&mut self) -> PMM_W<23> {
        PMM_W::new(self)
    }
    #[doc = "Bits 28:29 - Functional mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmode(&mut self) -> FMODE_W<28> {
        FMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
