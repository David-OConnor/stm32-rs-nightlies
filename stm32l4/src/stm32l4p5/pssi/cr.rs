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
#[doc = "Field `CKPOL` reader - Parallel data clock polarity"]
pub type CKPOL_R = crate::BitReader<CKPOL_A>;
#[doc = "Parallel data clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL_A {
    #[doc = "0: Falling edge active for inputs or rising edge active for outputs"]
    FallingEdge = 0,
    #[doc = "1: Rising edge active for inputs or falling edge active for outputs"]
    RisingEdge = 1,
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
            false => CKPOL_A::FallingEdge,
            true => CKPOL_A::RisingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKPOL_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKPOL_A::RisingEdge
    }
}
#[doc = "Field `CKPOL` writer - Parallel data clock polarity"]
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CKPOL_A, O>;
impl<'a, const O: u8> CKPOL_W<'a, O> {
    #[doc = "Falling edge active for inputs or rising edge active for outputs"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CKPOL_A::FallingEdge)
    }
    #[doc = "Rising edge active for inputs or falling edge active for outputs"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CKPOL_A::RisingEdge)
    }
}
#[doc = "Field `DEPOL` reader - Data enable (PSSI_DE) polarity"]
pub type DEPOL_R = crate::BitReader<DEPOL_A>;
#[doc = "Data enable (PSSI_DE) polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEPOL_A {
    #[doc = "0: PSSI_DE active low (0 indicates that data is valid)"]
    ActiveLow = 0,
    #[doc = "1: PSSI_DE active high (1 indicates that data is valid)"]
    ActiveHigh = 1,
}
impl From<DEPOL_A> for bool {
    #[inline(always)]
    fn from(variant: DEPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl DEPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEPOL_A {
        match self.bits {
            false => DEPOL_A::ActiveLow,
            true => DEPOL_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == DEPOL_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == DEPOL_A::ActiveHigh
    }
}
#[doc = "Field `DEPOL` writer - Data enable (PSSI_DE) polarity"]
pub type DEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DEPOL_A, O>;
impl<'a, const O: u8> DEPOL_W<'a, O> {
    #[doc = "PSSI_DE active low (0 indicates that data is valid)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(DEPOL_A::ActiveLow)
    }
    #[doc = "PSSI_DE active high (1 indicates that data is valid)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(DEPOL_A::ActiveHigh)
    }
}
#[doc = "Field `RDYPOL` reader - Ready (PSSI_RDY) polarity"]
pub type RDYPOL_R = crate::BitReader<RDYPOL_A>;
#[doc = "Ready (PSSI_RDY) polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDYPOL_A {
    #[doc = "0: PSSI_RDY active low (0 indicates that the receiver is ready to receive)"]
    ActiveLow = 0,
    #[doc = "1: PSSI_RDY active high (1 indicates that the receiver is ready to receive)"]
    ActiveHigh = 1,
}
impl From<RDYPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RDYPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl RDYPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDYPOL_A {
        match self.bits {
            false => RDYPOL_A::ActiveLow,
            true => RDYPOL_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == RDYPOL_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == RDYPOL_A::ActiveHigh
    }
}
#[doc = "Field `RDYPOL` writer - Ready (PSSI_RDY) polarity"]
pub type RDYPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RDYPOL_A, O>;
impl<'a, const O: u8> RDYPOL_W<'a, O> {
    #[doc = "PSSI_RDY active low (0 indicates that the receiver is ready to receive)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(RDYPOL_A::ActiveLow)
    }
    #[doc = "PSSI_RDY active high (1 indicates that the receiver is ready to receive)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(RDYPOL_A::ActiveHigh)
    }
}
#[doc = "Field `EDM` reader - Extended data mode"]
pub type EDM_R = crate::FieldReader<u8, EDM_A>;
#[doc = "Extended data mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDM_A {
    #[doc = "0: Interface captures 8-bit data on every parallel data clock"]
    BitWidth8 = 0,
    #[doc = "3: The interface captures 16-bit data on every parallel data clock"]
    BitWidth16 = 3,
}
impl From<EDM_A> for u8 {
    #[inline(always)]
    fn from(variant: EDM_A) -> Self {
        variant as _
    }
}
impl EDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EDM_A> {
        match self.bits {
            0 => Some(EDM_A::BitWidth8),
            3 => Some(EDM_A::BitWidth16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BitWidth8`"]
    #[inline(always)]
    pub fn is_bit_width8(&self) -> bool {
        *self == EDM_A::BitWidth8
    }
    #[doc = "Checks if the value of the field is `BitWidth16`"]
    #[inline(always)]
    pub fn is_bit_width16(&self) -> bool {
        *self == EDM_A::BitWidth16
    }
}
#[doc = "Field `EDM` writer - Extended data mode"]
pub type EDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, EDM_A, 2, O>;
impl<'a, const O: u8> EDM_W<'a, O> {
    #[doc = "Interface captures 8-bit data on every parallel data clock"]
    #[inline(always)]
    pub fn bit_width8(self) -> &'a mut W {
        self.variant(EDM_A::BitWidth8)
    }
    #[doc = "The interface captures 16-bit data on every parallel data clock"]
    #[inline(always)]
    pub fn bit_width16(self) -> &'a mut W {
        self.variant(EDM_A::BitWidth16)
    }
}
#[doc = "Field `ENABLE` reader - PSSI enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "PSSI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: PSSI disabled"]
    Disabled = 0,
    #[doc = "1: PSSI enabled"]
    Enabled = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::Disabled,
            true => ENABLE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::Enabled
    }
}
#[doc = "Field `ENABLE` writer - PSSI enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "PSSI disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::Disabled)
    }
    #[doc = "PSSI enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::Enabled)
    }
}
#[doc = "Field `DERDYCFG` reader - Data enable and ready configuration"]
pub type DERDYCFG_R = crate::FieldReader<u8, DERDYCFG_A>;
#[doc = "Data enable and ready configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DERDYCFG_A {
    #[doc = "0: PSSI_DE and PSSI_RDY both disabled"]
    Disabled = 0,
    #[doc = "1: Only PSSI_RDY enabled"]
    Rdy = 1,
    #[doc = "2: Only PSSI_DE enabled"]
    De = 2,
    #[doc = "3: Both PSSI_RDY and PSSI_DE alternate functions enabled"]
    RdyDeAlt = 3,
    #[doc = "4: Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin"]
    RdyDe = 4,
    #[doc = "5: Only PSSI_RDY function enabled, but mapped to PSSI_DE pin"]
    RdyRemapped = 5,
    #[doc = "6: Only PSSI_DE function enabled, but mapped to PSSI_RDY pin"]
    DeRemapped = 6,
    #[doc = "7: Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin"]
    RdyDeBidi = 7,
}
impl From<DERDYCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DERDYCFG_A) -> Self {
        variant as _
    }
}
impl DERDYCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DERDYCFG_A {
        match self.bits {
            0 => DERDYCFG_A::Disabled,
            1 => DERDYCFG_A::Rdy,
            2 => DERDYCFG_A::De,
            3 => DERDYCFG_A::RdyDeAlt,
            4 => DERDYCFG_A::RdyDe,
            5 => DERDYCFG_A::RdyRemapped,
            6 => DERDYCFG_A::DeRemapped,
            7 => DERDYCFG_A::RdyDeBidi,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DERDYCFG_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Rdy`"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == DERDYCFG_A::Rdy
    }
    #[doc = "Checks if the value of the field is `De`"]
    #[inline(always)]
    pub fn is_de(&self) -> bool {
        *self == DERDYCFG_A::De
    }
    #[doc = "Checks if the value of the field is `RdyDeAlt`"]
    #[inline(always)]
    pub fn is_rdy_de_alt(&self) -> bool {
        *self == DERDYCFG_A::RdyDeAlt
    }
    #[doc = "Checks if the value of the field is `RdyDe`"]
    #[inline(always)]
    pub fn is_rdy_de(&self) -> bool {
        *self == DERDYCFG_A::RdyDe
    }
    #[doc = "Checks if the value of the field is `RdyRemapped`"]
    #[inline(always)]
    pub fn is_rdy_remapped(&self) -> bool {
        *self == DERDYCFG_A::RdyRemapped
    }
    #[doc = "Checks if the value of the field is `DeRemapped`"]
    #[inline(always)]
    pub fn is_de_remapped(&self) -> bool {
        *self == DERDYCFG_A::DeRemapped
    }
    #[doc = "Checks if the value of the field is `RdyDeBidi`"]
    #[inline(always)]
    pub fn is_rdy_de_bidi(&self) -> bool {
        *self == DERDYCFG_A::RdyDeBidi
    }
}
#[doc = "Field `DERDYCFG` writer - Data enable and ready configuration"]
pub type DERDYCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, DERDYCFG_A, 3, O>;
impl<'a, const O: u8> DERDYCFG_W<'a, O> {
    #[doc = "PSSI_DE and PSSI_RDY both disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DERDYCFG_A::Disabled)
    }
    #[doc = "Only PSSI_RDY enabled"]
    #[inline(always)]
    pub fn rdy(self) -> &'a mut W {
        self.variant(DERDYCFG_A::Rdy)
    }
    #[doc = "Only PSSI_DE enabled"]
    #[inline(always)]
    pub fn de(self) -> &'a mut W {
        self.variant(DERDYCFG_A::De)
    }
    #[doc = "Both PSSI_RDY and PSSI_DE alternate functions enabled"]
    #[inline(always)]
    pub fn rdy_de_alt(self) -> &'a mut W {
        self.variant(DERDYCFG_A::RdyDeAlt)
    }
    #[doc = "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin"]
    #[inline(always)]
    pub fn rdy_de(self) -> &'a mut W {
        self.variant(DERDYCFG_A::RdyDe)
    }
    #[doc = "Only PSSI_RDY function enabled, but mapped to PSSI_DE pin"]
    #[inline(always)]
    pub fn rdy_remapped(self) -> &'a mut W {
        self.variant(DERDYCFG_A::RdyRemapped)
    }
    #[doc = "Only PSSI_DE function enabled, but mapped to PSSI_RDY pin"]
    #[inline(always)]
    pub fn de_remapped(self) -> &'a mut W {
        self.variant(DERDYCFG_A::DeRemapped)
    }
    #[doc = "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin"]
    #[inline(always)]
    pub fn rdy_de_bidi(self) -> &'a mut W {
        self.variant(DERDYCFG_A::RdyDeBidi)
    }
}
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled."]
    Disabled = 0,
    #[doc = "1: DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR"]
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
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    #[doc = "DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
#[doc = "Field `OUTEN` reader - Data direction selection bit"]
pub type OUTEN_R = crate::BitReader<OUTEN_A>;
#[doc = "Data direction selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTEN_A {
    #[doc = "0: Data is input synchronously with PSSI_PDCK"]
    ReceiveMode = 0,
    #[doc = "1: Data is output synchronously with PSSI_PDCK"]
    TransmitMode = 1,
}
impl From<OUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: OUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTEN_A {
        match self.bits {
            false => OUTEN_A::ReceiveMode,
            true => OUTEN_A::TransmitMode,
        }
    }
    #[doc = "Checks if the value of the field is `ReceiveMode`"]
    #[inline(always)]
    pub fn is_receive_mode(&self) -> bool {
        *self == OUTEN_A::ReceiveMode
    }
    #[doc = "Checks if the value of the field is `TransmitMode`"]
    #[inline(always)]
    pub fn is_transmit_mode(&self) -> bool {
        *self == OUTEN_A::TransmitMode
    }
}
#[doc = "Field `OUTEN` writer - Data direction selection bit"]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OUTEN_A, O>;
impl<'a, const O: u8> OUTEN_W<'a, O> {
    #[doc = "Data is input synchronously with PSSI_PDCK"]
    #[inline(always)]
    pub fn receive_mode(self) -> &'a mut W {
        self.variant(OUTEN_A::ReceiveMode)
    }
    #[doc = "Data is output synchronously with PSSI_PDCK"]
    #[inline(always)]
    pub fn transmit_mode(self) -> &'a mut W {
        self.variant(OUTEN_A::TransmitMode)
    }
}
impl R {
    #[doc = "Bit 5 - Parallel data clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data enable (PSSI_DE) polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Ready (PSSI_RDY) polarity"]
    #[inline(always)]
    pub fn rdypol(&self) -> RDYPOL_R {
        RDYPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - PSSI enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Data enable and ready configuration"]
    #[inline(always)]
    pub fn derdycfg(&self) -> DERDYCFG_R {
        DERDYCFG_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data direction selection bit"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Parallel data clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<5> {
        CKPOL_W::new(self)
    }
    #[doc = "Bit 6 - Data enable (PSSI_DE) polarity"]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<6> {
        DEPOL_W::new(self)
    }
    #[doc = "Bit 8 - Ready (PSSI_RDY) polarity"]
    #[inline(always)]
    #[must_use]
    pub fn rdypol(&mut self) -> RDYPOL_W<8> {
        RDYPOL_W::new(self)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<10> {
        EDM_W::new(self)
    }
    #[doc = "Bit 14 - PSSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<14> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 18:20 - Data enable and ready configuration"]
    #[inline(always)]
    #[must_use]
    pub fn derdycfg(&mut self) -> DERDYCFG_W<18> {
        DERDYCFG_W::new(self)
    }
    #[doc = "Bit 30 - DMA enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<30> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 31 - Data direction selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<31> {
        OUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PSSI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0x4000_0000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
