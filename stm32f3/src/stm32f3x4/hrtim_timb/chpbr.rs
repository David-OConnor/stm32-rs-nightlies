#[doc = "Register `CHPBR` reader"]
pub struct R(crate::R<CHPBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHPBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHPBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHPBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHPBR` writer"]
pub struct W(crate::W<CHPBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPBR_SPEC>;
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
impl From<crate::W<CHPBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARFRQ` reader - Timerx carrier frequency value"]
pub type CARFRQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CARFRQ` writer - Timerx carrier frequency value"]
pub type CARFRQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHPBR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CARDTY` reader - Timerx chopper duty cycle value"]
pub type CARDTY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CARDTY` writer - Timerx chopper duty cycle value"]
pub type CARDTY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHPBR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STRTPW` reader - STRTPW"]
pub type STRTPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STRTPW` writer - STRTPW"]
pub type STRTPW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHPBR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn carfrq(&self) -> CARFRQ_R {
        CARFRQ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn cardty(&self) -> CARDTY_R {
        CARDTY_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&self) -> STRTPW_R {
        STRTPW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    #[must_use]
    pub fn carfrq(&mut self) -> CARFRQ_W<0> {
        CARFRQ_W::new(self)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    #[must_use]
    pub fn cardty(&mut self) -> CARDTY_W<4> {
        CARDTY_W::new(self)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    #[must_use]
    pub fn strtpw(&mut self) -> STRTPW_W<7> {
        STRTPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpbr](index.html) module"]
pub struct CHPBR_SPEC;
impl crate::RegisterSpec for CHPBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chpbr::R](R) reader structure"]
impl crate::Readable for CHPBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chpbr::W](W) writer structure"]
impl crate::Writable for CHPBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHPBR to value 0"]
impl crate::Resettable for CHPBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
