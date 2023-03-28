#[doc = "Register `TIM17_DMAR` reader"]
pub struct R(crate::R<TIM17_DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM17_DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM17_DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM17_DMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM17_DMAR` writer"]
pub struct W(crate::W<TIM17_DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM17_DMAR_SPEC>;
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
impl From<crate::W<TIM17_DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM17_DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAB` reader - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
pub type DMAB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAB` writer - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
pub type DMAB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM17_DMAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<0> {
        DMAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim17_dmar](index.html) module"]
pub struct TIM17_DMAR_SPEC;
impl crate::RegisterSpec for TIM17_DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim17_dmar::R](R) reader structure"]
impl crate::Readable for TIM17_DMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim17_dmar::W](W) writer structure"]
impl crate::Writable for TIM17_DMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM17_DMAR to value 0"]
impl crate::Resettable for TIM17_DMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
