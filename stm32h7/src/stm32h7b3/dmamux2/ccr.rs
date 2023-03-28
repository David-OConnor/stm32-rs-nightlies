#[doc = "Register `CCR%s` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR%s` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAREQ_ID` reader - Input DMA request line selected"]
pub type DMAREQ_ID_R = crate::FieldReader<u8, DMAREQ_ID_A>;
#[doc = "Input DMA request line selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAREQ_ID_A {
    #[doc = "0: No signal selected as request input"]
    None = 0,
    #[doc = "1: Signal `dmamux2_req_gen0` selected as request input"]
    Dmamux2ReqGen0 = 1,
    #[doc = "2: Signal `dmamux2_req_gen1` selected as request input"]
    Dmamux2ReqGen1 = 2,
    #[doc = "3: Signal `dmamux2_req_gen2` selected as request input"]
    Dmamux2ReqGen2 = 3,
    #[doc = "4: Signal `dmamux2_req_gen3` selected as request input"]
    Dmamux2ReqGen3 = 4,
    #[doc = "5: Signal `dmamux2_req_gen4` selected as request input"]
    Dmamux2ReqGen4 = 5,
    #[doc = "6: Signal `dmamux2_req_gen5` selected as request input"]
    Dmamux2ReqGen5 = 6,
    #[doc = "7: Signal `dmamux2_req_gen6` selected as request input"]
    Dmamux2ReqGen6 = 7,
    #[doc = "8: Signal `dmamux2_req_gen7` selected as request input"]
    Dmamux2ReqGen7 = 8,
    #[doc = "9: Signal `lpuart1_rx_dma` selected as request input"]
    Lpuart1RxDma = 9,
    #[doc = "10: Signal `lpuart1_tx_dma` selected as request input"]
    Lpuart1TxDma = 10,
    #[doc = "11: Signal `spi6_rx_dma` selected as request input"]
    Spi6RxDma = 11,
    #[doc = "12: Signal `spi6_tx_dma` selected as request input"]
    Spi6TxDma = 12,
    #[doc = "13: Signal `i2c4_rx_dma` selected as request input"]
    I2c4RxDma = 13,
    #[doc = "14: Signal `i2c4_tx_dma` selected as request input"]
    I2c4TxDma = 14,
    #[doc = "15: Signal `sai4_a_dma` selected as request input"]
    Sai4ADma = 15,
    #[doc = "16: Signal `sai4_b_dma` selected as request input"]
    Sai4BDma = 16,
    #[doc = "17: Signal `adc3_dma` selected as request input"]
    Adc3Dma = 17,
}
impl From<DMAREQ_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAREQ_ID_A) -> Self {
        variant as _
    }
}
impl DMAREQ_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAREQ_ID_A> {
        match self.bits {
            0 => Some(DMAREQ_ID_A::None),
            1 => Some(DMAREQ_ID_A::Dmamux2ReqGen0),
            2 => Some(DMAREQ_ID_A::Dmamux2ReqGen1),
            3 => Some(DMAREQ_ID_A::Dmamux2ReqGen2),
            4 => Some(DMAREQ_ID_A::Dmamux2ReqGen3),
            5 => Some(DMAREQ_ID_A::Dmamux2ReqGen4),
            6 => Some(DMAREQ_ID_A::Dmamux2ReqGen5),
            7 => Some(DMAREQ_ID_A::Dmamux2ReqGen6),
            8 => Some(DMAREQ_ID_A::Dmamux2ReqGen7),
            9 => Some(DMAREQ_ID_A::Lpuart1RxDma),
            10 => Some(DMAREQ_ID_A::Lpuart1TxDma),
            11 => Some(DMAREQ_ID_A::Spi6RxDma),
            12 => Some(DMAREQ_ID_A::Spi6TxDma),
            13 => Some(DMAREQ_ID_A::I2c4RxDma),
            14 => Some(DMAREQ_ID_A::I2c4TxDma),
            15 => Some(DMAREQ_ID_A::Sai4ADma),
            16 => Some(DMAREQ_ID_A::Sai4BDma),
            17 => Some(DMAREQ_ID_A::Adc3Dma),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAREQ_ID_A::None
    }
    #[doc = "Checks if the value of the field is `Dmamux2ReqGen0`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen0(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux2ReqGen0
    }
    #[doc = "Checks if the value of the field is `Dmamux2ReqGen1`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen1(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux2ReqGen1
    }
    #[doc = "Checks if the value of the field is `Dmamux2ReqGen2`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen2(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux2ReqGen2
    }
    #[doc = "Checks if the value of the field is `Dmamux2ReqGen3`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen3(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux2ReqGen3
    }
    #[doc = "Checks if the value of the field is `Dmamux2ReqGen4`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen4(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux2ReqGen4
    }
    #[doc = "Checks if the value of the field is `Dmamux2ReqGen5`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen5(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux2ReqGen5
    }
    #[doc = "Checks if the value of the field is `Dmamux2ReqGen6`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen6(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux2ReqGen6
    }
    #[doc = "Checks if the value of the field is `Dmamux2ReqGen7`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen7(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux2ReqGen7
    }
    #[doc = "Checks if the value of the field is `Lpuart1RxDma`"]
    #[inline(always)]
    pub fn is_lpuart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Lpuart1RxDma
    }
    #[doc = "Checks if the value of the field is `Lpuart1TxDma`"]
    #[inline(always)]
    pub fn is_lpuart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Lpuart1TxDma
    }
    #[doc = "Checks if the value of the field is `Spi6RxDma`"]
    #[inline(always)]
    pub fn is_spi6_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi6RxDma
    }
    #[doc = "Checks if the value of the field is `Spi6TxDma`"]
    #[inline(always)]
    pub fn is_spi6_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi6TxDma
    }
    #[doc = "Checks if the value of the field is `I2c4RxDma`"]
    #[inline(always)]
    pub fn is_i2c4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c4RxDma
    }
    #[doc = "Checks if the value of the field is `I2c4TxDma`"]
    #[inline(always)]
    pub fn is_i2c4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c4TxDma
    }
    #[doc = "Checks if the value of the field is `Sai4ADma`"]
    #[inline(always)]
    pub fn is_sai4_a_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Sai4ADma
    }
    #[doc = "Checks if the value of the field is `Sai4BDma`"]
    #[inline(always)]
    pub fn is_sai4_b_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Sai4BDma
    }
    #[doc = "Checks if the value of the field is `Adc3Dma`"]
    #[inline(always)]
    pub fn is_adc3_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Adc3Dma
    }
}
#[doc = "Field `DMAREQ_ID` writer - Input DMA request line selected"]
pub type DMAREQ_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCR_SPEC, u8, DMAREQ_ID_A, 8, O>;
impl<'a, const O: u8> DMAREQ_ID_W<'a, O> {
    #[doc = "No signal selected as request input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::None)
    }
    #[doc = "Signal `dmamux2_req_gen0` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen0(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux2ReqGen0)
    }
    #[doc = "Signal `dmamux2_req_gen1` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux2ReqGen1)
    }
    #[doc = "Signal `dmamux2_req_gen2` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux2ReqGen2)
    }
    #[doc = "Signal `dmamux2_req_gen3` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux2ReqGen3)
    }
    #[doc = "Signal `dmamux2_req_gen4` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux2ReqGen4)
    }
    #[doc = "Signal `dmamux2_req_gen5` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen5(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux2ReqGen5)
    }
    #[doc = "Signal `dmamux2_req_gen6` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen6(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux2ReqGen6)
    }
    #[doc = "Signal `dmamux2_req_gen7` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen7(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux2ReqGen7)
    }
    #[doc = "Signal `lpuart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn lpuart1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Lpuart1RxDma)
    }
    #[doc = "Signal `lpuart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn lpuart1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Lpuart1TxDma)
    }
    #[doc = "Signal `spi6_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi6_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi6RxDma)
    }
    #[doc = "Signal `spi6_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi6_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi6TxDma)
    }
    #[doc = "Signal `i2c4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c4_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c4RxDma)
    }
    #[doc = "Signal `i2c4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c4_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c4TxDma)
    }
    #[doc = "Signal `sai4_a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai4_a_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Sai4ADma)
    }
    #[doc = "Signal `sai4_b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai4_b_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Sai4BDma)
    }
    #[doc = "Signal `adc3_dma` selected as request input"]
    #[inline(always)]
    pub fn adc3_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Adc3Dma)
    }
}
#[doc = "Field `SOIE` reader - Interrupt enable at synchronization event overrun"]
pub type SOIE_R = crate::BitReader<SOIE_A>;
#[doc = "Interrupt enable at synchronization event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE_A {
    #[doc = "0: Synchronization overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Synchronization overrun interrupt enabled"]
    Enabled = 1,
}
impl From<SOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOIE_A {
        match self.bits {
            false => SOIE_A::Disabled,
            true => SOIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE_A::Enabled
    }
}
#[doc = "Field `SOIE` writer - Interrupt enable at synchronization event overrun"]
pub type SOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, SOIE_A, O>;
impl<'a, const O: u8> SOIE_W<'a, O> {
    #[doc = "Synchronization overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOIE_A::Disabled)
    }
    #[doc = "Synchronization overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOIE_A::Enabled)
    }
}
#[doc = "Field `EGE` reader - Event generation enable/disable"]
pub type EGE_R = crate::BitReader<EGE_A>;
#[doc = "Event generation enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE_A {
    #[doc = "0: Event generation disabled"]
    Disabled = 0,
    #[doc = "1: Event generation enabled"]
    Enabled = 1,
}
impl From<EGE_A> for bool {
    #[inline(always)]
    fn from(variant: EGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EGE_A {
        match self.bits {
            false => EGE_A::Disabled,
            true => EGE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE_A::Enabled
    }
}
#[doc = "Field `EGE` writer - Event generation enable/disable"]
pub type EGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, EGE_A, O>;
impl<'a, const O: u8> EGE_W<'a, O> {
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EGE_A::Disabled)
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EGE_A::Enabled)
    }
}
#[doc = "Field `SE` reader - Synchronous operating mode enable/disable"]
pub type SE_R = crate::BitReader<SE_A>;
#[doc = "Synchronous operating mode enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE_A {
    #[doc = "0: Synchronization disabled"]
    Disabled = 0,
    #[doc = "1: Synchronization enabled"]
    Enabled = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
impl SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::Disabled,
            true => SE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE_A::Enabled
    }
}
#[doc = "Field `SE` writer - Synchronous operating mode enable/disable"]
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, SE_A, O>;
impl<'a, const O: u8> SE_W<'a, O> {
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SE_A::Disabled)
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SE_A::Enabled)
    }
}
#[doc = "Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SPOL_R = crate::FieldReader<u8, SPOL_A>;
#[doc = "Synchronization event type selector Defines the synchronization event on the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL_A {
    #[doc = "0: No event, i.e. no synchronization nor detection"]
    NoEdge = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
    BothEdges = 3,
}
impl From<SPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as _
    }
}
impl SPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            0 => SPOL_A::NoEdge,
            1 => SPOL_A::RisingEdge,
            2 => SPOL_A::FallingEdge,
            3 => SPOL_A::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoEdge`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL_A::NoEdge
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `BothEdges`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL_A::BothEdges
    }
}
#[doc = "Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SPOL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, SPOL_A, 2, O>;
impl<'a, const O: u8> SPOL_W<'a, O> {
    #[doc = "No event, i.e. no synchronization nor detection"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(SPOL_A::NoEdge)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SPOL_A::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SPOL_A::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(SPOL_A::BothEdges)
    }
}
#[doc = "Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NBREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NBREQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `SYNC_ID` reader - Synchronization input selected"]
pub type SYNC_ID_R = crate::FieldReader<u8, SYNC_ID_A>;
#[doc = "Synchronization input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC_ID_A {
    #[doc = "0: Signal `dmamux2_evt0` selected as synchronization input"]
    Dmamux2Evt0 = 0,
    #[doc = "1: Signal `dmamux2_evt1` selected as synchronization input"]
    Dmamux2Evt1 = 1,
    #[doc = "2: Signal `dmamux2_evt2` selected as synchronization input"]
    Dmamux2Evt2 = 2,
    #[doc = "3: Signal `dmamux2_evt3` selected as synchronization input"]
    Dmamux2Evt3 = 3,
    #[doc = "4: Signal `dmamux2_evt4` selected as synchronization input"]
    Dmamux2Evt4 = 4,
    #[doc = "5: Signal `dmamux2_evt5` selected as synchronization input"]
    Dmamux2Evt5 = 5,
    #[doc = "6: Signal `lpuart1_rx_wkup` selected as synchronization input"]
    Lpuart1RxWkup = 6,
    #[doc = "7: Signal `lpuart1_tx_wkup` selected as synchronization input"]
    Lpuart1TxWkup = 7,
    #[doc = "8: Signal `lptim2_out` selected as synchronization input"]
    Lptim2Out = 8,
    #[doc = "9: Signal `lptim3_out` selected as synchronization input"]
    Lptim3Out = 9,
    #[doc = "10: Signal `i2c4_wkup` selected as synchronization input"]
    I2c4Wkup = 10,
    #[doc = "11: Signal `spi6_wkup` selected as synchronization input"]
    Spi6Wkup = 11,
    #[doc = "12: Signal `comp1_out` selected as synchronization input"]
    Comp1Out = 12,
    #[doc = "13: Signal `rtc_wkup` selected as synchronization input"]
    RtcWkup = 13,
    #[doc = "14: Signal `syscfg_exti0_mux` selected as synchronization input"]
    SyscfgExti0Mux = 14,
    #[doc = "15: Signal `syscfg_exti2_mux` selected as synchronization input"]
    SyscfgExti2Mux = 15,
}
impl From<SYNC_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_ID_A) -> Self {
        variant as _
    }
}
impl SYNC_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNC_ID_A> {
        match self.bits {
            0 => Some(SYNC_ID_A::Dmamux2Evt0),
            1 => Some(SYNC_ID_A::Dmamux2Evt1),
            2 => Some(SYNC_ID_A::Dmamux2Evt2),
            3 => Some(SYNC_ID_A::Dmamux2Evt3),
            4 => Some(SYNC_ID_A::Dmamux2Evt4),
            5 => Some(SYNC_ID_A::Dmamux2Evt5),
            6 => Some(SYNC_ID_A::Lpuart1RxWkup),
            7 => Some(SYNC_ID_A::Lpuart1TxWkup),
            8 => Some(SYNC_ID_A::Lptim2Out),
            9 => Some(SYNC_ID_A::Lptim3Out),
            10 => Some(SYNC_ID_A::I2c4Wkup),
            11 => Some(SYNC_ID_A::Spi6Wkup),
            12 => Some(SYNC_ID_A::Comp1Out),
            13 => Some(SYNC_ID_A::RtcWkup),
            14 => Some(SYNC_ID_A::SyscfgExti0Mux),
            15 => Some(SYNC_ID_A::SyscfgExti2Mux),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt0`"]
    #[inline(always)]
    pub fn is_dmamux2_evt0(&self) -> bool {
        *self == SYNC_ID_A::Dmamux2Evt0
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt1`"]
    #[inline(always)]
    pub fn is_dmamux2_evt1(&self) -> bool {
        *self == SYNC_ID_A::Dmamux2Evt1
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt2`"]
    #[inline(always)]
    pub fn is_dmamux2_evt2(&self) -> bool {
        *self == SYNC_ID_A::Dmamux2Evt2
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt3`"]
    #[inline(always)]
    pub fn is_dmamux2_evt3(&self) -> bool {
        *self == SYNC_ID_A::Dmamux2Evt3
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt4`"]
    #[inline(always)]
    pub fn is_dmamux2_evt4(&self) -> bool {
        *self == SYNC_ID_A::Dmamux2Evt4
    }
    #[doc = "Checks if the value of the field is `Dmamux2Evt5`"]
    #[inline(always)]
    pub fn is_dmamux2_evt5(&self) -> bool {
        *self == SYNC_ID_A::Dmamux2Evt5
    }
    #[doc = "Checks if the value of the field is `Lpuart1RxWkup`"]
    #[inline(always)]
    pub fn is_lpuart1_rx_wkup(&self) -> bool {
        *self == SYNC_ID_A::Lpuart1RxWkup
    }
    #[doc = "Checks if the value of the field is `Lpuart1TxWkup`"]
    #[inline(always)]
    pub fn is_lpuart1_tx_wkup(&self) -> bool {
        *self == SYNC_ID_A::Lpuart1TxWkup
    }
    #[doc = "Checks if the value of the field is `Lptim2Out`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SYNC_ID_A::Lptim2Out
    }
    #[doc = "Checks if the value of the field is `Lptim3Out`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SYNC_ID_A::Lptim3Out
    }
    #[doc = "Checks if the value of the field is `I2c4Wkup`"]
    #[inline(always)]
    pub fn is_i2c4_wkup(&self) -> bool {
        *self == SYNC_ID_A::I2c4Wkup
    }
    #[doc = "Checks if the value of the field is `Spi6Wkup`"]
    #[inline(always)]
    pub fn is_spi6_wkup(&self) -> bool {
        *self == SYNC_ID_A::Spi6Wkup
    }
    #[doc = "Checks if the value of the field is `Comp1Out`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == SYNC_ID_A::Comp1Out
    }
    #[doc = "Checks if the value of the field is `RtcWkup`"]
    #[inline(always)]
    pub fn is_rtc_wkup(&self) -> bool {
        *self == SYNC_ID_A::RtcWkup
    }
    #[doc = "Checks if the value of the field is `SyscfgExti0Mux`"]
    #[inline(always)]
    pub fn is_syscfg_exti0_mux(&self) -> bool {
        *self == SYNC_ID_A::SyscfgExti0Mux
    }
    #[doc = "Checks if the value of the field is `SyscfgExti2Mux`"]
    #[inline(always)]
    pub fn is_syscfg_exti2_mux(&self) -> bool {
        *self == SYNC_ID_A::SyscfgExti2Mux
    }
}
#[doc = "Field `SYNC_ID` writer - Synchronization input selected"]
pub type SYNC_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, SYNC_ID_A, 5, O>;
impl<'a, const O: u8> SYNC_ID_W<'a, O> {
    #[doc = "Signal `dmamux2_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux2Evt0)
    }
    #[doc = "Signal `dmamux2_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt1(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux2Evt1)
    }
    #[doc = "Signal `dmamux2_evt2` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt2(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux2Evt2)
    }
    #[doc = "Signal `dmamux2_evt3` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt3(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux2Evt3)
    }
    #[doc = "Signal `dmamux2_evt4` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt4(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux2Evt4)
    }
    #[doc = "Signal `dmamux2_evt5` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt5(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux2Evt5)
    }
    #[doc = "Signal `lpuart1_rx_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn lpuart1_rx_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lpuart1RxWkup)
    }
    #[doc = "Signal `lpuart1_tx_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn lpuart1_tx_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lpuart1TxWkup)
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lptim2Out)
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lptim3Out)
    }
    #[doc = "Signal `i2c4_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn i2c4_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::I2c4Wkup)
    }
    #[doc = "Signal `spi6_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn spi6_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Spi6Wkup)
    }
    #[doc = "Signal `comp1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Comp1Out)
    }
    #[doc = "Signal `rtc_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn rtc_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::RtcWkup)
    }
    #[doc = "Signal `syscfg_exti0_mux` selected as synchronization input"]
    #[inline(always)]
    pub fn syscfg_exti0_mux(self) -> &'a mut W {
        self.variant(SYNC_ID_A::SyscfgExti0Mux)
    }
    #[doc = "Signal `syscfg_exti2_mux` selected as synchronization input"]
    #[inline(always)]
    pub fn syscfg_exti2_mux(self) -> &'a mut W {
        self.variant(SYNC_ID_A::SyscfgExti2Mux)
    }
}
impl R {
    #[doc = "Bits 0:7 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input DMA request line selected"]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<0> {
        DMAREQ_ID_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<8> {
        SOIE_W::new(self)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<9> {
        EGE_W::new(self)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<16> {
        SE_W::new(self)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<17> {
        SPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<19> {
        NBREQ_W::new(self)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<24> {
        SYNC_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMux - DMA request line multiplexer channel x control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR%s to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
