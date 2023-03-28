#[doc = "Register `C6BR1` reader"]
pub struct R(crate::R<C6BR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C6BR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C6BR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C6BR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C6BR1` writer"]
pub struct W(crate::W<C6BR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C6BR1_SPEC>;
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
impl From<crate::W<C6BR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C6BR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNDT` reader - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: When configured in packing mode (GPDMA_CxTR1.PAM\\[1\\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
pub type BNDT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BNDT` writer - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: When configured in packing mode (GPDMA_CxTR1.PAM\\[1\\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
pub type BNDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C6BR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `BRC` reader - Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\\[10:0\\]
= BNDT\\[15:0\\]
= 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer."]
pub type BRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRC` writer - Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\\[10:0\\]
= BNDT\\[15:0\\]
= 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer."]
pub type BRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C6BR1_SPEC, u16, u16, 11, O>;
#[doc = "Field `SDEC` reader - source address decrement"]
pub type SDEC_R = crate::BitReader<bool>;
#[doc = "Field `SDEC` writer - source address decrement"]
pub type SDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C6BR1_SPEC, bool, O>;
#[doc = "Field `DDEC` reader - destination address decrement"]
pub type DDEC_R = crate::BitReader<bool>;
#[doc = "Field `DDEC` writer - destination address decrement"]
pub type DDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C6BR1_SPEC, bool, O>;
#[doc = "Field `BRSDEC` reader - Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer."]
pub type BRSDEC_R = crate::BitReader<bool>;
#[doc = "Field `BRSDEC` writer - Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer."]
pub type BRSDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C6BR1_SPEC, bool, O>;
#[doc = "Field `BRDDEC` reader - Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer."]
pub type BRDDEC_R = crate::BitReader<bool>;
#[doc = "Field `BRDDEC` writer - Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer."]
pub type BRDDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C6BR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: When configured in packing mode (GPDMA_CxTR1.PAM\\[1\\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\\[10:0\\]
= BNDT\\[15:0\\]
= 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer."]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - source address decrement"]
    #[inline(always)]
    pub fn sdec(&self) -> SDEC_R {
        SDEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - destination address decrement"]
    #[inline(always)]
    pub fn ddec(&self) -> DDEC_R {
        DDEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer."]
    #[inline(always)]
    pub fn brsdec(&self) -> BRSDEC_R {
        BRSDEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer."]
    #[inline(always)]
    pub fn brddec(&self) -> BRDDEC_R {
        BRDDEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: When configured in packing mode (GPDMA_CxTR1.PAM\\[1\\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<0> {
        BNDT_W::new(self)
    }
    #[doc = "Bits 16:26 - Block repeat counter This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\\[10:0\\]
= BNDT\\[15:0\\]
= 0): If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the memory. If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn brc(&mut self) -> BRC_W<16> {
        BRC_W::new(self)
    }
    #[doc = "Bit 28 - source address decrement"]
    #[inline(always)]
    #[must_use]
    pub fn sdec(&mut self) -> SDEC_W<28> {
        SDEC_W::new(self)
    }
    #[doc = "Bit 29 - destination address decrement"]
    #[inline(always)]
    #[must_use]
    pub fn ddec(&mut self) -> DDEC_W<29> {
        DDEC_W::new(self)
    }
    #[doc = "Bit 30 - Block repeat source address decrement Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the same time also updated by the increment/decrement (depending on SDEC) of the GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer."]
    #[inline(always)]
    #[must_use]
    pub fn brsdec(&mut self) -> BRSDEC_W<30> {
        BRSDEC_W::new(self)
    }
    #[doc = "Bit 31 - Block repeat destination address decrement Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the same time also updated by the increment/decrement (depending on DDEC) of the GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed burst transfer."]
    #[inline(always)]
    #[must_use]
    pub fn brddec(&mut self) -> BRDDEC_W<31> {
        BRDDEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 6 alternate block register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6br1](index.html) module"]
pub struct C6BR1_SPEC;
impl crate::RegisterSpec for C6BR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c6br1::R](R) reader structure"]
impl crate::Readable for C6BR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c6br1::W](W) writer structure"]
impl crate::Writable for C6BR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C6BR1 to value 0"]
impl crate::Resettable for C6BR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
