#[doc = "Register `ETH_MTLTxQ1ECR` reader"]
pub struct R(crate::R<ETH_MTLTX_Q1ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTX_Q1ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTX_Q1ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTX_Q1ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTxQ1ECR` writer"]
pub struct W(crate::W<ETH_MTLTX_Q1ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTX_Q1ECR_SPEC>;
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
impl From<crate::W<ETH_MTLTX_Q1ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTX_Q1ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVALG` reader - AVALG"]
pub type AVALG_R = crate::BitReader<bool>;
#[doc = "Field `AVALG` writer - AVALG"]
pub type AVALG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLTX_Q1ECR_SPEC, bool, O>;
#[doc = "Field `CC` reader - CC"]
pub type CC_R = crate::BitReader<bool>;
#[doc = "Field `CC` writer - CC"]
pub type CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLTX_Q1ECR_SPEC, bool, O>;
#[doc = "Field `SLC` reader - SLC"]
pub type SLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLC` writer - SLC"]
pub type SLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLTX_Q1ECR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 2 - AVALG"]
    #[inline(always)]
    pub fn avalg(&self) -> AVALG_R {
        AVALG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SLC"]
    #[inline(always)]
    pub fn slc(&self) -> SLC_R {
        SLC_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - AVALG"]
    #[inline(always)]
    #[must_use]
    pub fn avalg(&mut self) -> AVALG_W<2> {
        AVALG_W::new(self)
    }
    #[doc = "Bit 3 - CC"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<3> {
        CC_W::new(self)
    }
    #[doc = "Bits 4:6 - SLC"]
    #[inline(always)]
    #[must_use]
    pub fn slc(&mut self) -> SLC_W<4> {
        SLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Queue ETS Control register controls the enhanced transmission selection operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1ecr](index.html) module"]
pub struct ETH_MTLTX_Q1ECR_SPEC;
impl crate::RegisterSpec for ETH_MTLTX_Q1ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q1ecr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q1ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1ecr::W](W) writer structure"]
impl crate::Writable for ETH_MTLTX_Q1ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1ECR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q1ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
