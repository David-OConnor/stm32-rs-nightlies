#[doc = "Register `COUNT6_TX` reader"]
pub struct R(crate::R<COUNT6_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT6_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT6_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT6_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT6_TX` writer"]
pub struct W(crate::W<COUNT6_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT6_TX_SPEC>;
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
impl From<crate::W<COUNT6_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT6_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT6_TX` reader - Transmission byte count"]
pub type COUNT6_TX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT6_TX` writer - Transmission byte count"]
pub type COUNT6_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, COUNT6_TX_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    pub fn count6_tx(&self) -> COUNT6_TX_R {
        COUNT6_TX_R::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    #[must_use]
    pub fn count6_tx(&mut self) -> COUNT6_TX_W<0> {
        COUNT6_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count6_tx](index.html) module"]
pub struct COUNT6_TX_SPEC;
impl crate::RegisterSpec for COUNT6_TX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [count6_tx::R](R) reader structure"]
impl crate::Readable for COUNT6_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count6_tx::W](W) writer structure"]
impl crate::Writable for COUNT6_TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT6_TX to value 0"]
impl crate::Resettable for COUNT6_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
