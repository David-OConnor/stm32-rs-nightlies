#[doc = "Register `ETH_MTLTxQ1QWR` reader"]
pub struct R(crate::R<ETH_MTLTX_Q1QWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTX_Q1QWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTX_Q1QWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTX_Q1QWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTxQ1QWR` writer"]
pub struct W(crate::W<ETH_MTLTX_Q1QWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTX_Q1QWR_SPEC>;
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
impl From<crate::W<ETH_MTLTX_Q1QWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTX_Q1QWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISCQW` reader - ISCQW"]
pub type ISCQW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISCQW` writer - ISCQW"]
pub type ISCQW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MTLTX_Q1QWR_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - ISCQW"]
    #[inline(always)]
    pub fn iscqw(&self) -> ISCQW_R {
        ISCQW_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - ISCQW"]
    #[inline(always)]
    #[must_use]
    pub fn iscqw(&mut self) -> ISCQW_W<0> {
        ISCQW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides the average traffic transmitted on queue 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1qwr](index.html) module"]
pub struct ETH_MTLTX_Q1QWR_SPEC;
impl crate::RegisterSpec for ETH_MTLTX_Q1QWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q1qwr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q1QWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1qwr::W](W) writer structure"]
impl crate::Writable for ETH_MTLTX_Q1QWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1QWR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q1QWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
