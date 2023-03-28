#[doc = "Register `RCC_AHB2RSTCLRR` reader"]
pub struct R(crate::R<RCC_AHB2RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB2RSTCLRR` writer"]
pub struct W(crate::W<RCC_AHB2RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_AHB2RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1RST` reader - DMA1RST"]
pub type DMA1RST_R = crate::BitReader<bool>;
#[doc = "Field `DMA1RST` writer - DMA1RST"]
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `DMA2RST` reader - DMA2RST"]
pub type DMA2RST_R = crate::BitReader<bool>;
#[doc = "Field `DMA2RST` writer - DMA2RST"]
pub type DMA2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `DMAMUXRST` reader - DMAMUXRST"]
pub type DMAMUXRST_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUXRST` writer - DMAMUXRST"]
pub type DMAMUXRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `ADC12RST` reader - ADC12RST"]
pub type ADC12RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC12RST` writer - ADC12RST"]
pub type ADC12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `USBORST` reader - USBORST"]
pub type USBORST_R = crate::BitReader<bool>;
#[doc = "Field `USBORST` writer - USBORST"]
pub type USBORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SDMMC3RST` reader - SDMMC3RST"]
pub type SDMMC3RST_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC3RST` writer - SDMMC3RST"]
pub type SDMMC3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2RSTCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA1RST"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2RST"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamuxrst(&self) -> DMAMUXRST_R {
        DMAMUXRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12RST"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USBORST"]
    #[inline(always)]
    pub fn usborst(&self) -> USBORST_R {
        USBORST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC3RST"]
    #[inline(always)]
    pub fn sdmmc3rst(&self) -> SDMMC3RST_R {
        SDMMC3RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1RST"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    #[doc = "Bit 1 - DMA2RST"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<1> {
        DMA2RST_W::new(self)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxrst(&mut self) -> DMAMUXRST_W<2> {
        DMAMUXRST_W::new(self)
    }
    #[doc = "Bit 5 - ADC12RST"]
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<5> {
        ADC12RST_W::new(self)
    }
    #[doc = "Bit 8 - USBORST"]
    #[inline(always)]
    #[must_use]
    pub fn usborst(&mut self) -> USBORST_W<8> {
        USBORST_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC3RST"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc3rst(&mut self) -> SDMMC3RST_W<16> {
        SDMMC3RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb2rstclrr](index.html) module"]
pub struct RCC_AHB2RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_AHB2RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb2rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_AHB2RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb2rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_AHB2RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB2RSTCLRR to value 0"]
impl crate::Resettable for RCC_AHB2RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
