#[doc = "Register `SECWM1R1` reader"]
pub struct R(crate::R<SECWM1R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM1R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM1R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM1R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECWM1R1` writer"]
pub struct W(crate::W<SECWM1R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM1R1_SPEC>;
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
impl From<crate::W<SECWM1R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM1R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECWM1_PSTRT` reader - SECWM1_PSTRT"]
pub type SECWM1_PSTRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM1_PSTRT` writer - SECWM1_PSTRT"]
pub type SECWM1_PSTRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECWM1R1_SPEC, u8, u8, 7, O>;
#[doc = "Field `SECWM1_PEND` reader - SECWM1_PEND"]
pub type SECWM1_PEND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM1_PEND` writer - SECWM1_PEND"]
pub type SECWM1_PEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECWM1R1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - SECWM1_PSTRT"]
    #[inline(always)]
    pub fn secwm1_pstrt(&self) -> SECWM1_PSTRT_R {
        SECWM1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - SECWM1_PEND"]
    #[inline(always)]
    pub fn secwm1_pend(&self) -> SECWM1_PEND_R {
        SECWM1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SECWM1_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_pstrt(&mut self) -> SECWM1_PSTRT_W<0> {
        SECWM1_PSTRT_W::new(self)
    }
    #[doc = "Bits 16:22 - SECWM1_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_pend(&mut self) -> SECWM1_PEND_W<16> {
        SECWM1_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash bank 1 secure watermak1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm1r1](index.html) module"]
pub struct SECWM1R1_SPEC;
impl crate::RegisterSpec for SECWM1R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm1r1::R](R) reader structure"]
impl crate::Readable for SECWM1R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secwm1r1::W](W) writer structure"]
impl crate::Writable for SECWM1R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECWM1R1 to value 0xff00_ff00"]
impl crate::Resettable for SECWM1R1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_ff00;
}
