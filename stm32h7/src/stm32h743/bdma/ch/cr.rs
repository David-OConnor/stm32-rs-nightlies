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
#[doc = "Field `EN` reader - Channel enable This bit is set and cleared by software."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Channel enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Channel enabled"]
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
#[doc = "Field `EN` writer - Channel enable This bit is set and cleared by software."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
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
#[doc = "Field `HTIE` reader - Half transfer interrupt enable This bit is set and cleared by software."]
pub type HTIE_R = crate::BitReader<HTIE_A>;
#[doc = "Half transfer interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE_A {
    #[doc = "0: HT interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HT interrupt enabled"]
    Enabled = 1,
}
impl From<HTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIE_A {
        match self.bits {
            false => HTIE_A::Disabled,
            true => HTIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE_A::Enabled
    }
}
#[doc = "Field `HTIE` writer - Half transfer interrupt enable This bit is set and cleared by software."]
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HTIE_A, O>;
impl<'a, const O: u8> HTIE_W<'a, O> {
    #[doc = "HT interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTIE_A::Disabled)
    }
    #[doc = "HT interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTIE_A::Enabled)
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
#[doc = "Field `DIR` reader - Data transfer direction This bit is set and cleared by software."]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Data transfer direction This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Peripheral-to-memory"]
    PeripheralToMemory = 0,
    #[doc = "1: Memory-to-peripheral"]
    MemoryToPeripheral = 1,
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
            false => DIR_A::PeripheralToMemory,
            true => DIR_A::MemoryToPeripheral,
        }
    }
    #[doc = "Checks if the value of the field is `PeripheralToMemory`"]
    #[inline(always)]
    pub fn is_peripheral_to_memory(&self) -> bool {
        *self == DIR_A::PeripheralToMemory
    }
    #[doc = "Checks if the value of the field is `MemoryToPeripheral`"]
    #[inline(always)]
    pub fn is_memory_to_peripheral(&self) -> bool {
        *self == DIR_A::MemoryToPeripheral
    }
}
#[doc = "Field `DIR` writer - Data transfer direction This bit is set and cleared by software."]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Peripheral-to-memory"]
    #[inline(always)]
    pub fn peripheral_to_memory(self) -> &'a mut W {
        self.variant(DIR_A::PeripheralToMemory)
    }
    #[doc = "Memory-to-peripheral"]
    #[inline(always)]
    pub fn memory_to_peripheral(self) -> &'a mut W {
        self.variant(DIR_A::MemoryToPeripheral)
    }
}
#[doc = "Field `CIRC` reader - Circular mode This bit is set and cleared by software."]
pub type CIRC_R = crate::BitReader<CIRC_A>;
#[doc = "Circular mode This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC_A {
    #[doc = "0: Circular mode disabled"]
    Disabled = 0,
    #[doc = "1: Circular mode enabled"]
    Enabled = 1,
}
impl From<CIRC_A> for bool {
    #[inline(always)]
    fn from(variant: CIRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CIRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIRC_A {
        match self.bits {
            false => CIRC_A::Disabled,
            true => CIRC_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC_A::Enabled
    }
}
#[doc = "Field `CIRC` writer - Circular mode This bit is set and cleared by software."]
pub type CIRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CIRC_A, O>;
impl<'a, const O: u8> CIRC_W<'a, O> {
    #[doc = "Circular mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRC_A::Disabled)
    }
    #[doc = "Circular mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRC_A::Enabled)
    }
}
#[doc = "Field `PINC` reader - Peripheral increment mode This bit is set and cleared by software."]
pub type PINC_R = crate::BitReader<PINC_A>;
#[doc = "Peripheral increment mode This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC_A {
    #[doc = "0: Address pointer is fixed"]
    Fixed = 0,
    #[doc = "1: Address pointer is incremented after each data transfer"]
    Incremented = 1,
}
impl From<PINC_A> for bool {
    #[inline(always)]
    fn from(variant: PINC_A) -> Self {
        variant as u8 != 0
    }
}
impl PINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINC_A {
        match self.bits {
            false => PINC_A::Fixed,
            true => PINC_A::Incremented,
        }
    }
    #[doc = "Checks if the value of the field is `Fixed`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == PINC_A::Fixed
    }
    #[doc = "Checks if the value of the field is `Incremented`"]
    #[inline(always)]
    pub fn is_incremented(&self) -> bool {
        *self == PINC_A::Incremented
    }
}
#[doc = "Field `PINC` writer - Peripheral increment mode This bit is set and cleared by software."]
pub type PINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PINC_A, O>;
impl<'a, const O: u8> PINC_W<'a, O> {
    #[doc = "Address pointer is fixed"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(PINC_A::Fixed)
    }
    #[doc = "Address pointer is incremented after each data transfer"]
    #[inline(always)]
    pub fn incremented(self) -> &'a mut W {
        self.variant(PINC_A::Incremented)
    }
}
#[doc = "Field `MINC` reader - Memory increment mode This bit is set and cleared by software."]
pub use PINC_R as MINC_R;
#[doc = "Field `MINC` writer - Memory increment mode This bit is set and cleared by software."]
pub use PINC_W as MINC_W;
#[doc = "Field `PSIZE` reader - Peripheral size These bits are set and cleared by software."]
pub type PSIZE_R = crate::FieldReader<u8, PSIZE_A>;
#[doc = "Peripheral size These bits are set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE_A {
    #[doc = "0: Byte (8-bit)"]
    Bits8 = 0,
    #[doc = "1: Half-word (16-bit)"]
    Bits16 = 1,
    #[doc = "2: Word (32-bit)"]
    Bits32 = 2,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSIZE_A> {
        match self.bits {
            0 => Some(PSIZE_A::Bits8),
            1 => Some(PSIZE_A::Bits16),
            2 => Some(PSIZE_A::Bits32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE_A::Bits32
    }
}
#[doc = "Field `PSIZE` writer - Peripheral size These bits are set and cleared by software."]
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, PSIZE_A, 2, O>;
impl<'a, const O: u8> PSIZE_W<'a, O> {
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits8)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits16)
    }
    #[doc = "Word (32-bit)"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits32)
    }
}
#[doc = "Field `MSIZE` reader - Memory size These bits are set and cleared by software."]
pub use PSIZE_R as MSIZE_R;
#[doc = "Field `MSIZE` writer - Memory size These bits are set and cleared by software."]
pub use PSIZE_W as MSIZE_W;
#[doc = "Field `PL` reader - Channel priority level These bits are set and cleared by software."]
pub type PL_R = crate::FieldReader<u8, PL_A>;
#[doc = "Channel priority level These bits are set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL_A {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: Medium"]
    Medium = 1,
    #[doc = "2: High"]
    High = 2,
    #[doc = "3: Very high"]
    VeryHigh = 3,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
impl PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PL_A {
        match self.bits {
            0 => PL_A::Low,
            1 => PL_A::Medium,
            2 => PL_A::High,
            3 => PL_A::VeryHigh,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PL_A::Low
    }
    #[doc = "Checks if the value of the field is `Medium`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PL_A::Medium
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PL_A::High
    }
    #[doc = "Checks if the value of the field is `VeryHigh`"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PL_A::VeryHigh
    }
}
#[doc = "Field `PL` writer - Channel priority level These bits are set and cleared by software."]
pub type PL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, PL_A, 2, O>;
impl<'a, const O: u8> PL_W<'a, O> {
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PL_A::Low)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(PL_A::Medium)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PL_A::High)
    }
    #[doc = "Very high"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PL_A::VeryHigh)
    }
}
#[doc = "Field `MEM2MEM` reader - Memory to memory mode This bit is set and cleared by software."]
pub type MEM2MEM_R = crate::BitReader<MEM2MEM_A>;
#[doc = "Memory to memory mode This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM_A {
    #[doc = "0: Memory-to-memory mode disabled"]
    Disabled = 0,
    #[doc = "1: Memory-to-memory mode enabled"]
    Enabled = 1,
}
impl From<MEM2MEM_A> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM2MEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM2MEM_A {
        match self.bits {
            false => MEM2MEM_A::Disabled,
            true => MEM2MEM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEM2MEM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEM2MEM_A::Enabled
    }
}
#[doc = "Field `MEM2MEM` writer - Memory to memory mode This bit is set and cleared by software."]
pub type MEM2MEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MEM2MEM_A, O>;
impl<'a, const O: u8> MEM2MEM_W<'a, O> {
    #[doc = "Memory-to-memory mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MEM2MEM_A::Disabled)
    }
    #[doc = "Memory-to-memory mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MEM2MEM_A::Enabled)
    }
}
#[doc = "Field `DBM` reader - Double-buffer mode"]
pub type DBM_R = crate::BitReader<DBM_A>;
#[doc = "Double-buffer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBM_A {
    #[doc = "0: No buffer switching at the end of transfer"]
    Disabled = 0,
    #[doc = "1: Memory target switched at the end of the DMA transfer"]
    Enabled = 1,
}
impl From<DBM_A> for bool {
    #[inline(always)]
    fn from(variant: DBM_A) -> Self {
        variant as u8 != 0
    }
}
impl DBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBM_A {
        match self.bits {
            false => DBM_A::Disabled,
            true => DBM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBM_A::Enabled
    }
}
#[doc = "Field `DBM` writer - Double-buffer mode"]
pub type DBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBM_A, O>;
impl<'a, const O: u8> DBM_W<'a, O> {
    #[doc = "No buffer switching at the end of transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBM_A::Disabled)
    }
    #[doc = "Memory target switched at the end of the DMA transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBM_A::Enabled)
    }
}
#[doc = "Field `CT` reader - Current target memory in double-buffer mode"]
pub type CT_R = crate::BitReader<CT_A>;
#[doc = "Current target memory in double-buffer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT_A {
    #[doc = "0: The current target memory is Memory 0"]
    Memory0 = 0,
    #[doc = "1: The current target memory is Memory 1"]
    Memory1 = 1,
}
impl From<CT_A> for bool {
    #[inline(always)]
    fn from(variant: CT_A) -> Self {
        variant as u8 != 0
    }
}
impl CT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT_A {
        match self.bits {
            false => CT_A::Memory0,
            true => CT_A::Memory1,
        }
    }
    #[doc = "Checks if the value of the field is `Memory0`"]
    #[inline(always)]
    pub fn is_memory0(&self) -> bool {
        *self == CT_A::Memory0
    }
    #[doc = "Checks if the value of the field is `Memory1`"]
    #[inline(always)]
    pub fn is_memory1(&self) -> bool {
        *self == CT_A::Memory1
    }
}
#[doc = "Field `CT` writer - Current target memory in double-buffer mode"]
pub type CT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CT_A, O>;
impl<'a, const O: u8> CT_W<'a, O> {
    #[doc = "The current target memory is Memory 0"]
    #[inline(always)]
    pub fn memory0(self) -> &'a mut W {
        self.variant(CT_A::Memory0)
    }
    #[doc = "The current target memory is Memory 1"]
    #[inline(always)]
    pub fn memory1(self) -> &'a mut W {
        self.variant(CT_A::Memory1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral size These bits are set and cleared by software."]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory size These bits are set and cleared by software."]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel priority level These bits are set and cleared by software."]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Double-buffer mode"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Current target memory in double-buffer mode"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<1> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<2> {
        HTIE_W::new(self)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<3> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 4 - Data transfer direction This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - Circular mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<5> {
        CIRC_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral increment mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<6> {
        PINC_W::new(self)
    }
    #[doc = "Bit 7 - Memory increment mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<7> {
        MINC_W::new(self)
    }
    #[doc = "Bits 8:9 - Peripheral size These bits are set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<8> {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 10:11 - Memory size These bits are set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<10> {
        MSIZE_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel priority level These bits are set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<12> {
        PL_W::new(self)
    }
    #[doc = "Bit 14 - Memory to memory mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<14> {
        MEM2MEM_W::new(self)
    }
    #[doc = "Bit 15 - Double-buffer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbm(&mut self) -> DBM_W<15> {
        DBM_W::new(self)
    }
    #[doc = "Bit 16 - Current target memory in double-buffer mode"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<16> {
        CT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
