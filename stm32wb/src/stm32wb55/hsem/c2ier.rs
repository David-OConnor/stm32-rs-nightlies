#[doc = "Register `C2IER` reader"]
pub struct R(crate::R<C2IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IER` writer"]
pub struct W(crate::W<C2IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IER_SPEC>;
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
impl From<crate::W<C2IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISEm` reader - CPU(2) semaphore m enable bit."]
pub type ISEM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISEm` writer - CPU(2) semaphore m enable bit."]
pub type ISEM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2IER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CPU(2) semaphore m enable bit."]
    #[inline(always)]
    pub fn isem(&self) -> ISEM_R {
        ISEM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(2) semaphore m enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn isem(&mut self) -> ISEM_W<0> {
        ISEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ier](index.html) module"]
pub struct C2IER_SPEC;
impl crate::RegisterSpec for C2IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ier::R](R) reader structure"]
impl crate::Readable for C2IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ier::W](W) writer structure"]
impl crate::Writable for C2IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2IER to value 0"]
impl crate::Resettable for C2IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
