#[doc = "Register `DMA_S3FCR` reader"]
pub struct R(crate::R<DMA_S3FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_S3FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_S3FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_S3FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_S3FCR` writer"]
pub struct W(crate::W<DMA_S3FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_S3FCR_SPEC>;
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
impl From<crate::W<DMA_S3FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_S3FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTH` reader - FTH"]
pub type FTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTH` writer - FTH"]
pub type FTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S3FCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMDIS` reader - DMDIS"]
pub type DMDIS_R = crate::BitReader<bool>;
#[doc = "Field `DMDIS` writer - DMDIS"]
pub type DMDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S3FCR_SPEC, bool, O>;
#[doc = "Field `FS` reader - FS"]
pub type FS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEIE` reader - FEIE"]
pub type FEIE_R = crate::BitReader<bool>;
#[doc = "Field `FEIE` writer - FEIE"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S3FCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - FTH"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DMDIS"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FS"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FEIE"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FTH"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<0> {
        FTH_W::new(self)
    }
    #[doc = "Bit 2 - DMDIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmdis(&mut self) -> DMDIS_W<2> {
        DMDIS_W::new(self)
    }
    #[doc = "Bit 7 - FEIE"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<7> {
        FEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA stream 3 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s3fcr](index.html) module"]
pub struct DMA_S3FCR_SPEC;
impl crate::RegisterSpec for DMA_S3FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_s3fcr::R](R) reader structure"]
impl crate::Readable for DMA_S3FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_s3fcr::W](W) writer structure"]
impl crate::Writable for DMA_S3FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_S3FCR to value 0x21"]
impl crate::Resettable for DMA_S3FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
