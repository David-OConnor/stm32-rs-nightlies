#[doc = "Register `SECWM1R_PRG` reader"]
pub struct R(crate::R<SECWM1R_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM1R_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM1R_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM1R_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECWM1R_PRG` writer"]
pub struct W(crate::W<SECWM1R_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM1R_PRG_SPEC>;
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
impl From<crate::W<SECWM1R_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM1R_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECWM1_STRT` reader - Bank1 security WM area 1 start sector"]
pub type SECWM1_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM1_STRT` writer - Bank1 security WM area 1 start sector"]
pub type SECWM1_STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECWM1R_PRG_SPEC, u8, u8, 7, O>;
#[doc = "Field `SECWM1_END` reader - Bank1 security WM area 1 end sector"]
pub type SECWM1_END_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM1_END` writer - Bank1 security WM area 1 end sector"]
pub type SECWM1_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECWM1R_PRG_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Bank1 security WM area 1 start sector"]
    #[inline(always)]
    pub fn secwm1_strt(&self) -> SECWM1_STRT_R {
        SECWM1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank1 security WM area 1 end sector"]
    #[inline(always)]
    pub fn secwm1_end(&self) -> SECWM1_END_R {
        SECWM1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank1 security WM area 1 start sector"]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_strt(&mut self) -> SECWM1_STRT_W<0> {
        SECWM1_STRT_W::new(self)
    }
    #[doc = "Bits 16:22 - Bank1 security WM area 1 end sector"]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_end(&mut self) -> SECWM1_END_W<16> {
        SECWM1_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH security watermark for Bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm1r_prg](index.html) module"]
pub struct SECWM1R_PRG_SPEC;
impl crate::RegisterSpec for SECWM1R_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm1r_prg::R](R) reader structure"]
impl crate::Readable for SECWM1R_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secwm1r_prg::W](W) writer structure"]
impl crate::Writable for SECWM1R_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECWM1R_PRG to value 0"]
impl crate::Resettable for SECWM1R_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
