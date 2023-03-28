#[doc = "Register `SPDIFRX_IMR` reader"]
pub struct R(crate::R<SPDIFRX_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDIFRX_IMR` writer"]
pub struct W(crate::W<SPDIFRX_IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDIFRX_IMR_SPEC>;
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
impl From<crate::W<SPDIFRX_IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDIFRX_IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNEIE` reader - RXNEIE"]
pub type RXNEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXNEIE` writer - RXNEIE"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IMR_SPEC, bool, O>;
#[doc = "Field `CSRNEIE` reader - CSRNEIE"]
pub type CSRNEIE_R = crate::BitReader<bool>;
#[doc = "Field `CSRNEIE` writer - CSRNEIE"]
pub type CSRNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IMR_SPEC, bool, O>;
#[doc = "Field `PERRIE` reader - PERRIE"]
pub type PERRIE_R = crate::BitReader<bool>;
#[doc = "Field `PERRIE` writer - PERRIE"]
pub type PERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IMR_SPEC, bool, O>;
#[doc = "Field `OVRIE` reader - OVRIE"]
pub type OVRIE_R = crate::BitReader<bool>;
#[doc = "Field `OVRIE` writer - OVRIE"]
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IMR_SPEC, bool, O>;
#[doc = "Field `SBLKIE` reader - SBLKIE"]
pub type SBLKIE_R = crate::BitReader<bool>;
#[doc = "Field `SBLKIE` writer - SBLKIE"]
pub type SBLKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IMR_SPEC, bool, O>;
#[doc = "Field `SYNCDIE` reader - SYNCDIE"]
pub type SYNCDIE_R = crate::BitReader<bool>;
#[doc = "Field `SYNCDIE` writer - SYNCDIE"]
pub type SYNCDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IMR_SPEC, bool, O>;
#[doc = "Field `IFEIE` reader - IFEIE"]
pub type IFEIE_R = crate::BitReader<bool>;
#[doc = "Field `IFEIE` writer - IFEIE"]
pub type IFEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSRNEIE"]
    #[inline(always)]
    pub fn csrneie(&self) -> CSRNEIE_R {
        CSRNEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PERRIE"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SBLKIE"]
    #[inline(always)]
    pub fn sblkie(&self) -> SBLKIE_R {
        SBLKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNCDIE"]
    #[inline(always)]
    pub fn syncdie(&self) -> SYNCDIE_R {
        SYNCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IFEIE"]
    #[inline(always)]
    pub fn ifeie(&self) -> IFEIE_R {
        IFEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<0> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 1 - CSRNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn csrneie(&mut self) -> CSRNEIE_W<1> {
        CSRNEIE_W::new(self)
    }
    #[doc = "Bit 2 - PERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PERRIE_W<2> {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 3 - OVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<3> {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 4 - SBLKIE"]
    #[inline(always)]
    #[must_use]
    pub fn sblkie(&mut self) -> SBLKIE_W<4> {
        SBLKIE_W::new(self)
    }
    #[doc = "Bit 5 - SYNCDIE"]
    #[inline(always)]
    #[must_use]
    pub fn syncdie(&mut self) -> SYNCDIE_W<5> {
        SYNCDIE_W::new(self)
    }
    #[doc = "Bit 6 - IFEIE"]
    #[inline(always)]
    #[must_use]
    pub fn ifeie(&mut self) -> IFEIE_W<6> {
        IFEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_imr](index.html) module"]
pub struct SPDIFRX_IMR_SPEC;
impl crate::RegisterSpec for SPDIFRX_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_imr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdifrx_imr::W](W) writer structure"]
impl crate::Writable for SPDIFRX_IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDIFRX_IMR to value 0"]
impl crate::Resettable for SPDIFRX_IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
