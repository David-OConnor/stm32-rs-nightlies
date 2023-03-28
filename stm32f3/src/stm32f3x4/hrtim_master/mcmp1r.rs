#[doc = "Register `MCMP1R` reader"]
pub struct R(crate::R<MCMP1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCMP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCMP1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCMP1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCMP1R` writer"]
pub struct W(crate::W<MCMP1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMP1R_SPEC>;
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
impl From<crate::W<MCMP1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMP1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCMP1` reader - Master Timer Compare 1 value"]
pub type MCMP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MCMP1` writer - Master Timer Compare 1 value"]
pub type MCMP1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCMP1R_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 1 value"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1(&mut self) -> MCMP1_W<0> {
        MCMP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmp1r](index.html) module"]
pub struct MCMP1R_SPEC;
impl crate::RegisterSpec for MCMP1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcmp1r::R](R) reader structure"]
impl crate::Readable for MCMP1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcmp1r::W](W) writer structure"]
impl crate::Writable for MCMP1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCMP1R to value 0"]
impl crate::Resettable for MCMP1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
