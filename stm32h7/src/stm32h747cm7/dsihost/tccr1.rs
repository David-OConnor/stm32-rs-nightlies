#[doc = "Register `TCCR1` reader"]
pub struct R(crate::R<TCCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1` writer"]
pub struct W(crate::W<TCCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1_SPEC>;
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
impl From<crate::W<TCCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSRD_TOCNT` reader - High-speed read timeout counter"]
pub type HSRD_TOCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSRD_TOCNT` writer - High-speed read timeout counter"]
pub type HSRD_TOCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - High-speed read timeout counter"]
    #[inline(always)]
    pub fn hsrd_tocnt(&self) -> HSRD_TOCNT_R {
        HSRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High-speed read timeout counter"]
    #[inline(always)]
    #[must_use]
    pub fn hsrd_tocnt(&mut self) -> HSRD_TOCNT_W<0> {
        HSRD_TOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host timeout counter configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1](index.html) module"]
pub struct TCCR1_SPEC;
impl crate::RegisterSpec for TCCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr1::R](R) reader structure"]
impl crate::Readable for TCCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1::W](W) writer structure"]
impl crate::Writable for TCCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1 to value 0"]
impl crate::Resettable for TCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
