#[doc = "Register `ETH_DMACTXDTPR` reader"]
pub struct R(crate::R<ETH_DMACTXDTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMACTXDTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMACTXDTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMACTXDTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMACTXDTPR` writer"]
pub struct W(crate::W<ETH_DMACTXDTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMACTXDTPR_SPEC>;
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
impl From<crate::W<ETH_DMACTXDTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMACTXDTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDT` reader - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers."]
pub type TDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TDT` writer - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers."]
pub type TDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMACTXDTPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers."]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers."]
    #[inline(always)]
    #[must_use]
    pub fn tdt(&mut self) -> TDT_W<0> {
        TDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Tx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmactxdtpr](index.html) module"]
pub struct ETH_DMACTXDTPR_SPEC;
impl crate::RegisterSpec for ETH_DMACTXDTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmactxdtpr::R](R) reader structure"]
impl crate::Readable for ETH_DMACTXDTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmactxdtpr::W](W) writer structure"]
impl crate::Writable for ETH_DMACTXDTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMACTXDTPR to value 0"]
impl crate::Resettable for ETH_DMACTXDTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
