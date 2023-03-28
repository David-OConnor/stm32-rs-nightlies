#[doc = "Register `CCIPR3` reader"]
pub struct R(crate::R<CCIPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR3` writer"]
pub struct W(crate::W<CCIPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR3_SPEC>;
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
impl From<crate::W<CCIPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1SEL` reader - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI1SEL` writer - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPI2SEL` reader - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI2SEL` writer - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPI3SEL` reader - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI3SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI3SEL` writer - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPI4SEL` reader - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI4SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI4SEL` writer - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPI5SEL` reader - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI5SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI5SEL` writer - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI5SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPI6SEL` reader - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI6SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI6SEL` writer - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI6SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPUART1SEL` reader - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPUART1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPUART1SEL` writer - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPUART1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi3sel(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi4sel(&self) -> SPI4SEL_R {
        SPI4SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi5sel(&self) -> SPI5SEL_R {
        SPI5SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<0> {
        SPI1SEL_W::new(self)
    }
    #[doc = "Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<3> {
        SPI2SEL_W::new(self)
    }
    #[doc = "Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<6> {
        SPI3SEL_W::new(self)
    }
    #[doc = "Bits 9:11 - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi4sel(&mut self) -> SPI4SEL_W<9> {
        SPI4SEL_W::new(self)
    }
    #[doc = "Bits 12:14 - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi5sel(&mut self) -> SPI5SEL_W<12> {
        SPI5SEL_W::new(self)
    }
    #[doc = "Bits 15:17 - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<15> {
        SPI6SEL_W::new(self)
    }
    #[doc = "Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<24> {
        LPUART1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr3](index.html) module"]
pub struct CCIPR3_SPEC;
impl crate::RegisterSpec for CCIPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr3::R](R) reader structure"]
impl crate::Readable for CCIPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr3::W](W) writer structure"]
impl crate::Writable for CCIPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR3 to value 0"]
impl crate::Resettable for CCIPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
