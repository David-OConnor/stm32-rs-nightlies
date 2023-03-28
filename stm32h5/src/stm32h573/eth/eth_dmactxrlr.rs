#[doc = "Register `ETH_DMACTXRLR` reader"]
pub struct R(crate::R<ETH_DMACTXRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMACTXRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMACTXRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMACTXRLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMACTXRLR` writer"]
pub struct W(crate::W<ETH_DMACTXRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMACTXRLR_SPEC>;
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
impl From<crate::W<ETH_DMACTXRLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMACTXRLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDRL` reader - Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. It is recommended to put a minimum ring descriptor length of 4. For example, you can program any value up to 0x3FF in this field. This field is 10 bits wide, if you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
pub type TDRL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDRL` writer - Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. It is recommended to put a minimum ring descriptor length of 4. For example, you can program any value up to 0x3FF in this field. This field is 10 bits wide, if you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
pub type TDRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMACTXRLR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. It is recommended to put a minimum ring descriptor length of 4. For example, you can program any value up to 0x3FF in this field. This field is 10 bits wide, if you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. It is recommended to put a minimum ring descriptor length of 4. For example, you can program any value up to 0x3FF in this field. This field is 10 bits wide, if you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
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
#[doc = "Channel Tx descriptor ring length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmactxrlr](index.html) module"]
pub struct ETH_DMACTXRLR_SPEC;
impl crate::RegisterSpec for ETH_DMACTXRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmactxrlr::R](R) reader structure"]
impl crate::Readable for ETH_DMACTXRLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmactxrlr::W](W) writer structure"]
impl crate::Writable for ETH_DMACTXRLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMACTXRLR to value 0"]
impl crate::Resettable for ETH_DMACTXRLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
