#[doc = "Register `DMACCATxBR` reader"]
pub struct R(crate::R<DMACCATX_BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCATX_BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCATX_BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCATX_BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACCATxBR` writer"]
pub struct W(crate::W<DMACCATX_BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCATX_BR_SPEC>;
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
impl From<crate::W<DMACCATX_BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCATX_BR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURTBUFAPTR` writer - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACCATX_BR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn curtbufaptr(&mut self) -> CURTBUFAPTR_W<0> {
        CURTBUFAPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current application transmit buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccatx_br](index.html) module"]
pub struct DMACCATX_BR_SPEC;
impl crate::RegisterSpec for DMACCATX_BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccatx_br::R](R) reader structure"]
impl crate::Readable for DMACCATX_BR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaccatx_br::W](W) writer structure"]
impl crate::Writable for DMACCATX_BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACCATxBR to value 0"]
impl crate::Resettable for DMACCATX_BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
