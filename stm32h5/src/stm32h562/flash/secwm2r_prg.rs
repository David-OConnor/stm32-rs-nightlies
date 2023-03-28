#[doc = "Register `SECWM2R_PRG` reader"]
pub struct R(crate::R<SECWM2R_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM2R_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM2R_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM2R_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECWM2R_PRG` writer"]
pub struct W(crate::W<SECWM2R_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM2R_PRG_SPEC>;
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
impl From<crate::W<SECWM2R_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM2R_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECWM_STRT2` reader - Bank 2 security WM area start sector"]
pub type SECWM_STRT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM_STRT2` writer - Bank 2 security WM area start sector"]
pub type SECWM_STRT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECWM2R_PRG_SPEC, u8, u8, 7, O>;
#[doc = "Field `SECWM_END2` reader - Bank 2 security WM area end sector"]
pub type SECWM_END2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM_END2` writer - Bank 2 security WM area end sector"]
pub type SECWM_END2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECWM2R_PRG_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Bank 2 security WM area start sector"]
    #[inline(always)]
    pub fn secwm_strt2(&self) -> SECWM_STRT2_R {
        SECWM_STRT2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 2 security WM area end sector"]
    #[inline(always)]
    pub fn secwm_end2(&self) -> SECWM_END2_R {
        SECWM_END2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 2 security WM area start sector"]
    #[inline(always)]
    #[must_use]
    pub fn secwm_strt2(&mut self) -> SECWM_STRT2_W<0> {
        SECWM_STRT2_W::new(self)
    }
    #[doc = "Bits 16:22 - Bank 2 security WM area end sector"]
    #[inline(always)]
    #[must_use]
    pub fn secwm_end2(&mut self) -> SECWM_END2_W<16> {
        SECWM_END2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH security watermark for Bank 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm2r_prg](index.html) module"]
pub struct SECWM2R_PRG_SPEC;
impl crate::RegisterSpec for SECWM2R_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm2r_prg::R](R) reader structure"]
impl crate::Readable for SECWM2R_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secwm2r_prg::W](W) writer structure"]
impl crate::Writable for SECWM2R_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECWM2R_PRG to value 0"]
impl crate::Resettable for SECWM2R_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
