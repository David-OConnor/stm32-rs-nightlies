#[doc = "Register `FLTINR4` reader"]
pub struct R(crate::R<FLTINR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR4` writer"]
pub struct W(crate::W<FLTINR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR4_SPEC>;
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
impl From<crate::W<FLTINR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT5BLKE` reader - FLT5BLKE"]
pub type FLT5BLKE_R = crate::BitReader<bool>;
#[doc = "Field `FLT5BLKE` writer - FLT5BLKE"]
pub type FLT5BLKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR4_SPEC, bool, O>;
#[doc = "Field `FLT5BLKS` reader - FLT5BLKS"]
pub type FLT5BLKS_R = crate::BitReader<bool>;
#[doc = "Field `FLT5BLKS` writer - FLT5BLKS"]
pub type FLT5BLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR4_SPEC, bool, O>;
#[doc = "Field `FLT5CNT` reader - FLT5CNT"]
pub type FLT5CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT5CNT` writer - FLT5CNT"]
pub type FLT5CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR4_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLT5CRES` reader - FLT5CRES"]
pub type FLT5CRES_R = crate::BitReader<bool>;
#[doc = "Field `FLT5CRES` writer - FLT5CRES"]
pub type FLT5CRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR4_SPEC, bool, O>;
#[doc = "Field `FLT5RSTM` reader - FLT5RSTM"]
pub type FLT5RSTM_R = crate::BitReader<bool>;
#[doc = "Field `FLT5RSTM` writer - FLT5RSTM"]
pub type FLT5RSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR4_SPEC, bool, O>;
#[doc = "Field `FLT6BLKE` reader - FLT6BLKE"]
pub type FLT6BLKE_R = crate::BitReader<bool>;
#[doc = "Field `FLT6BLKE` writer - FLT6BLKE"]
pub type FLT6BLKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR4_SPEC, bool, O>;
#[doc = "Field `FLT6BLKS` reader - FLT6BLKS"]
pub type FLT6BLKS_R = crate::BitReader<bool>;
#[doc = "Field `FLT6BLKS` writer - FLT6BLKS"]
pub type FLT6BLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR4_SPEC, bool, O>;
#[doc = "Field `FLT6CNT` reader - FLT6CNT"]
pub type FLT6CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT6CNT` writer - FLT6CNT"]
pub type FLT6CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR4_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLT6CRES` reader - FLT6CRES"]
pub type FLT6CRES_R = crate::BitReader<bool>;
#[doc = "Field `FLT6CRES` writer - FLT6CRES"]
pub type FLT6CRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR4_SPEC, bool, O>;
#[doc = "Field `FLT6RSTM` reader - FLT6RSTM"]
pub type FLT6RSTM_R = crate::BitReader<bool>;
#[doc = "Field `FLT6RSTM` writer - FLT6RSTM"]
pub type FLT6RSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FLT5BLKE"]
    #[inline(always)]
    pub fn flt5blke(&self) -> FLT5BLKE_R {
        FLT5BLKE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT5BLKS"]
    #[inline(always)]
    pub fn flt5blks(&self) -> FLT5BLKS_R {
        FLT5BLKS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - FLT5CNT"]
    #[inline(always)]
    pub fn flt5cnt(&self) -> FLT5CNT_R {
        FLT5CNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - FLT5CRES"]
    #[inline(always)]
    pub fn flt5cres(&self) -> FLT5CRES_R {
        FLT5CRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLT5RSTM"]
    #[inline(always)]
    pub fn flt5rstm(&self) -> FLT5RSTM_R {
        FLT5RSTM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT6BLKE"]
    #[inline(always)]
    pub fn flt6blke(&self) -> FLT6BLKE_R {
        FLT6BLKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT6BLKS"]
    #[inline(always)]
    pub fn flt6blks(&self) -> FLT6BLKS_R {
        FLT6BLKS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - FLT6CNT"]
    #[inline(always)]
    pub fn flt6cnt(&self) -> FLT6CNT_R {
        FLT6CNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - FLT6CRES"]
    #[inline(always)]
    pub fn flt6cres(&self) -> FLT6CRES_R {
        FLT6CRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FLT6RSTM"]
    #[inline(always)]
    pub fn flt6rstm(&self) -> FLT6RSTM_R {
        FLT6RSTM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLT5BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt5blke(&mut self) -> FLT5BLKE_W<0> {
        FLT5BLKE_W::new(self)
    }
    #[doc = "Bit 1 - FLT5BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt5blks(&mut self) -> FLT5BLKS_W<1> {
        FLT5BLKS_W::new(self)
    }
    #[doc = "Bits 2:5 - FLT5CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt5cnt(&mut self) -> FLT5CNT_W<2> {
        FLT5CNT_W::new(self)
    }
    #[doc = "Bit 6 - FLT5CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt5cres(&mut self) -> FLT5CRES_W<6> {
        FLT5CRES_W::new(self)
    }
    #[doc = "Bit 7 - FLT5RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt5rstm(&mut self) -> FLT5RSTM_W<7> {
        FLT5RSTM_W::new(self)
    }
    #[doc = "Bit 8 - FLT6BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt6blke(&mut self) -> FLT6BLKE_W<8> {
        FLT6BLKE_W::new(self)
    }
    #[doc = "Bit 9 - FLT6BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt6blks(&mut self) -> FLT6BLKS_W<9> {
        FLT6BLKS_W::new(self)
    }
    #[doc = "Bits 10:13 - FLT6CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt6cnt(&mut self) -> FLT6CNT_W<10> {
        FLT6CNT_W::new(self)
    }
    #[doc = "Bit 14 - FLT6CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt6cres(&mut self) -> FLT6CRES_W<14> {
        FLT6CRES_W::new(self)
    }
    #[doc = "Bit 15 - FLT6RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt6rstm(&mut self) -> FLT6RSTM_W<15> {
        FLT6RSTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr4](index.html) module"]
pub struct FLTINR4_SPEC;
impl crate::RegisterSpec for FLTINR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr4::R](R) reader structure"]
impl crate::Readable for FLTINR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr4::W](W) writer structure"]
impl crate::Writable for FLTINR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTINR4 to value 0"]
impl crate::Resettable for FLTINR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
