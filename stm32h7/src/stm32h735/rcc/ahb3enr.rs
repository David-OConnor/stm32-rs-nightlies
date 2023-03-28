#[doc = "Register `AHB3ENR` reader"]
pub struct R(crate::R<AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3ENR` writer"]
pub struct W(crate::W<AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3ENR_SPEC>;
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
impl From<crate::W<AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3ENR_SPEC>) -> Self {
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
pub type MDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3ENR_SPEC, MDMAEN_A, O>;
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
#[doc = "Field `FMCEN` reader - FMC Peripheral Clocks Enable"]
pub use MDMAEN_R as FMCEN_R;
#[doc = "Field `OCTOSPI1EN` reader - OCTOSPI1 and OCTOSPI1 delay block enable"]
pub use MDMAEN_R as OCTOSPI1EN_R;
#[doc = "Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub use MDMAEN_R as SDMMC1EN_R;
#[doc = "Field `OCTOSPI2EN` reader - OCTOSPI2 and OCTOSPI2 delay block enable"]
pub use MDMAEN_R as OCTOSPI2EN_R;
#[doc = "Field `IOMNGREN` reader - OCTOSPI IO manager enable"]
pub use MDMAEN_R as IOMNGREN_R;
#[doc = "Field `OTFD1EN` reader - OTFDEC1 enable"]
pub use MDMAEN_R as OTFD1EN_R;
#[doc = "Field `OTFD2EN` reader - OTFDEC2 enable"]
pub use MDMAEN_R as OTFD2EN_R;
#[doc = "Field `DMA2DEN` writer - DMA2D Peripheral Clock Enable"]
pub use MDMAEN_W as DMA2DEN_W;
#[doc = "Field `FMCEN` writer - FMC Peripheral Clocks Enable"]
pub use MDMAEN_W as FMCEN_W;
#[doc = "Field `OCTOSPI1EN` writer - OCTOSPI1 and OCTOSPI1 delay block enable"]
pub use MDMAEN_W as OCTOSPI1EN_W;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub use MDMAEN_W as SDMMC1EN_W;
#[doc = "Field `OCTOSPI2EN` writer - OCTOSPI2 and OCTOSPI2 delay block enable"]
pub use MDMAEN_W as OCTOSPI2EN_W;
#[doc = "Field `IOMNGREN` writer - OCTOSPI IO manager enable"]
pub use MDMAEN_W as IOMNGREN_W;
#[doc = "Field `OTFD1EN` writer - OTFDEC1 enable"]
pub use MDMAEN_W as OTFD1EN_W;
#[doc = "Field `OTFD2EN` writer - OTFDEC2 enable"]
pub use MDMAEN_W as OTFD2EN_W;
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
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block enable"]
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block enable"]
    #[inline(always)]
    pub fn octospi2en(&self) -> OCTOSPI2EN_R {
        OCTOSPI2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager enable"]
    #[inline(always)]
    pub fn iomngren(&self) -> IOMNGREN_R {
        IOMNGREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OTFDEC1 enable"]
    #[inline(always)]
    pub fn otfd1en(&self) -> OTFD1EN_R {
        OTFD1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OTFDEC2 enable"]
    #[inline(always)]
    pub fn otfd2en(&self) -> OTFD2EN_R {
        OTFD2EN_R::new(((self.bits >> 23) & 1) != 0)
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
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<12> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block enable"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<14> {
        OCTOSPI1EN_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<16> {
        SDMMC1EN_W::new(self)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block enable"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2en(&mut self) -> OCTOSPI2EN_W<19> {
        OCTOSPI2EN_W::new(self)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager enable"]
    #[inline(always)]
    #[must_use]
    pub fn iomngren(&mut self) -> IOMNGREN_W<21> {
        IOMNGREN_W::new(self)
    }
    #[doc = "Bit 22 - OTFDEC1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn otfd1en(&mut self) -> OTFD1EN_W<22> {
        OTFD1EN_W::new(self)
    }
    #[doc = "Bit 23 - OTFDEC2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn otfd2en(&mut self) -> OTFD2EN_W<23> {
        OTFD2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB3 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3enr](index.html) module"]
pub struct AHB3ENR_SPEC;
impl crate::RegisterSpec for AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3enr::R](R) reader structure"]
impl crate::Readable for AHB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3enr::W](W) writer structure"]
impl crate::Writable for AHB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for AHB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
