#[doc = "Register `SYSCFG_IOCTRLCLRR` reader"]
pub struct R(crate::R<SYSCFG_IOCTRLCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_IOCTRLCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_IOCTRLCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_IOCTRLCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_IOCTRLCLRR` writer"]
pub struct W(crate::W<SYSCFG_IOCTRLCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_IOCTRLCLRR_SPEC>;
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
impl From<crate::W<SYSCFG_IOCTRLCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_IOCTRLCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSLVEN_TRACE` reader - HSLVEN_TRACE"]
pub type HSLVEN_TRACE_R = crate::BitReader<bool>;
#[doc = "Field `HSLVEN_TRACE` writer - HSLVEN_TRACE"]
pub type HSLVEN_TRACE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCFG_IOCTRLCLRR_SPEC, bool, O>;
#[doc = "Field `HSLVEN_QUADSPI` reader - HSLVEN_QUADSPI"]
pub type HSLVEN_QUADSPI_R = crate::BitReader<bool>;
#[doc = "Field `HSLVEN_QUADSPI` writer - HSLVEN_QUADSPI"]
pub type HSLVEN_QUADSPI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCFG_IOCTRLCLRR_SPEC, bool, O>;
#[doc = "Field `HSLVEN_ETH` reader - HSLVEN_ETH"]
pub type HSLVEN_ETH_R = crate::BitReader<bool>;
#[doc = "Field `HSLVEN_ETH` writer - HSLVEN_ETH"]
pub type HSLVEN_ETH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_IOCTRLCLRR_SPEC, bool, O>;
#[doc = "Field `HSLVEN_SDMMC` reader - HSLVEN_SDMMC"]
pub type HSLVEN_SDMMC_R = crate::BitReader<bool>;
#[doc = "Field `HSLVEN_SDMMC` writer - HSLVEN_SDMMC"]
pub type HSLVEN_SDMMC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCFG_IOCTRLCLRR_SPEC, bool, O>;
#[doc = "Field `HSLVEN_SPI` reader - HSLVEN_SPI"]
pub type HSLVEN_SPI_R = crate::BitReader<bool>;
#[doc = "Field `HSLVEN_SPI` writer - HSLVEN_SPI"]
pub type HSLVEN_SPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_IOCTRLCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HSLVEN_TRACE"]
    #[inline(always)]
    pub fn hslven_trace(&self) -> HSLVEN_TRACE_R {
        HSLVEN_TRACE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSLVEN_QUADSPI"]
    #[inline(always)]
    pub fn hslven_quadspi(&self) -> HSLVEN_QUADSPI_R {
        HSLVEN_QUADSPI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSLVEN_ETH"]
    #[inline(always)]
    pub fn hslven_eth(&self) -> HSLVEN_ETH_R {
        HSLVEN_ETH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSLVEN_SDMMC"]
    #[inline(always)]
    pub fn hslven_sdmmc(&self) -> HSLVEN_SDMMC_R {
        HSLVEN_SDMMC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSLVEN_SPI"]
    #[inline(always)]
    pub fn hslven_spi(&self) -> HSLVEN_SPI_R {
        HSLVEN_SPI_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSLVEN_TRACE"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_trace(&mut self) -> HSLVEN_TRACE_W<0> {
        HSLVEN_TRACE_W::new(self)
    }
    #[doc = "Bit 1 - HSLVEN_QUADSPI"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_quadspi(&mut self) -> HSLVEN_QUADSPI_W<1> {
        HSLVEN_QUADSPI_W::new(self)
    }
    #[doc = "Bit 2 - HSLVEN_ETH"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_eth(&mut self) -> HSLVEN_ETH_W<2> {
        HSLVEN_ETH_W::new(self)
    }
    #[doc = "Bit 3 - HSLVEN_SDMMC"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_sdmmc(&mut self) -> HSLVEN_SDMMC_W<3> {
        HSLVEN_SDMMC_W::new(self)
    }
    #[doc = "Bit 4 - HSLVEN_SPI"]
    #[inline(always)]
    #[must_use]
    pub fn hslven_spi(&mut self) -> HSLVEN_SPI_W<4> {
        HSLVEN_SPI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG IO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_ioctrlclrr](index.html) module"]
pub struct SYSCFG_IOCTRLCLRR_SPEC;
impl crate::RegisterSpec for SYSCFG_IOCTRLCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_ioctrlclrr::R](R) reader structure"]
impl crate::Readable for SYSCFG_IOCTRLCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_ioctrlclrr::W](W) writer structure"]
impl crate::Writable for SYSCFG_IOCTRLCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_IOCTRLCLRR to value 0"]
impl crate::Resettable for SYSCFG_IOCTRLCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
