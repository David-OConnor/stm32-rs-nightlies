#[doc = "Register `TCCR3` reader"]
pub struct R(crate::R<TCCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR3` writer"]
pub struct W(crate::W<TCCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR3_SPEC>;
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
impl From<crate::W<TCCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSWR_TOCNT` reader - HSWR_TOCNT"]
pub type HSWR_TOCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSWR_TOCNT` writer - HSWR_TOCNT"]
pub type HSWR_TOCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR3_SPEC, u16, u16, 16, O>;
#[doc = "Field `PM` reader - PM"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - PM"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCCR3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - HSWR_TOCNT"]
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - HSWR_TOCNT"]
    #[inline(always)]
    #[must_use]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W<0> {
        HSWR_TOCNT_W::new(self)
    }
    #[doc = "Bit 24 - PM"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<24> {
        PM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host timeout counter configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr3](index.html) module"]
pub struct TCCR3_SPEC;
impl crate::RegisterSpec for TCCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr3::R](R) reader structure"]
impl crate::Readable for TCCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr3::W](W) writer structure"]
impl crate::Writable for TCCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR3 to value 0"]
impl crate::Resettable for TCCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
