#[doc = "Register `CNTDR` reader"]
pub struct R(crate::R<CNTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTDR` writer"]
pub struct W(crate::W<CNTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTDR_SPEC>;
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
impl From<crate::W<CNTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTx` reader - Timerx Counter value"]
pub type CNTX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNTx` writer - Timerx Counter value"]
pub type CNTX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CNTDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cntx(&mut self) -> CNTX_W<0> {
        CNTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntdr](index.html) module"]
pub struct CNTDR_SPEC;
impl crate::RegisterSpec for CNTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntdr::R](R) reader structure"]
impl crate::Readable for CNTDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntdr::W](W) writer structure"]
impl crate::Writable for CNTDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTDR to value 0"]
impl crate::Resettable for CNTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
