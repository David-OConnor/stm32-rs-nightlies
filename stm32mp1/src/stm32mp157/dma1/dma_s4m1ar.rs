#[doc = "Register `DMA_S4M1AR` reader"]
pub struct R(crate::R<DMA_S4M1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_S4M1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_S4M1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_S4M1AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_S4M1AR` writer"]
pub struct W(crate::W<DMA_S4M1AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_S4M1AR_SPEC>;
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
impl From<crate::W<DMA_S4M1AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_S4M1AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M1A` reader - M1A"]
pub type M1A_R = crate::FieldReader<u32, u32>;
#[doc = "Field `M1A` writer - M1A"]
pub type M1A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S4M1AR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - M1A"]
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - M1A"]
    #[inline(always)]
    #[must_use]
    pub fn m1a(&mut self) -> M1A_W<0> {
        M1A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA stream 4 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s4m1ar](index.html) module"]
pub struct DMA_S4M1AR_SPEC;
impl crate::RegisterSpec for DMA_S4M1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_s4m1ar::R](R) reader structure"]
impl crate::Readable for DMA_S4M1AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_s4m1ar::W](W) writer structure"]
impl crate::Writable for DMA_S4M1AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_S4M1AR to value 0"]
impl crate::Resettable for DMA_S4M1AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
