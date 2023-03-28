#[doc = "Register `AHB1ENR` reader"]
pub struct R(crate::R<AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1ENR` writer"]
pub struct W(crate::W<AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1ENR_SPEC>;
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
impl From<crate::W<AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1EN` reader - CPU1 DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader<DMA1EN_A>;
#[doc = "CPU1 DMA1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::Disabled,
            true => DMA1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN_A::Enabled
    }
}
#[doc = "Field `DMA1EN` writer - CPU1 DMA1 clock enable"]
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, DMA1EN_A, O>;
impl<'a, const O: u8> DMA1EN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Enabled)
    }
}
#[doc = "Field `DMA2EN` reader - CPU1 DMA2 clock enable"]
pub use DMA1EN_R as DMA2EN_R;
#[doc = "Field `DMAMUX1EN` reader - CPU1 DMAMUX1 clock enable"]
pub use DMA1EN_R as DMAMUX1EN_R;
#[doc = "Field `CRCEN` reader - CPU1 CRC clock enable"]
pub use DMA1EN_R as CRCEN_R;
#[doc = "Field `DMA2EN` writer - CPU1 DMA2 clock enable"]
pub use DMA1EN_W as DMA2EN_W;
#[doc = "Field `DMAMUX1EN` writer - CPU1 DMAMUX1 clock enable"]
pub use DMA1EN_W as DMAMUX1EN_W;
#[doc = "Field `CRCEN` writer - CPU1 CRC clock enable"]
pub use DMA1EN_W as CRCEN_W;
impl R {
    #[doc = "Bit 0 - CPU1 DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU1 DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU1 DMAMUX1 clock enable"]
    #[inline(always)]
    pub fn dmamux1en(&self) -> DMAMUX1EN_R {
        DMAMUX1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU1 CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - CPU1 DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - CPU1 DMAMUX1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1en(&mut self) -> DMAMUX1EN_W<2> {
        DMAMUX1EN_W::new(self)
    }
    #[doc = "Bit 12 - CPU1 CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1enr](index.html) module"]
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1enr::R](R) reader structure"]
impl crate::Readable for AHB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1enr::W](W) writer structure"]
impl crate::Writable for AHB1ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0"]
impl crate::Resettable for AHB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
