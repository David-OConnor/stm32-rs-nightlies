#[doc = "Register `ETH_MTLQ1ICSR` reader"]
pub struct R(crate::R<ETH_MTLQ1ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLQ1ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLQ1ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLQ1ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLQ1ICSR` writer"]
pub struct W(crate::W<ETH_MTLQ1ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLQ1ICSR_SPEC>;
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
impl From<crate::W<ETH_MTLQ1ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLQ1ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXUNFIS` reader - TXUNFIS"]
pub type TXUNFIS_R = crate::BitReader<bool>;
#[doc = "Field `ABPSIS` reader - ABPSIS"]
pub type ABPSIS_R = crate::BitReader<bool>;
#[doc = "Field `ABPSIS` writer - ABPSIS"]
pub type ABPSIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ1ICSR_SPEC, bool, O>;
#[doc = "Field `TXUIE` reader - TXUIE"]
pub type TXUIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUIE` writer - TXUIE"]
pub type TXUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ1ICSR_SPEC, bool, O>;
#[doc = "Field `ABPSIE` reader - ABPSIE"]
pub type ABPSIE_R = crate::BitReader<bool>;
#[doc = "Field `ABPSIE` writer - ABPSIE"]
pub type ABPSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ1ICSR_SPEC, bool, O>;
#[doc = "Field `RXOVFIS` reader - RXOVFIS"]
pub type RXOVFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXOVFIS` writer - RXOVFIS"]
pub type RXOVFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ1ICSR_SPEC, bool, O>;
#[doc = "Field `RXOIE` reader - RXOIE"]
pub type RXOIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOIE` writer - RXOIE"]
pub type RXOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ1ICSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TXUNFIS"]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ABPSIS"]
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ABPSIE"]
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ABPSIS"]
    #[inline(always)]
    #[must_use]
    pub fn abpsis(&mut self) -> ABPSIS_W<1> {
        ABPSIS_W::new(self)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    #[must_use]
    pub fn txuie(&mut self) -> TXUIE_W<8> {
        TXUIE_W::new(self)
    }
    #[doc = "Bit 9 - ABPSIE"]
    #[inline(always)]
    #[must_use]
    pub fn abpsie(&mut self) -> ABPSIE_W<9> {
        ABPSIE_W::new(self)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    #[must_use]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<16> {
        RXOVFIS_W::new(self)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<24> {
        RXOIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Queue 1 interrupt control status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlq1icsr](index.html) module"]
pub struct ETH_MTLQ1ICSR_SPEC;
impl crate::RegisterSpec for ETH_MTLQ1ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlq1icsr::R](R) reader structure"]
impl crate::Readable for ETH_MTLQ1ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtlq1icsr::W](W) writer structure"]
impl crate::Writable for ETH_MTLQ1ICSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLQ1ICSR to value 0"]
impl crate::Resettable for ETH_MTLQ1ICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
