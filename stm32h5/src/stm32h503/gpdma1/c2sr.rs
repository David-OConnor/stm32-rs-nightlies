#[doc = "Register `C2SR` reader"]
pub struct R(crate::R<C2SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDLEF` reader - idle flag This idle flag is deasserted by hardware when the channel is enabled (GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state)."]
pub type IDLEF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` reader - transfer complete flag A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, or a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\])."]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `HTF` reader - half transfer flag A half transfer event is either a half block transfer or a half 2D/repeated block transfer, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\]). A half block transfer occurs when half of the bytes of the source block size (rounded up integer of GPDMA_CxBR1.BNDT\\[15:0\\]/2) has been transferred to the destination. A half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (GPDMA_CxBR1.BRC\\[10:0\\]+1)/2)) has been transferred to the destination."]
pub type HTF_R = crate::BitReader<bool>;
#[doc = "Field `DTEF` reader - data transfer error flag"]
pub type DTEF_R = crate::BitReader<bool>;
#[doc = "Field `ULEF` reader - update link transfer error flag"]
pub type ULEF_R = crate::BitReader<bool>;
#[doc = "Field `USEF` reader - user setting error flag"]
pub type USEF_R = crate::BitReader<bool>;
#[doc = "Field `SUSPF` reader - completed suspension flag"]
pub type SUSPF_R = crate::BitReader<bool>;
#[doc = "Field `TOF` reader - trigger overrun flag"]
pub type TOF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOL` reader - monitored FIFO level Number of available write beats in the FIFO, in units of the programmed destination data width (see GPDMA_CxTR1.DDW_LOG2\\[1:0\\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\\[7:0\\], additionally to GPDMA_CxBR1.BDNT\\[15:0\\]
and GPDMA_CxBR1.BRC\\[10:0\\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (GPDMA_CxSR.SUSPF = 1)."]
pub type FIFOL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - idle flag This idle flag is deasserted by hardware when the channel is enabled (GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state)."]
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - transfer complete flag A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, or a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\])."]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - half transfer flag A half transfer event is either a half block transfer or a half 2D/repeated block transfer, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\]). A half block transfer occurs when half of the bytes of the source block size (rounded up integer of GPDMA_CxBR1.BNDT\\[15:0\\]/2) has been transferred to the destination. A half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (GPDMA_CxBR1.BRC\\[10:0\\]+1)/2)) has been transferred to the destination."]
    #[inline(always)]
    pub fn htf(&self) -> HTF_R {
        HTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - data transfer error flag"]
    #[inline(always)]
    pub fn dtef(&self) -> DTEF_R {
        DTEF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - update link transfer error flag"]
    #[inline(always)]
    pub fn ulef(&self) -> ULEF_R {
        ULEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - user setting error flag"]
    #[inline(always)]
    pub fn usef(&self) -> USEF_R {
        USEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - completed suspension flag"]
    #[inline(always)]
    pub fn suspf(&self) -> SUSPF_R {
        SUSPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - trigger overrun flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - monitored FIFO level Number of available write beats in the FIFO, in units of the programmed destination data width (see GPDMA_CxTR1.DDW_LOG2\\[1:0\\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\\[7:0\\], additionally to GPDMA_CxBR1.BDNT\\[15:0\\]
and GPDMA_CxBR1.BRC\\[10:0\\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (GPDMA_CxSR.SUSPF = 1)."]
    #[inline(always)]
    pub fn fifol(&self) -> FIFOL_R {
        FIFOL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "GPDMA channel 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2sr](index.html) module"]
pub struct C2SR_SPEC;
impl crate::RegisterSpec for C2SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2sr::R](R) reader structure"]
impl crate::Readable for C2SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C2SR to value 0x01"]
impl crate::Resettable for C2SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
