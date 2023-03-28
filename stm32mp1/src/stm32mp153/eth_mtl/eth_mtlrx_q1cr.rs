#[doc = "Register `ETH_MTLRxQ1CR` reader"]
pub struct R(crate::R<ETH_MTLRX_Q1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRX_Q1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRX_Q1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRX_Q1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLRxQ1CR` writer"]
pub struct W(crate::W<ETH_MTLRX_Q1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLRX_Q1CR_SPEC>;
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
impl From<crate::W<ETH_MTLRX_Q1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLRX_Q1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXQ_WEGT` reader - RXQ_WEGT"]
pub type RXQ_WEGT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQ_FRM_ARBIT` reader - RXQ_FRM_ARBIT"]
pub type RXQ_FRM_ARBIT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - RXQ_WEGT"]
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - RXQ_FRM_ARBIT"]
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q1cr](index.html) module"]
pub struct ETH_MTLRX_Q1CR_SPEC;
impl crate::RegisterSpec for ETH_MTLRX_Q1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlrx_q1cr::R](R) reader structure"]
impl crate::Readable for ETH_MTLRX_Q1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtlrx_q1cr::W](W) writer structure"]
impl crate::Writable for ETH_MTLRX_Q1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLRxQ1CR to value 0"]
impl crate::Resettable for ETH_MTLRX_Q1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
