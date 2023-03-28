#[doc = "Register `DMACCARxBR` reader"]
pub struct R(crate::R<DMACCARX_BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARX_BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARX_BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARX_BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACCARxBR` writer"]
pub struct W(crate::W<DMACCARX_BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCARX_BR_SPEC>;
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
impl From<crate::W<DMACCARX_BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCARX_BR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer"]
pub type CURRBUFAPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURRBUFAPTR` writer - Application Receive Buffer Address Pointer"]
pub type CURRBUFAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACCARX_BR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn currbufaptr(&mut self) -> CURRBUFAPTR_W<0> {
        CURRBUFAPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current application receive buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_br](index.html) module"]
pub struct DMACCARX_BR_SPEC;
impl crate::RegisterSpec for DMACCARX_BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccarx_br::R](R) reader structure"]
impl crate::Readable for DMACCARX_BR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaccarx_br::W](W) writer structure"]
impl crate::Writable for DMACCARX_BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACCARxBR to value 0"]
impl crate::Resettable for DMACCARX_BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
