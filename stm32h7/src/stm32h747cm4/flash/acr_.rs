#[doc = "Register `ACR_` reader"]
pub struct R(crate::R<ACR__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR_` writer"]
pub struct W(crate::W<ACR__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR__SPEC>;
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
impl From<crate::W<ACR__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - Read latency"]
pub type LATENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LATENCY` writer - Read latency"]
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR__SPEC, u8, u8, 4, O>;
#[doc = "Field `WRHIGHFREQ` reader - Flash signal delay"]
pub type WRHIGHFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRHIGHFREQ` writer - Flash signal delay"]
pub type WRHIGHFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR__SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Read latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    pub fn wrhighfreq(&self) -> WRHIGHFREQ_R {
        WRHIGHFREQ_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    #[must_use]
    pub fn wrhighfreq(&mut self) -> WRHIGHFREQ_W<4> {
        WRHIGHFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr_](index.html) module"]
pub struct ACR__SPEC;
impl crate::RegisterSpec for ACR__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr_::R](R) reader structure"]
impl crate::Readable for ACR__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr_::W](W) writer structure"]
impl crate::Writable for ACR__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR_ to value 0x37"]
impl crate::Resettable for ACR__SPEC {
    const RESET_VALUE: Self::Ux = 0x37;
}
