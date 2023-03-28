#[doc = "Register `AES_SUSP7R` reader"]
pub struct R(crate::R<AES_SUSP7R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_SUSP7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_SUSP7R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_SUSP7R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_SUSP7R` writer"]
pub struct W(crate::W<AES_SUSP7R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_SUSP7R_SPEC>;
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
impl From<crate::W<AES_SUSP7R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_SUSP7R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSP` reader - AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers."]
pub type SUSP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SUSP` writer - AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers."]
pub type SUSP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AES_SUSP7R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of one of internal AES registers."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<0> {
        SUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES suspend registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_susp7r](index.html) module"]
pub struct AES_SUSP7R_SPEC;
impl crate::RegisterSpec for AES_SUSP7R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_susp7r::R](R) reader structure"]
impl crate::Readable for AES_SUSP7R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_susp7r::W](W) writer structure"]
impl crate::Writable for AES_SUSP7R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_SUSP7R to value 0"]
impl crate::Resettable for AES_SUSP7R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
