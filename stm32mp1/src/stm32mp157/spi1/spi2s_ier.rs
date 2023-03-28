#[doc = "Register `SPI2S_IER` reader"]
pub struct R(crate::R<SPI2S_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2S_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2S_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2S_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI2S_IER` writer"]
pub struct W(crate::W<SPI2S_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2S_IER_SPEC>;
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
impl From<crate::W<SPI2S_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2S_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPIE` reader - RXPIE"]
pub type RXPIE_R = crate::BitReader<bool>;
#[doc = "Field `RXPIE` writer - RXPIE"]
pub type RXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `TXPIE` reader - TXPIE"]
pub type TXPIE_R = crate::BitReader<bool>;
#[doc = "Field `TXPIE` writer - TXPIE"]
pub type TXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `DXPIE` reader - DXPIE"]
pub type DXPIE_R = crate::BitReader<bool>;
#[doc = "Field `DXPIE` writer - DXPIE"]
pub type DXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `EOTIE` reader - EOTIE"]
pub type EOTIE_R = crate::BitReader<bool>;
#[doc = "Field `EOTIE` writer - EOTIE"]
pub type EOTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `TXTFIE` reader - TXTFIE"]
pub type TXTFIE_R = crate::BitReader<bool>;
#[doc = "Field `TXTFIE` writer - TXTFIE"]
pub type TXTFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `UDRIE` reader - UDRIE"]
pub type UDRIE_R = crate::BitReader<bool>;
#[doc = "Field `UDRIE` writer - UDRIE"]
pub type UDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `OVRIE` reader - OVRIE"]
pub type OVRIE_R = crate::BitReader<bool>;
#[doc = "Field `OVRIE` writer - OVRIE"]
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `CRCEIE` reader - CRCEIE"]
pub type CRCEIE_R = crate::BitReader<bool>;
#[doc = "Field `CRCEIE` writer - CRCEIE"]
pub type CRCEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `TIFREIE` reader - TIFREIE"]
pub type TIFREIE_R = crate::BitReader<bool>;
#[doc = "Field `TIFREIE` writer - TIFREIE"]
pub type TIFREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `MODFIE` reader - MODFIE"]
pub type MODFIE_R = crate::BitReader<bool>;
#[doc = "Field `MODFIE` writer - MODFIE"]
pub type MODFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
#[doc = "Field `TSERFIE` reader - TSERFIE"]
pub type TSERFIE_R = crate::BitReader<bool>;
#[doc = "Field `TSERFIE` writer - TSERFIE"]
pub type TSERFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXPIE"]
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXPIE"]
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXPIE"]
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOTIE"]
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXTFIE"]
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UDRIE"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIFREIE"]
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MODFIE"]
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TSERFIE"]
    #[inline(always)]
    pub fn tserfie(&self) -> TSERFIE_R {
        TSERFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXPIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxpie(&mut self) -> RXPIE_W<0> {
        RXPIE_W::new(self)
    }
    #[doc = "Bit 1 - TXPIE"]
    #[inline(always)]
    #[must_use]
    pub fn txpie(&mut self) -> TXPIE_W<1> {
        TXPIE_W::new(self)
    }
    #[doc = "Bit 2 - DXPIE"]
    #[inline(always)]
    #[must_use]
    pub fn dxpie(&mut self) -> DXPIE_W<2> {
        DXPIE_W::new(self)
    }
    #[doc = "Bit 3 - EOTIE"]
    #[inline(always)]
    #[must_use]
    pub fn eotie(&mut self) -> EOTIE_W<3> {
        EOTIE_W::new(self)
    }
    #[doc = "Bit 4 - TXTFIE"]
    #[inline(always)]
    #[must_use]
    pub fn txtfie(&mut self) -> TXTFIE_W<4> {
        TXTFIE_W::new(self)
    }
    #[doc = "Bit 5 - UDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<5> {
        UDRIE_W::new(self)
    }
    #[doc = "Bit 6 - OVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<6> {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 7 - CRCEIE"]
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<7> {
        CRCEIE_W::new(self)
    }
    #[doc = "Bit 8 - TIFREIE"]
    #[inline(always)]
    #[must_use]
    pub fn tifreie(&mut self) -> TIFREIE_W<8> {
        TIFREIE_W::new(self)
    }
    #[doc = "Bit 9 - MODFIE"]
    #[inline(always)]
    #[must_use]
    pub fn modfie(&mut self) -> MODFIE_W<9> {
        MODFIE_W::new(self)
    }
    #[doc = "Bit 10 - TSERFIE"]
    #[inline(always)]
    #[must_use]
    pub fn tserfie(&mut self) -> TSERFIE_W<10> {
        TSERFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_ier](index.html) module"]
pub struct SPI2S_IER_SPEC;
impl crate::RegisterSpec for SPI2S_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2s_ier::R](R) reader structure"]
impl crate::Readable for SPI2S_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi2s_ier::W](W) writer structure"]
impl crate::Writable for SPI2S_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI2S_IER to value 0"]
impl crate::Resettable for SPI2S_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
