#[doc = "Register `TXBTO` reader"]
pub struct R(crate::R<TXBTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBTO` writer"]
pub struct W(crate::W<TXBTO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBTO_SPEC>;
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
impl From<crate::W<TXBTO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBTO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO` reader - Transmission Occurred."]
pub type TO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TO` writer - Transmission Occurred."]
pub type TO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBTO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmission Occurred."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission Occurred."]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<0> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx Buffer Transmission Occurred Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbto](index.html) module"]
pub struct TXBTO_SPEC;
impl crate::RegisterSpec for TXBTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbto::R](R) reader structure"]
impl crate::Readable for TXBTO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbto::W](W) writer structure"]
impl crate::Writable for TXBTO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TXBTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
