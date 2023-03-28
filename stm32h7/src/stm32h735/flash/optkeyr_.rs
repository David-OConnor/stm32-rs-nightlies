#[doc = "Register `OPTKEYR_` reader"]
pub struct R(crate::R<OPTKEYR__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTKEYR__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTKEYR__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTKEYR__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTKEYR_` writer"]
pub struct W(crate::W<OPTKEYR__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTKEYR__SPEC>;
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
impl From<crate::W<OPTKEYR__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTKEYR__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTKEYR` reader - Unlock key option bytes"]
pub type OPTKEYR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OPTKEYR` writer - Unlock key option bytes"]
pub type OPTKEYR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTKEYR__SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    pub fn optkeyr(&self) -> OPTKEYR_R {
        OPTKEYR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn optkeyr(&mut self) -> OPTKEYR_W<0> {
        OPTKEYR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkeyr_](index.html) module"]
pub struct OPTKEYR__SPEC;
impl crate::RegisterSpec for OPTKEYR__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optkeyr_::R](R) reader structure"]
impl crate::Readable for OPTKEYR__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optkeyr_::W](W) writer structure"]
impl crate::Writable for OPTKEYR__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTKEYR_ to value 0"]
impl crate::Resettable for OPTKEYR__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
