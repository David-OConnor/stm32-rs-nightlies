#[doc = "Register `RCC_MP_AHB2ENCLRR` reader"]
pub struct R(crate::R<RCC_MP_AHB2ENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB2ENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB2ENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB2ENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_AHB2ENCLRR` writer"]
pub struct W(crate::W<RCC_MP_AHB2ENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB2ENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB2ENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB2ENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1EN` reader - DMA1EN"]
pub type DMA1EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1EN` writer - DMA1EN"]
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB2ENCLRR_SPEC, bool, O>;
#[doc = "Field `DMA2EN` reader - DMA2EN"]
pub type DMA2EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2EN` writer - DMA2EN"]
pub type DMA2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB2ENCLRR_SPEC, bool, O>;
#[doc = "Field `DMAMUXEN` reader - DMAMUXEN"]
pub type DMAMUXEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUXEN` writer - DMAMUXEN"]
pub type DMAMUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB2ENCLRR_SPEC, bool, O>;
#[doc = "Field `ADC12EN` reader - ADC12EN"]
pub type ADC12EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC12EN` writer - ADC12EN"]
pub type ADC12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB2ENCLRR_SPEC, bool, O>;
#[doc = "Field `USBOEN` reader - USBOEN"]
pub type USBOEN_R = crate::BitReader<bool>;
#[doc = "Field `USBOEN` writer - USBOEN"]
pub type USBOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB2ENCLRR_SPEC, bool, O>;
#[doc = "Field `SDMMC3EN` reader - SDMMC3EN"]
pub type SDMMC3EN_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC3EN` writer - SDMMC3EN"]
pub type SDMMC3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB2ENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA1EN"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2EN"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUXEN"]
    #[inline(always)]
    pub fn dmamuxen(&self) -> DMAMUXEN_R {
        DMAMUXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12EN"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USBOEN"]
    #[inline(always)]
    pub fn usboen(&self) -> USBOEN_R {
        USBOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC3EN"]
    #[inline(always)]
    pub fn sdmmc3en(&self) -> SDMMC3EN_R {
        SDMMC3EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1EN"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2EN"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - DMAMUXEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxen(&mut self) -> DMAMUXEN_W<2> {
        DMAMUXEN_W::new(self)
    }
    #[doc = "Bit 5 - ADC12EN"]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<5> {
        ADC12EN_W::new(self)
    }
    #[doc = "Bit 8 - USBOEN"]
    #[inline(always)]
    #[must_use]
    pub fn usboen(&mut self) -> USBOEN_W<8> {
        USBOEN_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC3EN"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc3en(&mut self) -> SDMMC3EN_W<16> {
        SDMMC3EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb2enclrr](index.html) module"]
pub struct RCC_MP_AHB2ENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB2ENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_ahb2enclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_AHB2ENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb2enclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_AHB2ENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_AHB2ENCLRR to value 0"]
impl crate::Resettable for RCC_MP_AHB2ENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
