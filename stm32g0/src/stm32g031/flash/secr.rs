#[doc = "Register `SECR` reader"]
pub struct R(crate::R<SECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECR` writer"]
pub struct W(crate::W<SECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECR_SPEC>;
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
impl From<crate::W<SECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_SIZE` reader - Securable memory area size"]
pub type SEC_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC_SIZE` writer - Securable memory area size"]
pub type SEC_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECR_SPEC, u8, u8, 7, O>;
#[doc = "Field `BOOT_LOCK` reader - used to force boot from user area"]
pub type BOOT_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_LOCK` writer - used to force boot from user area"]
pub type BOOT_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Securable memory area size"]
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - used to force boot from user area"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Securable memory area size"]
    #[inline(always)]
    #[must_use]
    pub fn sec_size(&mut self) -> SEC_SIZE_W<0> {
        SEC_SIZE_W::new(self)
    }
    #[doc = "Bit 16 - used to force boot from user area"]
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<16> {
        BOOT_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Security register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secr](index.html) module"]
pub struct SECR_SPEC;
impl crate::RegisterSpec for SECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secr::R](R) reader structure"]
impl crate::Readable for SECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secr::W](W) writer structure"]
impl crate::Writable for SECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECR to value 0"]
impl crate::Resettable for SECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
