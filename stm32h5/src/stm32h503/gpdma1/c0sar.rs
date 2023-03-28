#[doc = "Register `C0SAR` reader"]
pub struct R(crate::R<C0SAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0SAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0SAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0SAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C0SAR` writer"]
pub struct W(crate::W<C0SAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C0SAR_SPEC>;
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
impl From<crate::W<C0SAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C0SAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SA` reader - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (GPDMA_CxTR1.SINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.SDW_LOG2\\[1:0\\]) after each burst source data, reflecting the next address from which data is read. During the channel activity, this address is updated after each completed source burst, consequently to: the programmed source burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.SINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.SBL_1\\[5:0\\]
and GPDMA_CxTR1.SDW_LOG2\\[21:0\\]
the additional source incremented/decremented offset value as programmed by GPDMA_CxBR1.SDEC and GPDMA_CxTR3.SAO\\[12:0\\]. once/if completed source block transfer, for a channel x with 2D addressing capability (x = 12 to 15). additional block repeat source incremented/decremented offset value as programmed by GPDMA_CxBR1.BRSDEC and GPDMA_CxBR2.BRSAO\\[15:0\\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source burst (SA\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\\[12:0\\]
is not applied."]
pub type SA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SA` writer - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (GPDMA_CxTR1.SINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.SDW_LOG2\\[1:0\\]) after each burst source data, reflecting the next address from which data is read. During the channel activity, this address is updated after each completed source burst, consequently to: the programmed source burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.SINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.SBL_1\\[5:0\\]
and GPDMA_CxTR1.SDW_LOG2\\[21:0\\]
the additional source incremented/decremented offset value as programmed by GPDMA_CxBR1.SDEC and GPDMA_CxTR3.SAO\\[12:0\\]. once/if completed source block transfer, for a channel x with 2D addressing capability (x = 12 to 15). additional block repeat source incremented/decremented offset value as programmed by GPDMA_CxBR1.BRSDEC and GPDMA_CxBR2.BRSAO\\[15:0\\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source burst (SA\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\\[12:0\\]
is not applied."]
pub type SA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C0SAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (GPDMA_CxTR1.SINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.SDW_LOG2\\[1:0\\]) after each burst source data, reflecting the next address from which data is read. During the channel activity, this address is updated after each completed source burst, consequently to: the programmed source burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.SINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.SBL_1\\[5:0\\]
and GPDMA_CxTR1.SDW_LOG2\\[21:0\\]
the additional source incremented/decremented offset value as programmed by GPDMA_CxBR1.SDEC and GPDMA_CxTR3.SAO\\[12:0\\]. once/if completed source block transfer, for a channel x with 2D addressing capability (x = 12 to 15). additional block repeat source incremented/decremented offset value as programmed by GPDMA_CxBR1.BRSDEC and GPDMA_CxBR2.BRSAO\\[15:0\\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source burst (SA\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\\[12:0\\]
is not applied."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (GPDMA_CxTR1.SINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.SDW_LOG2\\[1:0\\]) after each burst source data, reflecting the next address from which data is read. During the channel activity, this address is updated after each completed source burst, consequently to: the programmed source burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.SINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.SBL_1\\[5:0\\]
and GPDMA_CxTR1.SDW_LOG2\\[21:0\\]
the additional source incremented/decremented offset value as programmed by GPDMA_CxBR1.SDEC and GPDMA_CxTR3.SAO\\[12:0\\]. once/if completed source block transfer, for a channel x with 2D addressing capability (x = 12 to 15). additional block repeat source incremented/decremented offset value as programmed by GPDMA_CxBR1.BRSDEC and GPDMA_CxBR2.BRSAO\\[15:0\\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source burst (SA\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\\[12:0\\]
is not applied."]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<0> {
        SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 0 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0sar](index.html) module"]
pub struct C0SAR_SPEC;
impl crate::RegisterSpec for C0SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c0sar::R](R) reader structure"]
impl crate::Readable for C0SAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c0sar::W](W) writer structure"]
impl crate::Writable for C0SAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C0SAR to value 0"]
impl crate::Resettable for C0SAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
