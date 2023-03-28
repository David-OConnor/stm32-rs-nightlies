#[doc = "Register `C1_AHB3LPENR` reader"]
pub struct R(crate::R<C1_AHB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_AHB3LPENR` writer"]
pub struct W(crate::W<C1_AHB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB3LPENR_SPEC>;
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
impl From<crate::W<C1_AHB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMALPEN` reader - MDMA Clock Enable During CSleep Mode"]
pub type MDMALPEN_R = crate::BitReader<MDMALPEN_A>;
#[doc = "MDMA Clock Enable During CSleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMALPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<MDMALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMALPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MDMALPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMALPEN_A {
        match self.bits {
            false => MDMALPEN_A::Disabled,
            true => MDMALPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMALPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MDMALPEN_A::Enabled
    }
}
#[doc = "Field `MDMALPEN` writer - MDMA Clock Enable During CSleep Mode"]
pub type MDMALPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_AHB3LPENR_SPEC, MDMALPEN_A, O>;
impl<'a, const O: u8> MDMALPEN_W<'a, O> {
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDMALPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MDMALPEN_A::Enabled)
    }
}
#[doc = "Field `DMA2DLPEN` reader - DMA2D Clock Enable During CSleep Mode"]
pub use MDMALPEN_R as DMA2DLPEN_R;
#[doc = "Field `DMA2DLPEN` writer - DMA2D Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as DMA2DLPEN_W;
#[doc = "Field `FLASHPREN` reader - Flash interface clock enable during csleep mode"]
pub type FLASHPREN_R = crate::BitReader<bool>;
#[doc = "Field `FLASHPREN` writer - Flash interface clock enable during csleep mode"]
pub type FLASHPREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_AHB3LPENR_SPEC, bool, O>;
#[doc = "Field `FMCLPEN` reader - FMC Peripheral Clocks Enable During CSleep Mode"]
pub use MDMALPEN_R as FMCLPEN_R;
#[doc = "Field `OCTOSPI1LPEN` reader - OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
pub use MDMALPEN_R as OCTOSPI1LPEN_R;
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub use MDMALPEN_R as SDMMC1LPEN_R;
#[doc = "Field `OCTOSPI2LPEN` reader - OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
pub use MDMALPEN_R as OCTOSPI2LPEN_R;
#[doc = "Field `IOMNGRLPEN` reader - OCTOSPI IO manager enable during CSleep Mode"]
pub use MDMALPEN_R as IOMNGRLPEN_R;
#[doc = "Field `OTFD1LPEN` reader - OTFDEC1 enable during CSleep Mode"]
pub use MDMALPEN_R as OTFD1LPEN_R;
#[doc = "Field `OTFD2LPEN` reader - OTFDEC2 enable during CSleep Mode"]
pub use MDMALPEN_R as OTFD2LPEN_R;
#[doc = "Field `D1DTCM1LPEN` reader - D1DTCM1 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as D1DTCM1LPEN_R;
#[doc = "Field `DTCM2LPEN` reader - D1 DTCM2 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as DTCM2LPEN_R;
#[doc = "Field `ITCMLPEN` reader - D1ITCM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as ITCMLPEN_R;
#[doc = "Field `AXISRAMLPEN` reader - AXISRAM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_R as AXISRAMLPEN_R;
#[doc = "Field `FMCLPEN` writer - FMC Peripheral Clocks Enable During CSleep Mode"]
pub use MDMALPEN_W as FMCLPEN_W;
#[doc = "Field `OCTOSPI1LPEN` writer - OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
pub use MDMALPEN_W as OCTOSPI1LPEN_W;
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub use MDMALPEN_W as SDMMC1LPEN_W;
#[doc = "Field `OCTOSPI2LPEN` writer - OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
pub use MDMALPEN_W as OCTOSPI2LPEN_W;
#[doc = "Field `IOMNGRLPEN` writer - OCTOSPI IO manager enable during CSleep Mode"]
pub use MDMALPEN_W as IOMNGRLPEN_W;
#[doc = "Field `OTFD1LPEN` writer - OTFDEC1 enable during CSleep Mode"]
pub use MDMALPEN_W as OTFD1LPEN_W;
#[doc = "Field `OTFD2LPEN` writer - OTFDEC2 enable during CSleep Mode"]
pub use MDMALPEN_W as OTFD2LPEN_W;
#[doc = "Field `D1DTCM1LPEN` writer - D1DTCM1 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_W as D1DTCM1LPEN_W;
#[doc = "Field `DTCM2LPEN` writer - D1 DTCM2 Block Clock Enable During CSleep mode"]
pub use MDMALPEN_W as DTCM2LPEN_W;
#[doc = "Field `ITCMLPEN` writer - D1ITCM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_W as ITCMLPEN_W;
#[doc = "Field `AXISRAMLPEN` writer - AXISRAM Block Clock Enable During CSleep mode"]
pub use MDMALPEN_W as AXISRAMLPEN_W;
impl R {
    #[doc = "Bit 0 - MDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash interface clock enable during csleep mode"]
    #[inline(always)]
    pub fn flashpren(&self) -> FLASHPREN_R {
        FLASHPREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
    #[inline(always)]
    pub fn octospi1lpen(&self) -> OCTOSPI1LPEN_R {
        OCTOSPI1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
    #[inline(always)]
    pub fn octospi2lpen(&self) -> OCTOSPI2LPEN_R {
        OCTOSPI2LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager enable during CSleep Mode"]
    #[inline(always)]
    pub fn iomngrlpen(&self) -> IOMNGRLPEN_R {
        IOMNGRLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OTFDEC1 enable during CSleep Mode"]
    #[inline(always)]
    pub fn otfd1lpen(&self) -> OTFD1LPEN_R {
        OTFD1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OTFDEC2 enable during CSleep Mode"]
    #[inline(always)]
    pub fn otfd2lpen(&self) -> OTFD2LPEN_R {
        OTFD2LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn d1dtcm1lpen(&self) -> D1DTCM1LPEN_R {
        D1DTCM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn axisramlpen(&self) -> AXISRAMLPEN_R {
        AXISRAMLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<0> {
        MDMALPEN_W::new(self)
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<4> {
        DMA2DLPEN_W::new(self)
    }
    #[doc = "Bit 8 - Flash interface clock enable during csleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashpren(&mut self) -> FLASHPREN_W<8> {
        FLASHPREN_W::new(self)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<12> {
        FMCLPEN_W::new(self)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1lpen(&mut self) -> OCTOSPI1LPEN_W<14> {
        OCTOSPI1LPEN_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<16> {
        SDMMC1LPEN_W::new(self)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2lpen(&mut self) -> OCTOSPI2LPEN_W<19> {
        OCTOSPI2LPEN_W::new(self)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn iomngrlpen(&mut self) -> IOMNGRLPEN_W<21> {
        IOMNGRLPEN_W::new(self)
    }
    #[doc = "Bit 22 - OTFDEC1 enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn otfd1lpen(&mut self) -> OTFD1LPEN_W<22> {
        OTFD1LPEN_W::new(self)
    }
    #[doc = "Bit 23 - OTFDEC2 enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn otfd2lpen(&mut self) -> OTFD2LPEN_W<23> {
        OTFD2LPEN_W::new(self)
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn d1dtcm1lpen(&mut self) -> D1DTCM1LPEN_W<28> {
        D1DTCM1LPEN_W::new(self)
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W<29> {
        DTCM2LPEN_W::new(self)
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W<30> {
        ITCMLPEN_W::new(self)
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn axisramlpen(&mut self) -> AXISRAMLPEN_W<31> {
        AXISRAMLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB3 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb3lpenr](index.html) module"]
pub struct C1_AHB3LPENR_SPEC;
impl crate::RegisterSpec for C1_AHB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_ahb3lpenr::R](R) reader structure"]
impl crate::Readable for C1_AHB3LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_ahb3lpenr::W](W) writer structure"]
impl crate::Writable for C1_AHB3LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_AHB3LPENR to value 0"]
impl crate::Resettable for C1_AHB3LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
