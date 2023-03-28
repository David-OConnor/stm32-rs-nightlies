#[doc = "Register `MTLQICSR` reader"]
pub struct R(crate::R<MTLQICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLQICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLQICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLQICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLQICSR` writer"]
pub struct W(crate::W<MTLQICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLQICSR_SPEC>;
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
impl From<crate::W<MTLQICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLQICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXUNFIS` reader - Transmit Queue Underflow Interrupt Status"]
pub type TXUNFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXUNFIS` writer - Transmit Queue Underflow Interrupt Status"]
pub type TXUNFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLQICSR_SPEC, bool, O>;
#[doc = "Field `TXUIE` reader - Transmit Queue Underflow Interrupt Enable"]
pub type TXUIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUIE` writer - Transmit Queue Underflow Interrupt Enable"]
pub type TXUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLQICSR_SPEC, bool, O>;
#[doc = "Field `RXOVFIS` reader - Receive Queue Overflow Interrupt Status"]
pub type RXOVFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXOVFIS` writer - Receive Queue Overflow Interrupt Status"]
pub type RXOVFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLQICSR_SPEC, bool, O>;
#[doc = "Field `RXOIE` reader - Receive Queue Overflow Interrupt Enable"]
pub type RXOIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOIE` writer - Receive Queue Overflow Interrupt Enable"]
pub type RXOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLQICSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status"]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status"]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn txunfis(&mut self) -> TXUNFIS_W<0> {
        TXUNFIS_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txuie(&mut self) -> TXUIE_W<8> {
        TXUIE_W::new(self)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<16> {
        RXOVFIS_W::new(self)
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable"]
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
#[doc = "Queue interrupt control status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlqicsr](index.html) module"]
pub struct MTLQICSR_SPEC;
impl crate::RegisterSpec for MTLQICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlqicsr::R](R) reader structure"]
impl crate::Readable for MTLQICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlqicsr::W](W) writer structure"]
impl crate::Writable for MTLQICSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLQICSR to value 0"]
impl crate::Resettable for MTLQICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
