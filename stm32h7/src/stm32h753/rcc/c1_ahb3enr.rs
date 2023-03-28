#[doc = "Register `C1_AHB3ENR` reader"]
pub struct R(crate::R<C1_AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_AHB3ENR` writer"]
pub struct W(crate::W<C1_AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB3ENR_SPEC>;
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
impl From<crate::W<C1_AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMAEN` reader - MDMA Peripheral Clock Enable"]
pub type MDMAEN_R = crate::BitReader<MDMAEN_A>;
#[doc = "MDMA Peripheral Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMAEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<MDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMAEN_A {
        match self.bits {
            false => MDMAEN_A::Disabled,
            true => MDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MDMAEN_A::Enabled
    }
}
#[doc = "Field `MDMAEN` writer - MDMA Peripheral Clock Enable"]
pub type MDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_AHB3ENR_SPEC, MDMAEN_A, O>;
impl<'a, const O: u8> MDMAEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDMAEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MDMAEN_A::Enabled)
    }
}
#[doc = "Field `DMA2DEN` reader - DMA2D Peripheral Clock Enable"]
pub use MDMAEN_R as DMA2DEN_R;
#[doc = "Field `JPGDECEN` reader - JPGDEC Peripheral Clock Enable"]
pub use MDMAEN_R as JPGDECEN_R;
#[doc = "Field `FMCEN` reader - FMC Peripheral Clocks Enable"]
pub use MDMAEN_R as FMCEN_R;
#[doc = "Field `QSPIEN` reader - QUADSPI and QUADSPI Delay Clock Enable"]
pub use MDMAEN_R as QSPIEN_R;
#[doc = "Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub use MDMAEN_R as SDMMC1EN_R;
#[doc = "Field `DMA2DEN` writer - DMA2D Peripheral Clock Enable"]
pub use MDMAEN_W as DMA2DEN_W;
#[doc = "Field `JPGDECEN` writer - JPGDEC Peripheral Clock Enable"]
pub use MDMAEN_W as JPGDECEN_W;
#[doc = "Field `FMCEN` writer - FMC Peripheral Clocks Enable"]
pub use MDMAEN_W as FMCEN_W;
#[doc = "Field `QSPIEN` writer - QUADSPI and QUADSPI Delay Clock Enable"]
pub use MDMAEN_W as QSPIEN_W;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub use MDMAEN_W as SDMMC1EN_W;
impl R {
    #[doc = "Bit 0 - MDMA Peripheral Clock Enable"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D Peripheral Clock Enable"]
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPGDEC Peripheral Clock Enable"]
    #[inline(always)]
    pub fn jpgdecen(&self) -> JPGDECEN_R {
        JPGDECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MDMAEN_W<0> {
        MDMAEN_W::new(self)
    }
    #[doc = "Bit 4 - DMA2D Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2den(&mut self) -> DMA2DEN_W<4> {
        DMA2DEN_W::new(self)
    }
    #[doc = "Bit 5 - JPGDEC Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jpgdecen(&mut self) -> JPGDECEN_W<5> {
        JPGDECEN_W::new(self)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<12> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspien(&mut self) -> QSPIEN_W<14> {
        QSPIEN_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<16> {
        SDMMC1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB3 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb3enr](index.html) module"]
pub struct C1_AHB3ENR_SPEC;
impl crate::RegisterSpec for C1_AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_ahb3enr::R](R) reader structure"]
impl crate::Readable for C1_AHB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_ahb3enr::W](W) writer structure"]
impl crate::Writable for C1_AHB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_AHB3ENR to value 0"]
impl crate::Resettable for C1_AHB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
