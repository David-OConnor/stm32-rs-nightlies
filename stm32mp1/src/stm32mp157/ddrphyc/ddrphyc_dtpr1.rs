#[doc = "Register `DDRPHYC_DTPR1` reader"]
pub struct R(crate::R<DDRPHYC_DTPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTPR1` writer"]
pub struct W(crate::W<DDRPHYC_DTPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTPR1_SPEC>;
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
impl From<crate::W<DDRPHYC_DTPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAOND` reader - TAOND"]
pub type TAOND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAOND` writer - TAOND"]
pub type TAOND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRTW` reader - TRTW"]
pub type TRTW_R = crate::BitReader<bool>;
#[doc = "Field `TRTW` writer - TRTW"]
pub type TRTW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DTPR1_SPEC, bool, O>;
#[doc = "Field `TFAW` reader - TFAW"]
pub type TFAW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFAW` writer - TFAW"]
pub type TFAW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `TMOD` reader - TMOD"]
pub type TMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMOD` writer - TMOD"]
pub type TMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRTODT` reader - TRTODT"]
pub type TRTODT_R = crate::BitReader<bool>;
#[doc = "Field `TRTODT` writer - TRTODT"]
pub type TRTODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DTPR1_SPEC, bool, O>;
#[doc = "Field `TRFC` reader - TRFC"]
pub type TRFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRFC` writer - TRFC"]
pub type TRFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `TDQSCKMIN` reader - TDQSCKMIN"]
pub type TDQSCKMIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDQSCKMIN` writer - TDQSCKMIN"]
pub type TDQSCKMIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DTPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `TDQSCKMAX` reader - TDQSCKMAX"]
pub type TDQSCKMAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDQSCKMAX` writer - TDQSCKMAX"]
pub type TDQSCKMAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DTPR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - TAOND"]
    #[inline(always)]
    pub fn taond(&self) -> TAOND_R {
        TAOND_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TRTW"]
    #[inline(always)]
    pub fn trtw(&self) -> TRTW_R {
        TRTW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - TFAW"]
    #[inline(always)]
    pub fn tfaw(&self) -> TFAW_R {
        TFAW_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:10 - TMOD"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - TRTODT"]
    #[inline(always)]
    pub fn trtodt(&self) -> TRTODT_R {
        TRTODT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - TRFC"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - TDQSCKMIN"]
    #[inline(always)]
    pub fn tdqsckmin(&self) -> TDQSCKMIN_R {
        TDQSCKMIN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - TDQSCKMAX"]
    #[inline(always)]
    pub fn tdqsckmax(&self) -> TDQSCKMAX_R {
        TDQSCKMAX_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TAOND"]
    #[inline(always)]
    #[must_use]
    pub fn taond(&mut self) -> TAOND_W<0> {
        TAOND_W::new(self)
    }
    #[doc = "Bit 2 - TRTW"]
    #[inline(always)]
    #[must_use]
    pub fn trtw(&mut self) -> TRTW_W<2> {
        TRTW_W::new(self)
    }
    #[doc = "Bits 3:8 - TFAW"]
    #[inline(always)]
    #[must_use]
    pub fn tfaw(&mut self) -> TFAW_W<3> {
        TFAW_W::new(self)
    }
    #[doc = "Bits 9:10 - TMOD"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TMOD_W<9> {
        TMOD_W::new(self)
    }
    #[doc = "Bit 11 - TRTODT"]
    #[inline(always)]
    #[must_use]
    pub fn trtodt(&mut self) -> TRTODT_W<11> {
        TRTODT_W::new(self)
    }
    #[doc = "Bits 16:23 - TRFC"]
    #[inline(always)]
    #[must_use]
    pub fn trfc(&mut self) -> TRFC_W<16> {
        TRFC_W::new(self)
    }
    #[doc = "Bits 24:26 - TDQSCKMIN"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsckmin(&mut self) -> TDQSCKMIN_W<24> {
        TDQSCKMIN_W::new(self)
    }
    #[doc = "Bits 27:29 - TDQSCKMAX"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsckmax(&mut self) -> TDQSCKMAX_W<27> {
        TDQSCKMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTP register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr1](index.html) module"]
pub struct DDRPHYC_DTPR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtpr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTPR1 to value 0x0a03_0090"]
impl crate::Resettable for DDRPHYC_DTPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a03_0090;
}
