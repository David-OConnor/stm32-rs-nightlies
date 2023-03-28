#[doc = "Register `DBG_AUTH_DEVICE` reader"]
pub struct R(crate::R<DBG_AUTH_DEVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_AUTH_DEVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_AUTH_DEVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_AUTH_DEVICE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MESSAGE` reader - Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register."]
pub type MESSAGE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register."]
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
#[doc = "DBGMCU debug authentication mailbox device register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_auth_device](index.html) module"]
pub struct DBG_AUTH_DEVICE_SPEC;
impl crate::RegisterSpec for DBG_AUTH_DEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_auth_device::R](R) reader structure"]
impl crate::Readable for DBG_AUTH_DEVICE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBG_AUTH_DEVICE to value 0"]
impl crate::Resettable for DBG_AUTH_DEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
