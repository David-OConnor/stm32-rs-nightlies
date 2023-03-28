#[doc = "Register `BOOTR_PRG` reader"]
pub struct R(crate::R<BOOTR_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTR_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTR_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTR_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTR_PRG` writer"]
pub struct W(crate::W<BOOTR_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTR_PRG_SPEC>;
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
impl From<crate::W<BOOTR_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTR_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECBOOT_LOCK` reader - A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
pub type SECBOOT_LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECBOOT_LOCK` writer - A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
pub type SECBOOT_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOTR_PRG_SPEC, u8, u8, 8, O>;
#[doc = "Field `SECBOOTADD` reader - Secure unique boot entry address. These bits allow configuring the secure UBE address."]
pub type SECBOOTADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SECBOOTADD` writer - Secure unique boot entry address. These bits allow configuring the secure UBE address."]
pub type SECBOOTADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOTR_PRG_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
    #[inline(always)]
    pub fn secboot_lock(&self) -> SECBOOT_LOCK_R {
        SECBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Secure unique boot entry address. These bits allow configuring the secure UBE address."]
    #[inline(always)]
    pub fn secbootadd(&self) -> SECBOOTADD_R {
        SECBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
    #[inline(always)]
    #[must_use]
    pub fn secboot_lock(&mut self) -> SECBOOT_LOCK_W<0> {
        SECBOOT_LOCK_W::new(self)
    }
    #[doc = "Bits 8:31 - Secure unique boot entry address. These bits allow configuring the secure UBE address."]
    #[inline(always)]
    #[must_use]
    pub fn secbootadd(&mut self) -> SECBOOTADD_W<8> {
        SECBOOTADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH secure boot register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootr_prg](index.html) module"]
pub struct BOOTR_PRG_SPEC;
impl crate::RegisterSpec for BOOTR_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootr_prg::R](R) reader structure"]
impl crate::Readable for BOOTR_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootr_prg::W](W) writer structure"]
impl crate::Writable for BOOTR_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOTR_PRG to value 0"]
impl crate::Resettable for BOOTR_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
