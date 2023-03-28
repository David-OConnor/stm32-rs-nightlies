#[doc = "Register `C1CR` reader"]
pub struct R(crate::R<C1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1CR` writer"]
pub struct W(crate::W<C1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1CR_SPEC>;
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
impl From<crate::W<C1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `RESET` writer - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in )."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - transfer complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `HTIE` reader - half transfer complete interrupt enable"]
pub type HTIE_R = crate::BitReader<bool>;
#[doc = "Field `HTIE` writer - half transfer complete interrupt enable"]
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `DTEIE` reader - data transfer error interrupt enable"]
pub type DTEIE_R = crate::BitReader<bool>;
#[doc = "Field `DTEIE` writer - data transfer error interrupt enable"]
pub type DTEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `ULEIE` reader - update link transfer error interrupt enable"]
pub type ULEIE_R = crate::BitReader<bool>;
#[doc = "Field `ULEIE` writer - update link transfer error interrupt enable"]
pub type ULEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `USEIE` reader - user setting error interrupt enable"]
pub type USEIE_R = crate::BitReader<bool>;
#[doc = "Field `USEIE` writer - user setting error interrupt enable"]
pub type USEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `SUSPIE` reader - completed suspension interrupt enable"]
pub type SUSPIE_R = crate::BitReader<bool>;
#[doc = "Field `SUSPIE` writer - completed suspension interrupt enable"]
pub type SUSPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `TOIE` reader - trigger overrun interrupt enable"]
pub type TOIE_R = crate::BitReader<bool>;
#[doc = "Field `TOIE` writer - trigger overrun interrupt enable"]
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `LSM` reader - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LSM_R = crate::BitReader<bool>;
#[doc = "Field `LSM` writer - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `LAP` reader - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LAP_R = crate::BitReader<bool>;
#[doc = "Field `LAP` writer - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `PRIO` reader - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C1CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - half transfer complete interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - data transfer error interrupt enable"]
    #[inline(always)]
    pub fn dteie(&self) -> DTEIE_R {
        DTEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - update link transfer error interrupt enable"]
    #[inline(always)]
    pub fn uleie(&self) -> ULEIE_R {
        ULEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - user setting error interrupt enable"]
    #[inline(always)]
    pub fn useie(&self) -> USEIE_R {
        USEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - completed suspension interrupt enable"]
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn lap(&self) -> LAP_R {
        LAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 22:23 - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in )."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<1> {
        RESET_W::new(self)
    }
    #[doc = "Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<2> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 8 - transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<8> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 9 - half transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<9> {
        HTIE_W::new(self)
    }
    #[doc = "Bit 10 - data transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dteie(&mut self) -> DTEIE_W<10> {
        DTEIE_W::new(self)
    }
    #[doc = "Bit 11 - update link transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uleie(&mut self) -> ULEIE_W<11> {
        ULEIE_W::new(self)
    }
    #[doc = "Bit 12 - user setting error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn useie(&mut self) -> USEIE_W<12> {
        USEIE_W::new(self)
    }
    #[doc = "Bit 13 - completed suspension interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn suspie(&mut self) -> SUSPIE_W<13> {
        SUSPIE_W::new(self)
    }
    #[doc = "Bit 14 - trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<14> {
        TOIE_W::new(self)
    }
    #[doc = "Bit 16 - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<16> {
        LSM_W::new(self)
    }
    #[doc = "Bit 17 - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    #[must_use]
    pub fn lap(&mut self) -> LAP_W<17> {
        LAP_W::new(self)
    }
    #[doc = "Bits 22:23 - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<22> {
        PRIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1cr](index.html) module"]
pub struct C1CR_SPEC;
impl crate::RegisterSpec for C1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1cr::R](R) reader structure"]
impl crate::Readable for C1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1cr::W](W) writer structure"]
impl crate::Writable for C1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1CR to value 0"]
impl crate::Resettable for C1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
