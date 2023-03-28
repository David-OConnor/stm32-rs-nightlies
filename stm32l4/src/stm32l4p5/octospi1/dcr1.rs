#[doc = "Register `DCR1` reader"]
pub struct R(crate::R<DCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR1` writer"]
pub struct W(crate::W<DCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR1_SPEC>;
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
impl From<crate::W<DCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKMODE` reader - Mode 0 / mode 3"]
pub type CKMODE_R = crate::BitReader<CKMODE_A>;
#[doc = "Mode 0 / mode 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKMODE_A {
    #[doc = "0: CLK must stay low while NCS is high (chip-select released). This is referred to as Mode 0"]
    Mode0 = 0,
    #[doc = "1: CLK must stay high while NCS is high (chip-select released). This is referred to as Mode 3"]
    Mode3 = 1,
}
impl From<CKMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            false => CKMODE_A::Mode0,
            true => CKMODE_A::Mode3,
        }
    }
    #[doc = "Checks if the value of the field is `Mode0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == CKMODE_A::Mode0
    }
    #[doc = "Checks if the value of the field is `Mode3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == CKMODE_A::Mode3
    }
}
#[doc = "Field `CKMODE` writer - Mode 0 / mode 3"]
pub type CKMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, CKMODE_A, O>;
impl<'a, const O: u8> CKMODE_W<'a, O> {
    #[doc = "CLK must stay low while NCS is high (chip-select released). This is referred to as Mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(CKMODE_A::Mode0)
    }
    #[doc = "CLK must stay high while NCS is high (chip-select released). This is referred to as Mode 3"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(CKMODE_A::Mode3)
    }
}
#[doc = "Field `FRCK` reader - Free running clock"]
pub type FRCK_R = crate::BitReader<FRCK_A>;
#[doc = "Free running clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRCK_A {
    #[doc = "0: CLK is not free running"]
    Disabled = 0,
    #[doc = "1: CLK is free running (always provided)"]
    Enabled = 1,
}
impl From<FRCK_A> for bool {
    #[inline(always)]
    fn from(variant: FRCK_A) -> Self {
        variant as u8 != 0
    }
}
impl FRCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRCK_A {
        match self.bits {
            false => FRCK_A::Disabled,
            true => FRCK_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRCK_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRCK_A::Enabled
    }
}
#[doc = "Field `FRCK` writer - Free running clock"]
pub type FRCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, FRCK_A, O>;
impl<'a, const O: u8> FRCK_W<'a, O> {
    #[doc = "CLK is not free running"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRCK_A::Disabled)
    }
    #[doc = "CLK is free running (always provided)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRCK_A::Enabled)
    }
}
#[doc = "Field `DLYBYP` reader - Delay block bypass"]
pub type DLYBYP_R = crate::BitReader<DLYBYP_A>;
#[doc = "Delay block bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBYP_A {
    #[doc = "0: The internal sampling clock (called feedback clock) or the DQS data strobe external signal is delayed by the delay block (for more details on this block, refer to the dedicated section of the reference manual as it is not part of the OCTOSPI peripheral)"]
    DelayBlockEnabled = 0,
    #[doc = "1: The delay block is bypassed, so the internal sampling clock or the DQS data strobe external signal is not affected by the delay block. The delay is shorter than when the delay block is not bypassed, even with the delay value set to minimum value in delay block"]
    DelayBlockBypassed = 1,
}
impl From<DLYBYP_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYBYP_A {
        match self.bits {
            false => DLYBYP_A::DelayBlockEnabled,
            true => DLYBYP_A::DelayBlockBypassed,
        }
    }
    #[doc = "Checks if the value of the field is `DelayBlockEnabled`"]
    #[inline(always)]
    pub fn is_delay_block_enabled(&self) -> bool {
        *self == DLYBYP_A::DelayBlockEnabled
    }
    #[doc = "Checks if the value of the field is `DelayBlockBypassed`"]
    #[inline(always)]
    pub fn is_delay_block_bypassed(&self) -> bool {
        *self == DLYBYP_A::DelayBlockBypassed
    }
}
#[doc = "Field `DLYBYP` writer - Delay block bypass"]
pub type DLYBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, DLYBYP_A, O>;
impl<'a, const O: u8> DLYBYP_W<'a, O> {
    #[doc = "The internal sampling clock (called feedback clock) or the DQS data strobe external signal is delayed by the delay block (for more details on this block, refer to the dedicated section of the reference manual as it is not part of the OCTOSPI peripheral)"]
    #[inline(always)]
    pub fn delay_block_enabled(self) -> &'a mut W {
        self.variant(DLYBYP_A::DelayBlockEnabled)
    }
    #[doc = "The delay block is bypassed, so the internal sampling clock or the DQS data strobe external signal is not affected by the delay block. The delay is shorter than when the delay block is not bypassed, even with the delay value set to minimum value in delay block"]
    #[inline(always)]
    pub fn delay_block_bypassed(self) -> &'a mut W {
        self.variant(DLYBYP_A::DelayBlockBypassed)
    }
}
#[doc = "Field `CSHT` reader - Chip-select high time"]
pub type CSHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSHT` writer - Chip-select high time"]
pub type CSHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `DEVSIZE` reader - Device size"]
pub type DEVSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVSIZE` writer - Device size"]
pub type DEVSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `MTYP` reader - Memory type"]
pub type MTYP_R = crate::FieldReader<u8, MTYP_A>;
#[doc = "Memory type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MTYP_A {
    #[doc = "0: Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    MicronMode = 0,
    #[doc = "1: Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    MacronixMode = 1,
    #[doc = "2: Standard Mode"]
    StandardMode = 2,
    #[doc = "3: Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes with dedicated address mapping"]
    MacronixRamMode = 3,
    #[doc = "4: HyperBus memory mode, the protocol follows the HyperBus specification. 8-data-bit DTR mode must be selected"]
    HyperBusMemoryMode = 4,
    #[doc = "5: HyperBus register mode, addressing register space. The memory-mapped accesses in this mode must be non-cacheable, or Indirect read/write modes must be used"]
    HyperBusMode = 5,
}
impl From<MTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: MTYP_A) -> Self {
        variant as _
    }
}
impl MTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MTYP_A> {
        match self.bits {
            0 => Some(MTYP_A::MicronMode),
            1 => Some(MTYP_A::MacronixMode),
            2 => Some(MTYP_A::StandardMode),
            3 => Some(MTYP_A::MacronixRamMode),
            4 => Some(MTYP_A::HyperBusMemoryMode),
            5 => Some(MTYP_A::HyperBusMode),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MicronMode`"]
    #[inline(always)]
    pub fn is_micron_mode(&self) -> bool {
        *self == MTYP_A::MicronMode
    }
    #[doc = "Checks if the value of the field is `MacronixMode`"]
    #[inline(always)]
    pub fn is_macronix_mode(&self) -> bool {
        *self == MTYP_A::MacronixMode
    }
    #[doc = "Checks if the value of the field is `StandardMode`"]
    #[inline(always)]
    pub fn is_standard_mode(&self) -> bool {
        *self == MTYP_A::StandardMode
    }
    #[doc = "Checks if the value of the field is `MacronixRamMode`"]
    #[inline(always)]
    pub fn is_macronix_ram_mode(&self) -> bool {
        *self == MTYP_A::MacronixRamMode
    }
    #[doc = "Checks if the value of the field is `HyperBusMemoryMode`"]
    #[inline(always)]
    pub fn is_hyper_bus_memory_mode(&self) -> bool {
        *self == MTYP_A::HyperBusMemoryMode
    }
    #[doc = "Checks if the value of the field is `HyperBusMode`"]
    #[inline(always)]
    pub fn is_hyper_bus_mode(&self) -> bool {
        *self == MTYP_A::HyperBusMode
    }
}
#[doc = "Field `MTYP` writer - Memory type"]
pub type MTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR1_SPEC, u8, MTYP_A, 3, O>;
impl<'a, const O: u8> MTYP_W<'a, O> {
    #[doc = "Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    #[inline(always)]
    pub fn micron_mode(self) -> &'a mut W {
        self.variant(MTYP_A::MicronMode)
    }
    #[doc = "Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    #[inline(always)]
    pub fn macronix_mode(self) -> &'a mut W {
        self.variant(MTYP_A::MacronixMode)
    }
    #[doc = "Standard Mode"]
    #[inline(always)]
    pub fn standard_mode(self) -> &'a mut W {
        self.variant(MTYP_A::StandardMode)
    }
    #[doc = "Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes with dedicated address mapping"]
    #[inline(always)]
    pub fn macronix_ram_mode(self) -> &'a mut W {
        self.variant(MTYP_A::MacronixRamMode)
    }
    #[doc = "HyperBus memory mode, the protocol follows the HyperBus specification. 8-data-bit DTR mode must be selected"]
    #[inline(always)]
    pub fn hyper_bus_memory_mode(self) -> &'a mut W {
        self.variant(MTYP_A::HyperBusMemoryMode)
    }
    #[doc = "HyperBus register mode, addressing register space. The memory-mapped accesses in this mode must be non-cacheable, or Indirect read/write modes must be used"]
    #[inline(always)]
    pub fn hyper_bus_mode(self) -> &'a mut W {
        self.variant(MTYP_A::HyperBusMode)
    }
}
impl R {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free running clock"]
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    pub fn dlybyp(&self) -> DLYBYP_R {
        DLYBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Chip-select high time"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Device size"]
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Memory type"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<0> {
        CKMODE_W::new(self)
    }
    #[doc = "Bit 1 - Free running clock"]
    #[inline(always)]
    #[must_use]
    pub fn frck(&mut self) -> FRCK_W<1> {
        FRCK_W::new(self)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    #[must_use]
    pub fn dlybyp(&mut self) -> DLYBYP_W<3> {
        DLYBYP_W::new(self)
    }
    #[doc = "Bits 8:10 - Chip-select high time"]
    #[inline(always)]
    #[must_use]
    pub fn csht(&mut self) -> CSHT_W<8> {
        CSHT_W::new(self)
    }
    #[doc = "Bits 16:20 - Device size"]
    #[inline(always)]
    #[must_use]
    pub fn devsize(&mut self) -> DEVSIZE_W<16> {
        DEVSIZE_W::new(self)
    }
    #[doc = "Bits 24:26 - Memory type"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<24> {
        MTYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr1](index.html) module"]
pub struct DCR1_SPEC;
impl crate::RegisterSpec for DCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr1::R](R) reader structure"]
impl crate::Readable for DCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr1::W](W) writer structure"]
impl crate::Writable for DCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCR1 to value 0"]
impl crate::Resettable for DCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
