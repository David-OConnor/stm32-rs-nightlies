#[doc = "Register `HRA1` reader"]
pub struct R(crate::R<HRA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRA1` writer"]
pub struct W(crate::W<HRA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRA1_SPEC>;
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
impl From<crate::W<HRA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H1` reader - H1"]
pub type H1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `H1` writer - H1"]
pub type H1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HRA1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - H1"]
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - H1"]
    #[inline(always)]
    #[must_use]
    pub fn h1(&mut self) -> H1_W<0> {
        H1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hra1](index.html) module"]
pub struct HRA1_SPEC;
impl crate::RegisterSpec for HRA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hra1::R](R) reader structure"]
impl crate::Readable for HRA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hra1::W](W) writer structure"]
impl crate::Writable for HRA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRA1 to value 0"]
impl crate::Resettable for HRA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
