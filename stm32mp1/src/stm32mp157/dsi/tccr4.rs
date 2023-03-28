#[doc = "Register `TCCR4` reader"]
pub struct R(crate::R<TCCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4` writer"]
pub struct W(crate::W<TCCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4_SPEC>;
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
impl From<crate::W<TCCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPWR_TOCNT` reader - LPWR_TOCNT"]
pub type LPWR_TOCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LPWR_TOCNT` writer - LPWR_TOCNT"]
pub type LPWR_TOCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - LPWR_TOCNT"]
    #[inline(always)]
    pub fn lpwr_tocnt(&self) -> LPWR_TOCNT_R {
        LPWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPWR_TOCNT"]
    #[inline(always)]
    #[must_use]
    pub fn lpwr_tocnt(&mut self) -> LPWR_TOCNT_W<0> {
        LPWR_TOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host timeout counter configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4](index.html) module"]
pub struct TCCR4_SPEC;
impl crate::RegisterSpec for TCCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr4::R](R) reader structure"]
impl crate::Readable for TCCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4::W](W) writer structure"]
impl crate::Writable for TCCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4 to value 0"]
impl crate::Resettable for TCCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
