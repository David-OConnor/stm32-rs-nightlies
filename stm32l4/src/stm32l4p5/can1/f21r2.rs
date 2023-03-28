#[doc = "Register `F21R2` reader"]
pub struct R(crate::R<F21R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F21R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F21R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F21R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F21R2` writer"]
pub struct W(crate::W<F21R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F21R2_SPEC>;
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
impl From<crate::W<F21R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F21R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB` reader - Filter bits"]
pub type FB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FB` writer - Filter bits"]
pub type FB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F21R2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Filter bits"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<0> {
        FB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter bank 21 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f21r2](index.html) module"]
pub struct F21R2_SPEC;
impl crate::RegisterSpec for F21R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f21r2::R](R) reader structure"]
impl crate::Readable for F21R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f21r2::W](W) writer structure"]
impl crate::Writable for F21R2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F21R2 to value 0"]
impl crate::Resettable for F21R2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
