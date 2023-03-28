#[doc = "Register `RX_ORDEXT1` reader"]
pub struct R(crate::R<RX_ORDEXT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ORDEXT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ORDEXT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ORDEXT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_ORDEXT1` writer"]
pub struct W(crate::W<RX_ORDEXT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_ORDEXT1_SPEC>;
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
impl From<crate::W<RX_ORDEXT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_ORDEXT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXSOPX1` reader - RXSOPX1"]
pub type RXSOPX1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RXSOPX1` writer - RXSOPX1"]
pub type RXSOPX1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_ORDEXT1_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - RXSOPX1"]
    #[inline(always)]
    pub fn rxsopx1(&self) -> RXSOPX1_R {
        RXSOPX1_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - RXSOPX1"]
    #[inline(always)]
    #[must_use]
    pub fn rxsopx1(&mut self) -> RXSOPX1_W<0> {
        RXSOPX1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Rx Ordered Set Extension Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ordext1](index.html) module"]
pub struct RX_ORDEXT1_SPEC;
impl crate::RegisterSpec for RX_ORDEXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ordext1::R](R) reader structure"]
impl crate::Readable for RX_ORDEXT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ordext1::W](W) writer structure"]
impl crate::Writable for RX_ORDEXT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_ORDEXT1 to value 0"]
impl crate::Resettable for RX_ORDEXT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
