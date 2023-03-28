#[doc = "Register `DBG_AUTH_ACK` reader"]
pub struct R(crate::R<DBG_AUTH_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_AUTH_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_AUTH_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_AUTH_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_AUTH_ACK` writer"]
pub struct W(crate::W<DBG_AUTH_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_AUTH_ACK_SPEC>;
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
impl From<crate::W<DBG_AUTH_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_AUTH_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_ACK` reader - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
pub type HOST_ACK_R = crate::BitReader<bool>;
#[doc = "Field `HOST_ACK` writer - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
pub type HOST_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_AUTH_ACK_SPEC, bool, O>;
#[doc = "Field `DEV_ACK` reader - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
pub type DEV_ACK_R = crate::BitReader<bool>;
#[doc = "Field `DEV_ACK` writer - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
pub type DEV_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_AUTH_ACK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
    #[inline(always)]
    pub fn host_ack(&self) -> HOST_ACK_R {
        HOST_ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
    #[inline(always)]
    pub fn dev_ack(&self) -> DEV_ACK_R {
        DEV_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
    #[inline(always)]
    #[must_use]
    pub fn host_ack(&mut self) -> HOST_ACK_W<0> {
        HOST_ACK_W::new(self)
    }
    #[doc = "Bit 1 - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
    #[inline(always)]
    #[must_use]
    pub fn dev_ack(&mut self) -> DEV_ACK_W<1> {
        DEV_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU debug authentication mailbox acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_auth_ack](index.html) module"]
pub struct DBG_AUTH_ACK_SPEC;
impl crate::RegisterSpec for DBG_AUTH_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_auth_ack::R](R) reader structure"]
impl crate::Readable for DBG_AUTH_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_auth_ack::W](W) writer structure"]
impl crate::Writable for DBG_AUTH_ACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_AUTH_ACK to value 0"]
impl crate::Resettable for DBG_AUTH_ACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
