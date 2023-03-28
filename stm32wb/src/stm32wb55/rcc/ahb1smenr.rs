#[doc = "Register `AHB1SMENR` reader"]
pub struct R(crate::R<AHB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1SMENR` writer"]
pub struct W(crate::W<AHB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1SMENR_SPEC>;
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
impl From<crate::W<AHB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1SMEN` reader - CPU1 DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1SMEN` writer - CPU1 DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
#[doc = "Field `DMA2SMEN` reader - CPU1 DMA2 clocks enable during Sleep and Stop modes"]
pub type DMA2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2SMEN` writer - CPU1 DMA2 clocks enable during Sleep and Stop modes"]
pub type DMA2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
#[doc = "Field `DMAMUXSMEN` reader - CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
pub type DMAMUXSMEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUXSMEN` writer - CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
pub type DMAMUXSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
#[doc = "Field `SRAM1SMEN` reader - CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1SMEN` writer - CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
#[doc = "Field `CRCSMEN` reader - CPU1 CRCSMEN"]
pub type CRCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCSMEN` writer - CPU1 CRCSMEN"]
pub type CRCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
#[doc = "Field `TSCSMEN` reader - CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `TSCSMEN` writer - CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU1 DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU1 DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dmamuxsmen(&self) -> DMAMUXSMEN_R {
        DMAMUXSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU1 CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<0> {
        DMA1SMEN_W::new(self)
    }
    #[doc = "Bit 1 - CPU1 DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<1> {
        DMA2SMEN_W::new(self)
    }
    #[doc = "Bit 2 - CPU1 DMAMUX clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxsmen(&mut self) -> DMAMUXSMEN_W<2> {
        DMAMUXSMEN_W::new(self)
    }
    #[doc = "Bit 9 - CPU1 SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<9> {
        SRAM1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - CPU1 CRCSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<12> {
        CRCSMEN_W::new(self)
    }
    #[doc = "Bit 16 - CPU1 Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<16> {
        TSCSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1smenr](index.html) module"]
pub struct AHB1SMENR_SPEC;
impl crate::RegisterSpec for AHB1SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1smenr::R](R) reader structure"]
impl crate::Readable for AHB1SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1smenr::W](W) writer structure"]
impl crate::Writable for AHB1SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1SMENR to value 0x0001_1207"]
impl crate::Resettable for AHB1SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_1207;
}
