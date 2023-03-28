#[doc = "Register `C2AHB1SMENR` reader"]
pub struct R(crate::R<C2AHB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB1SMENR` writer"]
pub struct W(crate::W<C2AHB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB1SMENR_SPEC>;
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
impl From<crate::W<C2AHB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1SMEN` reader - DMA1 clock enable during CPU2 CSleep mode."]
pub type DMA1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1SMEN` writer - DMA1 clock enable during CPU2 CSleep mode."]
pub type DMA1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB1SMENR_SPEC, bool, O>;
#[doc = "Field `DMA2SMEN` reader - DMA2 clock enable during CPU2 CSleep mode."]
pub type DMA2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2SMEN` writer - DMA2 clock enable during CPU2 CSleep mode."]
pub type DMA2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB1SMENR_SPEC, bool, O>;
#[doc = "Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during CPU2 CSleep mode."]
pub type DMAMUX1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during CPU2 CSleep mode."]
pub type DMAMUX1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB1SMENR_SPEC, bool, O>;
#[doc = "Field `CRCSMEN` reader - CRC clock enable during CPU2 CSleep mode."]
pub type CRCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCSMEN` writer - CRC clock enable during CPU2 CSleep mode."]
pub type CRCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB1SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<0> {
        DMA1SMEN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<1> {
        DMA2SMEN_W::new(self)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<2> {
        DMAMUX1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<12> {
        CRCSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep modes register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb1smenr](index.html) module"]
pub struct C2AHB1SMENR_SPEC;
impl crate::RegisterSpec for C2AHB1SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb1smenr::R](R) reader structure"]
impl crate::Readable for C2AHB1SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb1smenr::W](W) writer structure"]
impl crate::Writable for C2AHB1SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2AHB1SMENR to value 0x1007"]
impl crate::Resettable for C2AHB1SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1007;
}
