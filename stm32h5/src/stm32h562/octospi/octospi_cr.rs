#[doc = "Register `OCTOSPI_CR` reader"]
pub struct R(crate::R<OCTOSPI_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_CR` writer"]
pub struct W(crate::W<OCTOSPI_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_CR_SPEC>;
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
impl From<crate::W<OCTOSPI_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable This bit enables the OCTOSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable This bit enables the OCTOSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `ABORT` reader - Abort request This bit aborts the ongoing command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0."]
pub type ABORT_R = crate::BitReader<bool>;
#[doc = "Field `ABORT` writer - Abort request This bit aborts the ongoing command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0."]
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMA enable In Indirect mode, the DMA can be used to input or output data via OCTOSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation."]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable In Indirect mode, the DMA can be used to input or output data via OCTOSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation."]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `TCEN` reader - Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\\[1:0\\]
= 11) is selected. This bit enables the timeout counter."]
pub type TCEN_R = crate::BitReader<bool>;
#[doc = "Field `TCEN` writer - Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\\[1:0\\]
= 11) is selected. This bit enables the timeout counter."]
pub type TCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `DMM` reader - Dual-memory configuration This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity"]
pub type DMM_R = crate::BitReader<bool>;
#[doc = "Field `DMM` writer - Dual-memory configuration This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity"]
pub type DMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `FSEL` reader - Flash select This bit selects the Flash memory to be addressed in Single-, Dual-, Quad-SPI mode in single-memory configuration (when DMM = 0). This bit is ignored when DMM = 1 or when Octal-SPI mode is selected."]
pub type FSEL_R = crate::BitReader<bool>;
#[doc = "Field `FSEL` writer - Flash select This bit selects the Flash memory to be addressed in Single-, Dual-, Quad-SPI mode in single-memory configuration (when DMM = 0). This bit is ignored when DMM = 1 or when Octal-SPI mode is selected."]
pub type FSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `FTHRES` reader - FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in OCTOSPI_SR, to be set. ... Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[4:0\\]
value."]
pub type FTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTHRES` writer - FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in OCTOSPI_SR, to be set. ... Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[4:0\\]
value."]
pub type FTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_CR_SPEC, u8, u8, 5, O>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit enables the transfer error interrupt."]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit enables the transfer error interrupt."]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `FTIE` reader - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
pub type FTIE_R = crate::BitReader<bool>;
#[doc = "Field `FTIE` writer - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
pub type FTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `SMIE` reader - Status match interrupt enable This bit enables the status match interrupt."]
pub type SMIE_R = crate::BitReader<bool>;
#[doc = "Field `SMIE` writer - Status match interrupt enable This bit enables the status match interrupt."]
pub type SMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `TOIE` reader - Timeout interrupt enable This bit enables the timeout interrupt."]
pub type TOIE_R = crate::BitReader<bool>;
#[doc = "Field `TOIE` writer - Timeout interrupt enable This bit enables the timeout interrupt."]
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `APMS` reader - Automatic status-polling mode stop This bit determines if the Automatic status-polling mode is stopped after a match."]
pub type APMS_R = crate::BitReader<bool>;
#[doc = "Field `APMS` writer - Automatic status-polling mode stop This bit determines if the Automatic status-polling mode is stopped after a match."]
pub type APMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `PMM` reader - Polling match mode This bit indicates which method must be used to determine a match during the Automatic status-polling mode."]
pub type PMM_R = crate::BitReader<bool>;
#[doc = "Field `PMM` writer - Polling match mode This bit indicates which method must be used to determine a match during the Automatic status-polling mode."]
pub type PMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_CR_SPEC, bool, O>;
#[doc = "Field `FMODE` reader - Functional mode This field defines the OCTOSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state."]
pub type FMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMODE` writer - Functional mode This field defines the OCTOSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state."]
pub type FMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Enable This bit enables the OCTOSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort request This bit aborts the ongoing command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA enable In Indirect mode, the DMA can be used to input or output data via OCTOSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\\[1:0\\]
= 11) is selected. This bit enables the timeout counter."]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Dual-memory configuration This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity"]
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flash select This bit selects the Flash memory to be addressed in Single-, Dual-, Quad-SPI mode in single-memory configuration (when DMM = 0). This bit is ignored when DMM = 1 or when Octal-SPI mode is selected."]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in OCTOSPI_SR, to be set. ... Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[4:0\\]
value."]
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Status match interrupt enable This bit enables the status match interrupt."]
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timeout interrupt enable This bit enables the timeout interrupt."]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Automatic status-polling mode stop This bit determines if the Automatic status-polling mode is stopped after a match."]
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Polling match mode This bit indicates which method must be used to determine a match during the Automatic status-polling mode."]
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Functional mode This field defines the OCTOSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state."]
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable This bit enables the OCTOSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Abort request This bit aborts the ongoing command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<1> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 2 - DMA enable In Indirect mode, the DMA can be used to input or output data via OCTOSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<2> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 3 - Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\\[1:0\\]
= 11) is selected. This bit enables the timeout counter."]
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<3> {
        TCEN_W::new(self)
    }
    #[doc = "Bit 6 - Dual-memory configuration This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity"]
    #[inline(always)]
    #[must_use]
    pub fn dmm(&mut self) -> DMM_W<6> {
        DMM_W::new(self)
    }
    #[doc = "Bit 7 - Flash select This bit selects the Flash memory to be addressed in Single-, Dual-, Quad-SPI mode in single-memory configuration (when DMM = 0). This bit is ignored when DMM = 1 or when Octal-SPI mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<7> {
        FSEL_W::new(self)
    }
    #[doc = "Bits 8:12 - FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in OCTOSPI_SR, to be set. ... Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[4:0\\]
value."]
    #[inline(always)]
    #[must_use]
    pub fn fthres(&mut self) -> FTHRES_W<8> {
        FTHRES_W::new(self)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<16> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<17> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<18> {
        FTIE_W::new(self)
    }
    #[doc = "Bit 19 - Status match interrupt enable This bit enables the status match interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SMIE_W<19> {
        SMIE_W::new(self)
    }
    #[doc = "Bit 20 - Timeout interrupt enable This bit enables the timeout interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<20> {
        TOIE_W::new(self)
    }
    #[doc = "Bit 22 - Automatic status-polling mode stop This bit determines if the Automatic status-polling mode is stopped after a match."]
    #[inline(always)]
    #[must_use]
    pub fn apms(&mut self) -> APMS_W<22> {
        APMS_W::new(self)
    }
    #[doc = "Bit 23 - Polling match mode This bit indicates which method must be used to determine a match during the Automatic status-polling mode."]
    #[inline(always)]
    #[must_use]
    pub fn pmm(&mut self) -> PMM_W<23> {
        PMM_W::new(self)
    }
    #[doc = "Bits 28:29 - Functional mode This field defines the OCTOSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state."]
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
#[doc = "OCTOSPI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_cr](index.html) module"]
pub struct OCTOSPI_CR_SPEC;
impl crate::RegisterSpec for OCTOSPI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_cr::R](R) reader structure"]
impl crate::Readable for OCTOSPI_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_cr::W](W) writer structure"]
impl crate::Writable for OCTOSPI_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_CR to value 0"]
impl crate::Resettable for OCTOSPI_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
