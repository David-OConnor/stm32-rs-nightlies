#[doc = "Register `TIM15_DCR` reader"]
pub struct R(crate::R<TIM15_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM15_DCR` writer"]
pub struct W(crate::W<TIM15_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_DCR_SPEC>;
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
impl From<crate::W<TIM15_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBA` reader - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIM15_DMAR address). DBA is defined as an offset starting from the address of the TIM15_CR1 register. Example: ..."]
pub type DBA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBA` writer - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIM15_DMAR address). DBA is defined as an offset starting from the address of the TIM15_CR1 register. Example: ..."]
pub type DBA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_DCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DBL` reader - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIM15_DMAR address). ..."]
pub type DBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBL` writer - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIM15_DMAR address). ..."]
pub type DBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_DCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DBSS` reader - DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Other: reserved"]
pub type DBSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBSS` writer - DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Other: reserved"]
pub type DBSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_DCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIM15_DMAR address). DBA is defined as an offset starting from the address of the TIM15_CR1 register. Example: ..."]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIM15_DMAR address). ..."]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Other: reserved"]
    #[inline(always)]
    pub fn dbss(&self) -> DBSS_R {
        DBSS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIM15_DMAR address). DBA is defined as an offset starting from the address of the TIM15_CR1 register. Example: ..."]
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DBA_W<0> {
        DBA_W::new(self)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIM15_DMAR address). ..."]
    #[inline(always)]
    #[must_use]
    pub fn dbl(&mut self) -> DBL_W<8> {
        DBL_W::new(self)
    }
    #[doc = "Bits 16:19 - DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Other: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dbss(&mut self) -> DBSS_W<16> {
        DBSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_dcr](index.html) module"]
pub struct TIM15_DCR_SPEC;
impl crate::RegisterSpec for TIM15_DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim15_dcr::R](R) reader structure"]
impl crate::Readable for TIM15_DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim15_dcr::W](W) writer structure"]
impl crate::Writable for TIM15_DCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM15_DCR to value 0"]
impl crate::Resettable for TIM15_DCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
