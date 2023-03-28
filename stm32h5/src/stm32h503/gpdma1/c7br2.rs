#[doc = "Register `C7BR2` reader"]
pub struct R(crate::R<C7BR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C7BR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C7BR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C7BR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C7BR2` writer"]
pub struct W(crate::W<C7BR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C7BR2_SPEC>;
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
impl From<crate::W<C7BR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C7BR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRSAO` reader - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\\[15:0\\]
must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
pub type BRSAO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRSAO` writer - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\\[15:0\\]
must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
pub type BRSAO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7BR2_SPEC, u16, u16, 16, O>;
#[doc = "Field `BRDAO` reader - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\\[15:0\\]
must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
pub type BRDAO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRDAO` writer - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\\[15:0\\]
must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
pub type BRDAO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7BR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\\[15:0\\]
must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
    #[inline(always)]
    pub fn brsao(&self) -> BRSAO_R {
        BRSAO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\\[15:0\\]
must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
    #[inline(always)]
    pub fn brdao(&self) -> BRDAO_R {
        BRDAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\\[15:0\\]
must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn brsao(&mut self) -> BRSAO_W<0> {
        BRSAO_W::new(self)
    }
    #[doc = "Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\\[15:0\\]
must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn brdao(&mut self) -> BRDAO_W<16> {
        BRDAO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 7 block register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7br2](index.html) module"]
pub struct C7BR2_SPEC;
impl crate::RegisterSpec for C7BR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c7br2::R](R) reader structure"]
impl crate::Readable for C7BR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c7br2::W](W) writer structure"]
impl crate::Writable for C7BR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C7BR2 to value 0"]
impl crate::Resettable for C7BR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
