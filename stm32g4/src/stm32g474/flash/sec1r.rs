#[doc = "Register `SEC1R` reader"]
pub struct R(crate::R<SEC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC1R` writer"]
pub struct W(crate::W<SEC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC1R_SPEC>;
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
impl From<crate::W<SEC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_SIZE1` reader - SEC_SIZE1"]
pub type SEC_SIZE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC_SIZE1` writer - SEC_SIZE1"]
pub type SEC_SIZE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEC1R_SPEC, u8, u8, 8, O>;
#[doc = "Field `BOOT_LOCK` reader - BOOT_LOCK"]
pub type BOOT_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_LOCK` writer - BOOT_LOCK"]
pub type BOOT_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC1R_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - SEC_SIZE1"]
    #[inline(always)]
    pub fn sec_size1(&self) -> SEC_SIZE1_R {
        SEC_SIZE1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - SEC_SIZE1"]
    #[inline(always)]
    #[must_use]
    pub fn sec_size1(&mut self) -> SEC_SIZE1_W<0> {
        SEC_SIZE1_W::new(self)
    }
    #[doc = "Bit 16 - BOOT_LOCK"]
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
#[doc = "securable area bank1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec1r](index.html) module"]
pub struct SEC1R_SPEC;
impl crate::RegisterSpec for SEC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec1r::R](R) reader structure"]
impl crate::Readable for SEC1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec1r::W](W) writer structure"]
impl crate::Writable for SEC1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC1R to value 0xff00_ff00"]
impl crate::Resettable for SEC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_ff00;
}
