#[doc = "Register `DDRPHYC_DTDR1` reader"]
pub struct R(crate::R<DDRPHYC_DTDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTDR1` writer"]
pub struct W(crate::W<DDRPHYC_DTDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTDR1_SPEC>;
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
impl From<crate::W<DDRPHYC_DTDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTBYTE4` reader - DTBYTE4"]
pub type DTBYTE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBYTE4` writer - DTBYTE4"]
pub type DTBYTE4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTDR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTBYTE5` reader - DTBYTE5"]
pub type DTBYTE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBYTE5` writer - DTBYTE5"]
pub type DTBYTE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTDR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTBYTE6` reader - DTBYTE6"]
pub type DTBYTE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBYTE6` writer - DTBYTE6"]
pub type DTBYTE6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTDR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTBYTE7` reader - DTBYTE7"]
pub type DTBYTE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBYTE7` writer - DTBYTE7"]
pub type DTBYTE7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTDR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DTBYTE4"]
    #[inline(always)]
    pub fn dtbyte4(&self) -> DTBYTE4_R {
        DTBYTE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DTBYTE5"]
    #[inline(always)]
    pub fn dtbyte5(&self) -> DTBYTE5_R {
        DTBYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DTBYTE6"]
    #[inline(always)]
    pub fn dtbyte6(&self) -> DTBYTE6_R {
        DTBYTE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTBYTE7"]
    #[inline(always)]
    pub fn dtbyte7(&self) -> DTBYTE7_R {
        DTBYTE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTBYTE4"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte4(&mut self) -> DTBYTE4_W<0> {
        DTBYTE4_W::new(self)
    }
    #[doc = "Bits 8:15 - DTBYTE5"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte5(&mut self) -> DTBYTE5_W<8> {
        DTBYTE5_W::new(self)
    }
    #[doc = "Bits 16:23 - DTBYTE6"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte6(&mut self) -> DTBYTE6_W<16> {
        DTBYTE6_W::new(self)
    }
    #[doc = "Bits 24:31 - DTBYTE7"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte7(&mut self) -> DTBYTE7_W<24> {
        DTBYTE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTD register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtdr1](index.html) module"]
pub struct DDRPHYC_DTDR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtdr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtdr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTDR1 to value 0x7788_bb44"]
impl crate::Resettable for DDRPHYC_DTDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x7788_bb44;
}
