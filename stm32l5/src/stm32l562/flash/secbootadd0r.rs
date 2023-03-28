#[doc = "Register `SECBOOTADD0R` reader"]
pub struct R(crate::R<SECBOOTADD0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECBOOTADD0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECBOOTADD0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECBOOTADD0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECBOOTADD0R` writer"]
pub struct W(crate::W<SECBOOTADD0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECBOOTADD0R_SPEC>;
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
impl From<crate::W<SECBOOTADD0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECBOOTADD0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_LOCK` reader - BOOT_LOCK"]
pub type BOOT_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_LOCK` writer - BOOT_LOCK"]
pub type BOOT_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECBOOTADD0R_SPEC, bool, O>;
#[doc = "Field `SECBOOTADD0` writer - SECBOOTADD0"]
pub type SECBOOTADD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECBOOTADD0R_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bit 0 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOOT_LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<0> {
        BOOT_LOCK_W::new(self)
    }
    #[doc = "Bits 7:31 - SECBOOTADD0"]
    #[inline(always)]
    #[must_use]
    pub fn secbootadd0(&mut self) -> SECBOOTADD0_W<7> {
        SECBOOTADD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FFlash secure boot address 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbootadd0r](index.html) module"]
pub struct SECBOOTADD0R_SPEC;
impl crate::RegisterSpec for SECBOOTADD0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secbootadd0r::R](R) reader structure"]
impl crate::Readable for SECBOOTADD0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secbootadd0r::W](W) writer structure"]
impl crate::Writable for SECBOOTADD0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECBOOTADD0R to value 0"]
impl crate::Resettable for SECBOOTADD0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
