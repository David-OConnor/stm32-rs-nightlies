#[doc = "Register `DBG_AUTH_HOST` reader"]
pub struct R(crate::R<DBG_AUTH_HOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_AUTH_HOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_AUTH_HOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_AUTH_HOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_AUTH_HOST` writer"]
pub struct W(crate::W<DBG_AUTH_HOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_AUTH_HOST_SPEC>;
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
impl From<crate::W<DBG_AUTH_HOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_AUTH_HOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MESSAGE` reader - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
pub type MESSAGE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MESSAGE` writer - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
pub type MESSAGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_AUTH_HOST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
    #[inline(always)]
    #[must_use]
    pub fn message(&mut self) -> MESSAGE_W<0> {
        MESSAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU debug authentication mailbox host register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_auth_host](index.html) module"]
pub struct DBG_AUTH_HOST_SPEC;
impl crate::RegisterSpec for DBG_AUTH_HOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_auth_host::R](R) reader structure"]
impl crate::Readable for DBG_AUTH_HOST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_auth_host::W](W) writer structure"]
impl crate::Writable for DBG_AUTH_HOST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_AUTH_HOST to value 0"]
impl crate::Resettable for DBG_AUTH_HOST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
