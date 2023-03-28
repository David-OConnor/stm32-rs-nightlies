#[doc = "Register `DMACTxRLR` reader"]
pub struct R(crate::R<DMACTX_RLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTX_RLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTX_RLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTX_RLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTxRLR` writer"]
pub struct W(crate::W<DMACTX_RLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTX_RLR_SPEC>;
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
impl From<crate::W<DMACTX_RLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTX_RLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDRL` reader - Transmit Descriptor Ring Length"]
pub type TDRL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDRL` writer - Transmit Descriptor Ring Length"]
pub type TDRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACTX_RLR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length"]
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length"]
    #[inline(always)]
    #[must_use]
    pub fn tdrl(&mut self) -> TDRL_W<0> {
        TDRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Tx descriptor ring length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactx_rlr](index.html) module"]
pub struct DMACTX_RLR_SPEC;
impl crate::RegisterSpec for DMACTX_RLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactx_rlr::R](R) reader structure"]
impl crate::Readable for DMACTX_RLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactx_rlr::W](W) writer structure"]
impl crate::Writable for DMACTX_RLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTxRLR to value 0"]
impl crate::Resettable for DMACTX_RLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
