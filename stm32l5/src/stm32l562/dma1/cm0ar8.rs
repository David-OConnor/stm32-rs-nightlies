#[doc = "Register `CM0AR8` reader"]
pub struct R(crate::R<CM0AR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0AR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0AR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0AR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0AR8` writer"]
pub struct W(crate::W<CM0AR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0AR8_SPEC>;
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
impl From<crate::W<CM0AR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0AR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA` reader - peripheral address"]
pub type PA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PA` writer - peripheral address"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0AR8_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<0> {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar8](index.html) module"]
pub struct CM0AR8_SPEC;
impl crate::RegisterSpec for CM0AR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0ar8::R](R) reader structure"]
impl crate::Readable for CM0AR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0ar8::W](W) writer structure"]
impl crate::Writable for CM0AR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0AR8 to value 0"]
impl crate::Resettable for CM0AR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
