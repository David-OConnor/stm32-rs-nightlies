#[doc = "Register `ETH_DMACTXDLAR` reader"]
pub struct R(crate::R<ETH_DMACTXDLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMACTXDLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMACTXDLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMACTXDLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMACTXDLAR` writer"]
pub struct W(crate::W<ETH_DMACTXDLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMACTXDLAR_SPEC>;
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
impl From<crate::W<ETH_DMACTXDLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMACTXDLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDESLA` reader - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
pub type TDESLA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TDESLA` writer - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
pub type TDESLA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_DMACTXDLAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    #[must_use]
    pub fn tdesla(&mut self) -> TDESLA_W<0> {
        TDESLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Tx descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmactxdlar](index.html) module"]
pub struct ETH_DMACTXDLAR_SPEC;
impl crate::RegisterSpec for ETH_DMACTXDLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmactxdlar::R](R) reader structure"]
impl crate::Readable for ETH_DMACTXDLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmactxdlar::W](W) writer structure"]
impl crate::Writable for ETH_DMACTXDLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMACTXDLAR to value 0"]
impl crate::Resettable for ETH_DMACTXDLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
