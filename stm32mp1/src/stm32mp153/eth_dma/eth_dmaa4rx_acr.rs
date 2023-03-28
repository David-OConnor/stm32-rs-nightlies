#[doc = "Register `ETH_DMAA4RxACR` reader"]
pub struct R(crate::R<ETH_DMAA4RX_ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAA4RX_ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAA4RX_ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAA4RX_ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAA4RxACR` writer"]
pub struct W(crate::W<ETH_DMAA4RX_ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAA4RX_ACR_SPEC>;
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
impl From<crate::W<ETH_DMAA4RX_ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAA4RX_ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDWC` reader - RDWC"]
pub type RDWC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDWC` writer - RDWC"]
pub type RDWC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4RX_ACR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RPC` reader - RPC"]
pub type RPC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPC` writer - RPC"]
pub type RPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4RX_ACR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RHC` reader - RHC"]
pub type RHC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RHC` writer - RHC"]
pub type RHC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4RX_ACR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RDC` reader - RDC"]
pub type RDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDC` writer - RDC"]
pub type RDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4RX_ACR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - RDWC"]
    #[inline(always)]
    pub fn rdwc(&self) -> RDWC_R {
        RDWC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RPC"]
    #[inline(always)]
    pub fn rpc(&self) -> RPC_R {
        RPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RHC"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - RDC"]
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RDWC"]
    #[inline(always)]
    #[must_use]
    pub fn rdwc(&mut self) -> RDWC_W<0> {
        RDWC_W::new(self)
    }
    #[doc = "Bits 8:11 - RPC"]
    #[inline(always)]
    #[must_use]
    pub fn rpc(&mut self) -> RPC_W<8> {
        RPC_W::new(self)
    }
    #[doc = "Bits 16:19 - RHC"]
    #[inline(always)]
    #[must_use]
    pub fn rhc(&mut self) -> RHC_W<16> {
        RHC_W::new(self)
    }
    #[doc = "Bits 24:25 - RDC"]
    #[inline(always)]
    #[must_use]
    pub fn rdc(&mut self) -> RDC_W<24> {
        RDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI4 receive channel ACE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaa4rx_acr](index.html) module"]
pub struct ETH_DMAA4RX_ACR_SPEC;
impl crate::RegisterSpec for ETH_DMAA4RX_ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaa4rx_acr::R](R) reader structure"]
impl crate::Readable for ETH_DMAA4RX_ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmaa4rx_acr::W](W) writer structure"]
impl crate::Writable for ETH_DMAA4RX_ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAA4RxACR to value 0"]
impl crate::Resettable for ETH_DMAA4RX_ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
