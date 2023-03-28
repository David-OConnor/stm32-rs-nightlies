#[doc = "Register `PRIVBB1R1` reader"]
pub struct R(crate::R<PRIVBB1R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVBB1R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVBB1R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVBB1R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVBB1R1` writer"]
pub struct W(crate::W<PRIVBB1R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVBB1R1_SPEC>;
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
impl From<crate::W<PRIVBB1R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVBB1R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVBB1` reader - Privileged/unprivileged 8-Kbyte flash Bank 1 sector attribute"]
pub type PRIVBB1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRIVBB1` writer - Privileged/unprivileged 8-Kbyte flash Bank 1 sector attribute"]
pub type PRIVBB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRIVBB1R1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Privileged/unprivileged 8-Kbyte flash Bank 1 sector attribute"]
    #[inline(always)]
    pub fn privbb1(&self) -> PRIVBB1_R {
        PRIVBB1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Privileged/unprivileged 8-Kbyte flash Bank 1 sector attribute"]
    #[inline(always)]
    #[must_use]
    pub fn privbb1(&mut self) -> PRIVBB1_W<0> {
        PRIVBB1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH privilege block based register for Bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privbb1r1](index.html) module"]
pub struct PRIVBB1R1_SPEC;
impl crate::RegisterSpec for PRIVBB1R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privbb1r1::R](R) reader structure"]
impl crate::Readable for PRIVBB1R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privbb1r1::W](W) writer structure"]
impl crate::Writable for PRIVBB1R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIVBB1R1 to value 0"]
impl crate::Resettable for PRIVBB1R1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
