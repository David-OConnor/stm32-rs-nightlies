#[doc = "Register `BGCMAR` reader"]
pub struct R(crate::R<BGCMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGCMAR` writer"]
pub struct W(crate::W<BGCMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCMAR_SPEC>;
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
impl From<crate::W<BGCMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - Memory address"]
pub type MA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MA` writer - Memory address"]
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGCMAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
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
#[doc = "background CLUT memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcmar](index.html) module"]
pub struct BGCMAR_SPEC;
impl crate::RegisterSpec for BGCMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgcmar::R](R) reader structure"]
impl crate::Readable for BGCMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgcmar::W](W) writer structure"]
impl crate::Writable for BGCMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BGCMAR to value 0"]
impl crate::Resettable for BGCMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
