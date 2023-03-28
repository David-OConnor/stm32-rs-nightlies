#[doc = "Register `HASH_SR` reader"]
pub struct R(crate::R<HASH_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_SR` writer"]
pub struct W(crate::W<HASH_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_SR_SPEC>;
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
impl From<crate::W<HASH_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DINIS` reader - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
pub type DINIS_R = crate::BitReader<bool>;
#[doc = "Field `DINIS` writer - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
pub type DINIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_SR_SPEC, bool, O>;
#[doc = "Field `DCIS` reader - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
pub type DCIS_R = crate::BitReader<bool>;
#[doc = "Field `DCIS` writer - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
pub type DCIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_SR_SPEC, bool, O>;
#[doc = "Field `DMAS` reader - DMA Status This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE = 0 and no DMA transfer is ongoing. No interrupt is associated with this bit."]
pub type DMAS_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - Busy bit"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `NBWP` reader - Number of words already pushed This bitfield is the exact number of words in the message that have already been pushed into the FIFO. NBWP is incremented by 1 when a write access is performed to the HASH_DIN register. When a digest calculation starts, NBWP is updated to NBWP- block size (in words), and NBWP goes to zero when the INIT bit is written to 1."]
pub type NBWP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINNE` reader - DIN not empty This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1."]
pub type DINNE_R = crate::BitReader<bool>;
#[doc = "Field `NBWE` reader - Number of words expected This bitfield reflects the number of words in the message that must be pushed into the FIFO to trigger a partial computation. NBWE is decremented by 1 when a write access is performed to the HASH_DIN register. NBWE is set to the expected block size +1 in words (0x11) when INIT bit is set in HASH_CR. It is set to the expected block size (0x10) when the partial digest calculation ends."]
pub type NBWE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
    #[inline(always)]
    pub fn dinis(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
    #[inline(always)]
    pub fn dcis(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Status This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE = 0 and no DMA transfer is ongoing. No interrupt is associated with this bit."]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 9:13 - Number of words already pushed This bitfield is the exact number of words in the message that have already been pushed into the FIFO. NBWP is incremented by 1 when a write access is performed to the HASH_DIN register. When a digest calculation starts, NBWP is updated to NBWP- block size (in words), and NBWP goes to zero when the INIT bit is written to 1."]
    #[inline(always)]
    pub fn nbwp(&self) -> NBWP_R {
        NBWP_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - DIN not empty This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1."]
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Number of words expected This bitfield reflects the number of words in the message that must be pushed into the FIFO to trigger a partial computation. NBWE is decremented by 1 when a write access is performed to the HASH_DIN register. NBWE is set to the expected block size +1 in words (0x11) when INIT bit is set in HASH_CR. It is set to the expected block size (0x10) when the partial digest calculation ends."]
    #[inline(always)]
    pub fn nbwe(&self) -> NBWE_R {
        NBWE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
    #[inline(always)]
    #[must_use]
    pub fn dinis(&mut self) -> DINIS_W<0> {
        DINIS_W::new(self)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn dcis(&mut self) -> DCIS_W<1> {
        DCIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_sr](index.html) module"]
pub struct HASH_SR_SPEC;
impl crate::RegisterSpec for HASH_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_sr::R](R) reader structure"]
impl crate::Readable for HASH_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_sr::W](W) writer structure"]
impl crate::Writable for HASH_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_SR to value 0x0011_0001"]
impl crate::Resettable for HASH_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_0001;
}
