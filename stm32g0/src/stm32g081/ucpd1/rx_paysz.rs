#[doc = "Register `RX_PAYSZ` reader"]
pub struct R(crate::R<RX_PAYSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PAYSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PAYSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PAYSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_PAYSZ` writer"]
pub struct W(crate::W<RX_PAYSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_PAYSZ_SPEC>;
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
impl From<crate::W<RX_PAYSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_PAYSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPAYSZ` reader - RXPAYSZ"]
pub type RXPAYSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXPAYSZ` writer - RXPAYSZ"]
pub type RXPAYSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_PAYSZ_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    #[must_use]
    pub fn rxpaysz(&mut self) -> RXPAYSZ_W<0> {
        RXPAYSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Rx Paysize Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_paysz](index.html) module"]
pub struct RX_PAYSZ_SPEC;
impl crate::RegisterSpec for RX_PAYSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_paysz::R](R) reader structure"]
impl crate::Readable for RX_PAYSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_paysz::W](W) writer structure"]
impl crate::Writable for RX_PAYSZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_PAYSZ to value 0"]
impl crate::Resettable for RX_PAYSZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
