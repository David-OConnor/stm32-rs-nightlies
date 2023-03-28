#[doc = "Register `C3TR2` reader"]
pub struct R(crate::R<C3TR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C3TR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C3TR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C3TR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C3TR2` writer"]
pub struct W(crate::W<C3TR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C3TR2_SPEC>;
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
impl From<crate::W<C3TR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C3TR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQSEL` reader - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
pub type REQSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REQSEL` writer - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
pub type REQSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C3TR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `SWREQ` reader - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
pub type SWREQ_R = crate::BitReader<bool>;
#[doc = "Field `SWREQ` writer - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
pub type SWREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, C3TR2_SPEC, bool, O>;
#[doc = "Field `DREQ` reader - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
pub type DREQ_R = crate::BitReader<bool>;
#[doc = "Field `DREQ` writer - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
pub type DREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, C3TR2_SPEC, bool, O>;
#[doc = "Field `BREQ` reader - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
pub type BREQ_R = crate::BitReader<bool>;
#[doc = "Field `BREQ` writer - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
pub type BREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, C3TR2_SPEC, bool, O>;
#[doc = "Field `PFREQ` reader - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size."]
pub type PFREQ_R = crate::BitReader<bool>;
#[doc = "Field `PFREQ` writer - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size."]
pub type PFREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, C3TR2_SPEC, bool, O>;
#[doc = "Field `TRIGM` reader - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
pub type TRIGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGM` writer - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
pub type TRIGM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C3TR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRIGSEL` reader - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
pub type TRIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGSEL` writer - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
pub type TRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C3TR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `TRIGPOL` reader - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
pub type TRIGPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGPOL` writer - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
pub type TRIGPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C3TR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `TCEM` reader - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1."]
pub type TCEM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCEM` writer - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1."]
pub type TCEM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C3TR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size."]
    #[inline(always)]
    pub fn pfreq(&self) -> PFREQ_R {
        PFREQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1."]
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> REQSEL_W<0> {
        REQSEL_W::new(self)
    }
    #[doc = "Bit 9 - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<9> {
        SWREQ_W::new(self)
    }
    #[doc = "Bit 10 - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DREQ_W<10> {
        DREQ_W::new(self)
    }
    #[doc = "Bit 11 - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
    #[inline(always)]
    #[must_use]
    pub fn breq(&mut self) -> BREQ_W<11> {
        BREQ_W::new(self)
    }
    #[doc = "Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size."]
    #[inline(always)]
    #[must_use]
    pub fn pfreq(&mut self) -> PFREQ_W<12> {
        PFREQ_W::new(self)
    }
    #[doc = "Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
    #[inline(always)]
    #[must_use]
    pub fn trigm(&mut self) -> TRIGM_W<14> {
        TRIGM_W::new(self)
    }
    #[doc = "Bits 16:21 - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<16> {
        TRIGSEL_W::new(self)
    }
    #[doc = "Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<24> {
        TRIGPOL_W::new(self)
    }
    #[doc = "Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1."]
    #[inline(always)]
    #[must_use]
    pub fn tcem(&mut self) -> TCEM_W<30> {
        TCEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 3 transfer register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3tr2](index.html) module"]
pub struct C3TR2_SPEC;
impl crate::RegisterSpec for C3TR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c3tr2::R](R) reader structure"]
impl crate::Readable for C3TR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c3tr2::W](W) writer structure"]
impl crate::Writable for C3TR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C3TR2 to value 0"]
impl crate::Resettable for C3TR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
