#[doc = "Register `ETH_MMC_TX_INTERRUPT` reader"]
pub struct R(crate::R<ETH_MMC_TX_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MMC_TX_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MMC_TX_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MMC_TX_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MMC_TX_INTERRUPT` writer"]
pub struct W(crate::W<ETH_MMC_TX_INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MMC_TX_INTERRUPT_SPEC>;
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
impl From<crate::W<ETH_MMC_TX_INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MMC_TX_INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSCOLGPIS` reader - MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the Tx single collision good packets register (ETH_TX_SINGLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TXSCOLGPIS_R = crate::BitReader<bool>;
#[doc = "Field `TXSCOLGPIS` writer - MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the Tx single collision good packets register (ETH_TX_SINGLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value."]
pub type TXSCOLGPIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_TX_INTERRUPT_SPEC, bool, O>;
#[doc = "Field `TXMCOLGPIS` reader - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the Tx multiple collision good packets register (ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TXMCOLGPIS_R = crate::BitReader<bool>;
#[doc = "Field `TXMCOLGPIS` writer - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the Tx multiple collision good packets register (ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value."]
pub type TXMCOLGPIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_TX_INTERRUPT_SPEC, bool, O>;
#[doc = "Field `TXGPKTIS` reader - MMC Transmit Good Packet Counter Interrupt Status This bit is set when the Tx packet count good register (ETH_TX_PACKET_COUNT_GOOD) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TXGPKTIS_R = crate::BitReader<bool>;
#[doc = "Field `TXGPKTIS` writer - MMC Transmit Good Packet Counter Interrupt Status This bit is set when the Tx packet count good register (ETH_TX_PACKET_COUNT_GOOD) counter reaches half of the maximum value or the maximum value."]
pub type TXGPKTIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_TX_INTERRUPT_SPEC, bool, O>;
#[doc = "Field `TXLPIUSCIS` reader - MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx LPI microsecond timer register (ETH_TX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TXLPIUSCIS_R = crate::BitReader<bool>;
#[doc = "Field `TXLPIUSCIS` writer - MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx LPI microsecond timer register (ETH_TX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
pub type TXLPIUSCIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_TX_INTERRUPT_SPEC, bool, O>;
#[doc = "Field `TXLPITRCIS` reader - MMC Transmit LPI transition counter interrupt status This bit is set when the Tx LPI transition counter register (ETH_TX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TXLPITRCIS_R = crate::BitReader<bool>;
#[doc = "Field `TXLPITRCIS` writer - MMC Transmit LPI transition counter interrupt status This bit is set when the Tx LPI transition counter register (ETH_TX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
pub type TXLPITRCIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_TX_INTERRUPT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the Tx single collision good packets register (ETH_TX_SINGLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the Tx multiple collision good packets register (ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Status This bit is set when the Tx packet count good register (ETH_TX_PACKET_COUNT_GOOD) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx LPI microsecond timer register (ETH_TX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Transmit LPI transition counter interrupt status This bit is set when the Tx LPI transition counter register (ETH_TX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the Tx single collision good packets register (ETH_TX_SINGLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txscolgpis(&mut self) -> TXSCOLGPIS_W<14> {
        TXSCOLGPIS_W::new(self)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the Tx multiple collision good packets register (ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txmcolgpis(&mut self) -> TXMCOLGPIS_W<15> {
        TXMCOLGPIS_W::new(self)
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Status This bit is set when the Tx packet count good register (ETH_TX_PACKET_COUNT_GOOD) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgpktis(&mut self) -> TXGPKTIS_W<21> {
        TXGPKTIS_W::new(self)
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx LPI microsecond timer register (ETH_TX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txlpiuscis(&mut self) -> TXLPIUSCIS_W<26> {
        TXLPIUSCIS_W::new(self)
    }
    #[doc = "Bit 27 - MMC Transmit LPI transition counter interrupt status This bit is set when the Tx LPI transition counter register (ETH_TX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txlpitrcis(&mut self) -> TXLPITRCIS_W<27> {
        TXLPITRCIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Tx interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mmc_tx_interrupt](index.html) module"]
pub struct ETH_MMC_TX_INTERRUPT_SPEC;
impl crate::RegisterSpec for ETH_MMC_TX_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mmc_tx_interrupt::R](R) reader structure"]
impl crate::Readable for ETH_MMC_TX_INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mmc_tx_interrupt::W](W) writer structure"]
impl crate::Writable for ETH_MMC_TX_INTERRUPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MMC_TX_INTERRUPT to value 0"]
impl crate::Resettable for ETH_MMC_TX_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
