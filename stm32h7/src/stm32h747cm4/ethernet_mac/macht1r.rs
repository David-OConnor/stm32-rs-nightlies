#[doc = "Register `MACHT1R` reader"]
pub struct R(crate::R<MACHT1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHT1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHT1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHT1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACHT1R` writer"]
pub struct W(crate::W<MACHT1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACHT1R_SPEC>;
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
impl From<crate::W<MACHT1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACHT1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT63T32` reader - MAC Hash Table Second 32 Bits"]
pub type HT63T32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HT63T32` writer - MAC Hash Table Second 32 Bits"]
pub type HT63T32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACHT1R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MAC Hash Table Second 32 Bits"]
    #[inline(always)]
    pub fn ht63t32(&self) -> HT63T32_R {
        HT63T32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Hash Table Second 32 Bits"]
    #[inline(always)]
    #[must_use]
    pub fn ht63t32(&mut self) -> HT63T32_W<0> {
        HT63T32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Table 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macht1r](index.html) module"]
pub struct MACHT1R_SPEC;
impl crate::RegisterSpec for MACHT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macht1r::R](R) reader structure"]
impl crate::Readable for MACHT1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macht1r::W](W) writer structure"]
impl crate::Writable for MACHT1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACHT1R to value 0"]
impl crate::Resettable for MACHT1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
