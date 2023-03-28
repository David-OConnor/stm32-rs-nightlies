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
#[doc = "Field `START` reader - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "1: Launch the DMA2D"]
    Start = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            true => Some(START_A::Start),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::Start
    }
}
#[doc = "Field `START` writer - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Launch the DMA2D"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::Start)
    }
}
#[doc = "Field `SUSP` reader - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
pub type SUSP_R = crate::BitReader<SUSP_A>;
#[doc = "Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP_A {
    #[doc = "0: Transfer not suspended"]
    NotSuspended = 0,
    #[doc = "1: Transfer suspended"]
    Suspended = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::NotSuspended,
            true => SUSP_A::Suspended,
        }
    }
    #[doc = "Checks if the value of the field is `NotSuspended`"]
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP_A::NotSuspended
    }
    #[doc = "Checks if the value of the field is `Suspended`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP_A::Suspended
    }
}
#[doc = "Field `SUSP` writer - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SUSP_A, O>;
impl<'a, const O: u8> SUSP_W<'a, O> {
    #[doc = "Transfer not suspended"]
    #[inline(always)]
    pub fn not_suspended(self) -> &'a mut W {
        self.variant(SUSP_A::NotSuspended)
    }
    #[doc = "Transfer suspended"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut W {
        self.variant(SUSP_A::Suspended)
    }
}
#[doc = "Field `ABORT` reader - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
pub type ABORT_R = crate::BitReader<ABORT_A>;
#[doc = "Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_A {
    #[doc = "1: Transfer abort requested"]
    AbortRequest = 1,
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
    pub fn variant(&self) -> Option<ABORT_A> {
        match self.bits {
            true => Some(ABORT_A::AbortRequest),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AbortRequest`"]
    #[inline(always)]
    pub fn is_abort_request(&self) -> bool {
        *self == ABORT_A::AbortRequest
    }
}
#[doc = "Field `ABORT` writer - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ABORT_A, O>;
impl<'a, const O: u8> ABORT_W<'a, O> {
    #[doc = "Transfer abort requested"]
    #[inline(always)]
    pub fn abort_request(self) -> &'a mut W {
        self.variant(ABORT_A::AbortRequest)
    }
}
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TEIE_R = crate::BitReader<TEIE_A>;
#[doc = "Transfer error interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    #[doc = "0: TE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TE interrupt enabled"]
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
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TEIE_A, O>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    #[doc = "TE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::Disabled)
    }
    #[doc = "TE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::Enabled)
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transfer complete interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: TC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TC interrupt enabled"]
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
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
#[doc = "Field `TWIE` reader - Transfer watermark interrupt enable This bit is set and cleared by software."]
pub type TWIE_R = crate::BitReader<TWIE_A>;
#[doc = "Transfer watermark interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWIE_A {
    #[doc = "0: TW interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TW interrupt enabled"]
    Enabled = 1,
}
impl From<TWIE_A> for bool {
    #[inline(always)]
    fn from(variant: TWIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWIE_A {
        match self.bits {
            false => TWIE_A::Disabled,
            true => TWIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TWIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TWIE_A::Enabled
    }
}
#[doc = "Field `TWIE` writer - Transfer watermark interrupt enable This bit is set and cleared by software."]
pub type TWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TWIE_A, O>;
impl<'a, const O: u8> TWIE_W<'a, O> {
    #[doc = "TW interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TWIE_A::Disabled)
    }
    #[doc = "TW interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TWIE_A::Enabled)
    }
}
#[doc = "Field `CAEIE` reader - CLUT access error interrupt enable This bit is set and cleared by software."]
pub type CAEIE_R = crate::BitReader<CAEIE_A>;
#[doc = "CLUT access error interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAEIE_A {
    #[doc = "0: CAE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CAE interrupt enabled"]
    Enabled = 1,
}
impl From<CAEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CAEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAEIE_A {
        match self.bits {
            false => CAEIE_A::Disabled,
            true => CAEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAEIE_A::Enabled
    }
}
#[doc = "Field `CAEIE` writer - CLUT access error interrupt enable This bit is set and cleared by software."]
pub type CAEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CAEIE_A, O>;
impl<'a, const O: u8> CAEIE_W<'a, O> {
    #[doc = "CAE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAEIE_A::Disabled)
    }
    #[doc = "CAE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAEIE_A::Enabled)
    }
}
#[doc = "Field `CTCIE` reader - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
pub type CTCIE_R = crate::BitReader<CTCIE_A>;
#[doc = "CLUT transfer complete interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIE_A {
    #[doc = "0: CTC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CTC interrupt enabled"]
    Enabled = 1,
}
impl From<CTCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIE_A {
        match self.bits {
            false => CTCIE_A::Disabled,
            true => CTCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTCIE_A::Enabled
    }
}
#[doc = "Field `CTCIE` writer - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
pub type CTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CTCIE_A, O>;
impl<'a, const O: u8> CTCIE_W<'a, O> {
    #[doc = "CTC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTCIE_A::Disabled)
    }
    #[doc = "CTC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTCIE_A::Enabled)
    }
}
#[doc = "Field `CEIE` reader - Configuration Error Interrupt Enable This bit is set and cleared by software."]
pub type CEIE_R = crate::BitReader<CEIE_A>;
#[doc = "Configuration Error Interrupt Enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIE_A {
    #[doc = "0: CE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CE interrupt enabled"]
    Enabled = 1,
}
impl From<CEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIE_A {
        match self.bits {
            false => CEIE_A::Disabled,
            true => CEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEIE_A::Enabled
    }
}
#[doc = "Field `CEIE` writer - Configuration Error Interrupt Enable This bit is set and cleared by software."]
pub type CEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CEIE_A, O>;
impl<'a, const O: u8> CEIE_W<'a, O> {
    #[doc = "CE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEIE_A::Disabled)
    }
    #[doc = "CE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEIE_A::Enabled)
    }
}
#[doc = "Field `MODE` reader - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Memory-to-memory (FG fetch only)"]
    MemoryToMemory = 0,
    #[doc = "1: Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
    MemoryToMemoryPfc = 1,
    #[doc = "2: Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
    MemoryToMemoryPfcblending = 2,
    #[doc = "3: Register-to-memory"]
    RegisterToMemory = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::MemoryToMemory,
            1 => MODE_A::MemoryToMemoryPfc,
            2 => MODE_A::MemoryToMemoryPfcblending,
            3 => MODE_A::RegisterToMemory,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MemoryToMemory`"]
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == MODE_A::MemoryToMemory
    }
    #[doc = "Checks if the value of the field is `MemoryToMemoryPfc`"]
    #[inline(always)]
    pub fn is_memory_to_memory_pfc(&self) -> bool {
        *self == MODE_A::MemoryToMemoryPfc
    }
    #[doc = "Checks if the value of the field is `MemoryToMemoryPfcblending`"]
    #[inline(always)]
    pub fn is_memory_to_memory_pfcblending(&self) -> bool {
        *self == MODE_A::MemoryToMemoryPfcblending
    }
    #[doc = "Checks if the value of the field is `RegisterToMemory`"]
    #[inline(always)]
    pub fn is_register_to_memory(&self) -> bool {
        *self == MODE_A::RegisterToMemory
    }
}
#[doc = "Field `MODE` writer - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Memory-to-memory (FG fetch only)"]
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut W {
        self.variant(MODE_A::MemoryToMemory)
    }
    #[doc = "Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
    #[inline(always)]
    pub fn memory_to_memory_pfc(self) -> &'a mut W {
        self.variant(MODE_A::MemoryToMemoryPfc)
    }
    #[doc = "Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
    #[inline(always)]
    pub fn memory_to_memory_pfcblending(self) -> &'a mut W {
        self.variant(MODE_A::MemoryToMemoryPfcblending)
    }
    #[doc = "Register-to-memory"]
    #[inline(always)]
    pub fn register_to_memory(self) -> &'a mut W {
        self.variant(MODE_A::RegisterToMemory)
    }
}
impl R {
    #[doc = "Bit 0 - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<1> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<2> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<8> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<9> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn twie(&mut self) -> TWIE_W<10> {
        TWIE_W::new(self)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn caeie(&mut self) -> CAEIE_W<11> {
        CAEIE_W::new(self)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ctcie(&mut self) -> CTCIE_W<12> {
        CTCIE_W::new(self)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ceie(&mut self) -> CEIE_W<13> {
        CEIE_W::new(self)
    }
    #[doc = "Bits 16:17 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<16> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA2D control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
