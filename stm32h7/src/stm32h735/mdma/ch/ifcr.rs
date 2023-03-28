#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEIF` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type CTEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CCTCIF` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type CCTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CBRTIF` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type CBRTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CBTIF` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type CBTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CLTCIF` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type CLTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<0> {
        CTEIF_W::new(self)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cctcif(&mut self) -> CCTCIF_W<1> {
        CCTCIF_W::new(self)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbrtif(&mut self) -> CBRTIF_W<2> {
        CBRTIF_W::new(self)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbtif(&mut self) -> CBTIF_W<3> {
        CBTIF_W::new(self)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cltcif(&mut self) -> CLTCIF_W<4> {
        CLTCIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
