#[doc = "Register `AHB3RSTR` reader"]
pub struct R(crate::R<AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3RSTR` writer"]
pub struct W(crate::W<AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3RSTR_SPEC>;
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
impl From<crate::W<AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMARST` reader - MDMA block reset"]
pub type MDMARST_R = crate::BitReader<MDMARST_A>;
#[doc = "MDMA block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMARST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<MDMARST_A> for bool {
    #[inline(always)]
    fn from(variant: MDMARST_A) -> Self {
        variant as u8 != 0
    }
}
impl MDMARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MDMARST_A> {
        match self.bits {
            true => Some(MDMARST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MDMARST_A::Reset
    }
}
#[doc = "Field `MDMARST` writer - MDMA block reset"]
pub type MDMARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, MDMARST_A, O>;
impl<'a, const O: u8> MDMARST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MDMARST_A::Reset)
    }
}
#[doc = "Field `DMA2DRST` reader - DMA2D block reset"]
pub use MDMARST_R as DMA2DRST_R;
#[doc = "Field `FMCRST` reader - FMC block reset"]
pub use MDMARST_R as FMCRST_R;
#[doc = "Field `OCTOSPI1RST` reader - OCTOSPI1 and OCTOSPI1 delay block reset"]
pub use MDMARST_R as OCTOSPI1RST_R;
#[doc = "Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay block reset"]
pub use MDMARST_R as SDMMC1RST_R;
#[doc = "Field `OCTOSPI2RST` reader - OCTOSPI2 and OCTOSPI2 delay block reset"]
pub use MDMARST_R as OCTOSPI2RST_R;
#[doc = "Field `IOMNGRRST` reader - OCTOSPI IO manager reset"]
pub use MDMARST_R as IOMNGRRST_R;
#[doc = "Field `OTFD1RST` reader - OTFDEC1 reset"]
pub use MDMARST_R as OTFD1RST_R;
#[doc = "Field `OTFD2RST` reader - OTFDEC2 reset"]
pub use MDMARST_R as OTFD2RST_R;
#[doc = "Field `CPURST` reader - CPU reset"]
pub use MDMARST_R as CPURST_R;
#[doc = "Field `DMA2DRST` writer - DMA2D block reset"]
pub use MDMARST_W as DMA2DRST_W;
#[doc = "Field `FMCRST` writer - FMC block reset"]
pub use MDMARST_W as FMCRST_W;
#[doc = "Field `OCTOSPI1RST` writer - OCTOSPI1 and OCTOSPI1 delay block reset"]
pub use MDMARST_W as OCTOSPI1RST_W;
#[doc = "Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay block reset"]
pub use MDMARST_W as SDMMC1RST_W;
#[doc = "Field `OCTOSPI2RST` writer - OCTOSPI2 and OCTOSPI2 delay block reset"]
pub use MDMARST_W as OCTOSPI2RST_W;
#[doc = "Field `IOMNGRRST` writer - OCTOSPI IO manager reset"]
pub use MDMARST_W as IOMNGRRST_W;
#[doc = "Field `OTFD1RST` writer - OTFDEC1 reset"]
pub use MDMARST_W as OTFD1RST_W;
#[doc = "Field `OTFD2RST` writer - OTFDEC2 reset"]
pub use MDMARST_W as OTFD2RST_W;
#[doc = "Field `CPURST` writer - CPU reset"]
pub use MDMARST_W as CPURST_W;
impl R {
    #[doc = "Bit 0 - MDMA block reset"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D block reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC block reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block reset"]
    #[inline(always)]
    pub fn octospi1rst(&self) -> OCTOSPI1RST_R {
        OCTOSPI1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay block reset"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block reset"]
    #[inline(always)]
    pub fn octospi2rst(&self) -> OCTOSPI2RST_R {
        OCTOSPI2RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager reset"]
    #[inline(always)]
    pub fn iomngrrst(&self) -> IOMNGRRST_R {
        IOMNGRRST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OTFDEC1 reset"]
    #[inline(always)]
    pub fn otfd1rst(&self) -> OTFD1RST_R {
        OTFD1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OTFDEC2 reset"]
    #[inline(always)]
    pub fn otfd2rst(&self) -> OTFD2RST_R {
        OTFD2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU reset"]
    #[inline(always)]
    pub fn cpurst(&self) -> CPURST_R {
        CPURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA block reset"]
    #[inline(always)]
    #[must_use]
    pub fn mdmarst(&mut self) -> MDMARST_W<0> {
        MDMARST_W::new(self)
    }
    #[doc = "Bit 4 - DMA2D block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<4> {
        DMA2DRST_W::new(self)
    }
    #[doc = "Bit 12 - FMC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<12> {
        FMCRST_W::new(self)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1rst(&mut self) -> OCTOSPI1RST_W<14> {
        OCTOSPI1RST_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<16> {
        SDMMC1RST_W::new(self)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2rst(&mut self) -> OCTOSPI2RST_W<19> {
        OCTOSPI2RST_W::new(self)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager reset"]
    #[inline(always)]
    #[must_use]
    pub fn iomngrrst(&mut self) -> IOMNGRRST_W<21> {
        IOMNGRRST_W::new(self)
    }
    #[doc = "Bit 22 - OTFDEC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn otfd1rst(&mut self) -> OTFD1RST_W<22> {
        OTFD1RST_W::new(self)
    }
    #[doc = "Bit 23 - OTFDEC2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn otfd2rst(&mut self) -> OTFD2RST_W<23> {
        OTFD2RST_W::new(self)
    }
    #[doc = "Bit 31 - CPU reset"]
    #[inline(always)]
    #[must_use]
    pub fn cpurst(&mut self) -> CPURST_W<31> {
        CPURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB3 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3rstr](index.html) module"]
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3rstr::R](R) reader structure"]
impl crate::Readable for AHB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3rstr::W](W) writer structure"]
impl crate::Writable for AHB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
