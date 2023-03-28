#[doc = "Register `C1ICR` reader"]
pub struct R(crate::R<C1ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1ICR` writer"]
pub struct W(crate::W<C1ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1ICR_SPEC>;
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
impl From<crate::W<C1ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISCm` reader - CPU(n) semaphore m clear bit"]
pub type ISCM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISCm` writer - CPU(n) semaphore m clear bit"]
pub type ISCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C1ICR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CPU(n) semaphore m clear bit"]
    #[inline(always)]
    pub fn iscm(&self) -> ISCM_R {
        ISCM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(n) semaphore m clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn iscm(&mut self) -> ISCM_W<0> {
        ISCM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1icr](index.html) module"]
pub struct C1ICR_SPEC;
impl crate::RegisterSpec for C1ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1icr::R](R) reader structure"]
impl crate::Readable for C1ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1icr::W](W) writer structure"]
impl crate::Writable for C1ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1ICR to value 0"]
impl crate::Resettable for C1ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
