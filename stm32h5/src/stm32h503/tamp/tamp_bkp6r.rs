#[doc = "Register `TAMP_BKP6R` reader"]
pub struct R(crate::R<TAMP_BKP6R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_BKP6R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_BKP6R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_BKP6R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_BKP6R` writer"]
pub struct W(crate::W<TAMP_BKP6R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_BKP6R_SPEC>;
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
impl From<crate::W<TAMP_BKP6R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_BKP6R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKP` reader - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
pub type BKP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BKP` writer - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
pub type BKP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_BKP6R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<0> {
        BKP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP backup 6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp6r](index.html) module"]
pub struct TAMP_BKP6R_SPEC;
impl crate::RegisterSpec for TAMP_BKP6R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_bkp6r::R](R) reader structure"]
impl crate::Readable for TAMP_BKP6R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_bkp6r::W](W) writer structure"]
impl crate::Writable for TAMP_BKP6R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_BKP6R to value 0"]
impl crate::Resettable for TAMP_BKP6R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
