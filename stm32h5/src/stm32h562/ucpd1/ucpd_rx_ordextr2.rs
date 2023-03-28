#[doc = "Register `UCPD_RX_ORDEXTR2` reader"]
pub struct R(crate::R<UCPD_RX_ORDEXTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCPD_RX_ORDEXTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCPD_RX_ORDEXTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCPD_RX_ORDEXTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCPD_RX_ORDEXTR2` writer"]
pub struct W(crate::W<UCPD_RX_ORDEXTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCPD_RX_ORDEXTR2_SPEC>;
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
impl From<crate::W<UCPD_RX_ORDEXTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCPD_RX_ORDEXTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXSOPX2` reader - Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
pub type RXSOPX2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RXSOPX2` writer - Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
pub type RXSOPX2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UCPD_RX_ORDEXTR2_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
    #[inline(always)]
    pub fn rxsopx2(&self) -> RXSOPX2_R {
        RXSOPX2_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
    #[inline(always)]
    #[must_use]
    pub fn rxsopx2(&mut self) -> RXSOPX2_W<0> {
        RXSOPX2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Rx ordered set extension register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucpd_rx_ordextr2](index.html) module"]
pub struct UCPD_RX_ORDEXTR2_SPEC;
impl crate::RegisterSpec for UCPD_RX_ORDEXTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ucpd_rx_ordextr2::R](R) reader structure"]
impl crate::Readable for UCPD_RX_ORDEXTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucpd_rx_ordextr2::W](W) writer structure"]
impl crate::Writable for UCPD_RX_ORDEXTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCPD_RX_ORDEXTR2 to value 0"]
impl crate::Resettable for UCPD_RX_ORDEXTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}