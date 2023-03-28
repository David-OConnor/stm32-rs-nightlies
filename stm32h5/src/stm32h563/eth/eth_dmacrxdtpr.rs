#[doc = "Register `ETH_DMACRXDTPR` reader"]
pub struct R(crate::R<ETH_DMACRXDTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMACRXDTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMACRXDTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMACRXDTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMACRXDTPR` writer"]
pub struct W(crate::W<ETH_DMACRXDTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMACRXDTPR_SPEC>;
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
impl From<crate::W<ETH_DMACRXDTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMACRXDTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDT` reader - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers."]
pub type RDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RDT` writer - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers."]
pub type RDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMACRXDTPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers."]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers."]
    #[inline(always)]
    #[must_use]
    pub fn rdt(&mut self) -> RDT_W<0> {
        RDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Rx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmacrxdtpr](index.html) module"]
pub struct ETH_DMACRXDTPR_SPEC;
impl crate::RegisterSpec for ETH_DMACRXDTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmacrxdtpr::R](R) reader structure"]
impl crate::Readable for ETH_DMACRXDTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmacrxdtpr::W](W) writer structure"]
impl crate::Writable for ETH_DMACRXDTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMACRXDTPR to value 0"]
impl crate::Resettable for ETH_DMACRXDTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
