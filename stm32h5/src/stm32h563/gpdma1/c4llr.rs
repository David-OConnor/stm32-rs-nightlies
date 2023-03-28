#[doc = "Register `C4LLR` reader"]
pub struct R(crate::R<C4LLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C4LLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C4LLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C4LLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C4LLR` writer"]
pub struct W(crate::W<C4LLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C4LLR_SPEC>;
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
impl From<crate::W<C4LLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C4LLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
pub type LA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
pub type LA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C4LLR_SPEC, u16, u16, 14, O>;
#[doc = "Field `ULL` reader - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
pub type ULL_R = crate::BitReader<bool>;
#[doc = "Field `ULL` writer - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
pub type ULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4LLR_SPEC, bool, O>;
#[doc = "Field `UDA` reader - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
pub type UDA_R = crate::BitReader<bool>;
#[doc = "Field `UDA` writer - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
pub type UDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4LLR_SPEC, bool, O>;
#[doc = "Field `USA` reader - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
pub type USA_R = crate::BitReader<bool>;
#[doc = "Field `USA` writer - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
pub type USA_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4LLR_SPEC, bool, O>;
#[doc = "Field `UB1` reader - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
pub type UB1_R = crate::BitReader<bool>;
#[doc = "Field `UB1` writer - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
pub type UB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4LLR_SPEC, bool, O>;
#[doc = "Field `UT2` reader - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
pub type UT2_R = crate::BitReader<bool>;
#[doc = "Field `UT2` writer - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
pub type UT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4LLR_SPEC, bool, O>;
#[doc = "Field `UT1` reader - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
pub type UT1_R = crate::BitReader<bool>;
#[doc = "Field `UT1` writer - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
pub type UT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C4LLR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
    #[inline(always)]
    pub fn la(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
    #[inline(always)]
    pub fn ull(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 27 - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
    #[inline(always)]
    pub fn uda(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
    #[inline(always)]
    pub fn usa(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
    #[inline(always)]
    pub fn ub1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
    #[inline(always)]
    pub fn ut2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
    #[inline(always)]
    #[must_use]
    pub fn la(&mut self) -> LA_W<2> {
        LA_W::new(self)
    }
    #[doc = "Bit 16 - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ull(&mut self) -> ULL_W<16> {
        ULL_W::new(self)
    }
    #[doc = "Bit 27 - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn uda(&mut self) -> UDA_W<27> {
        UDA_W::new(self)
    }
    #[doc = "Bit 28 - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn usa(&mut self) -> USA_W<28> {
        USA_W::new(self)
    }
    #[doc = "Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ub1(&mut self) -> UB1_W<29> {
        UB1_W::new(self)
    }
    #[doc = "Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ut2(&mut self) -> UT2_W<30> {
        UT2_W::new(self)
    }
    #[doc = "Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ut1(&mut self) -> UT1_W<31> {
        UT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 4 linked-list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4llr](index.html) module"]
pub struct C4LLR_SPEC;
impl crate::RegisterSpec for C4LLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c4llr::R](R) reader structure"]
impl crate::Readable for C4LLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c4llr::W](W) writer structure"]
impl crate::Writable for C4LLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C4LLR to value 0"]
impl crate::Resettable for C4LLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
