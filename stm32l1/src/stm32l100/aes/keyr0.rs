#[doc = "Register `KEYR0` reader"]
pub struct R(crate::R<KEYR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR0` writer"]
pub struct W(crate::W<KEYR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR0_SPEC>;
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
impl From<crate::W<KEYR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYR0` reader - AES key"]
pub type KEYR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYR0` writer - AES key"]
pub type KEYR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    pub fn keyr0(&self) -> KEYR0_R {
        KEYR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    #[must_use]
    pub fn keyr0(&mut self) -> KEYR0_W<0> {
        KEYR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Key register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr0](index.html) module"]
pub struct KEYR0_SPEC;
impl crate::RegisterSpec for KEYR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr0::R](R) reader structure"]
impl crate::Readable for KEYR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr0::W](W) writer structure"]
impl crate::Writable for KEYR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYR0 to value 0"]
impl crate::Resettable for KEYR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
