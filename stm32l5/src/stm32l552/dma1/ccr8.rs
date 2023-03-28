#[doc = "Register `CCR8` reader"]
pub struct R(crate::R<CCR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR8` writer"]
pub struct W(crate::W<CCR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR8_SPEC>;
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
impl From<crate::W<CCR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - peripheral address"]
pub type MA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MA` writer - peripheral address"]
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR8_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<0> {
        MA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr8](index.html) module"]
pub struct CCR8_SPEC;
impl crate::RegisterSpec for CCR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr8::R](R) reader structure"]
impl crate::Readable for CCR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr8::W](W) writer structure"]
impl crate::Writable for CCR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR8 to value 0"]
impl crate::Resettable for CCR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
