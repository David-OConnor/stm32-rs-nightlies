#[doc = "Register `FLTINR3` reader"]
pub struct R(crate::R<FLTINR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR3` writer"]
pub struct W(crate::W<FLTINR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR3_SPEC>;
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
impl From<crate::W<FLTINR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT1BLKE` reader - FLT1BLKE"]
pub type FLT1BLKE_R = crate::BitReader<bool>;
#[doc = "Field `FLT1BLKE` writer - FLT1BLKE"]
pub type FLT1BLKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT1BLKS` reader - FLT1BLKS"]
pub type FLT1BLKS_R = crate::BitReader<bool>;
#[doc = "Field `FLT1BLKS` writer - FLT1BLKS"]
pub type FLT1BLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT1CNT` reader - FLT1CNT"]
pub type FLT1CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT1CNT` writer - FLT1CNT"]
pub type FLT1CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLT1CRES` reader - FLT1CRES"]
pub type FLT1CRES_R = crate::BitReader<bool>;
#[doc = "Field `FLT1CRES` writer - FLT1CRES"]
pub type FLT1CRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT1RSTM` reader - FLT1RSTM"]
pub type FLT1RSTM_R = crate::BitReader<bool>;
#[doc = "Field `FLT1RSTM` writer - FLT1RSTM"]
pub type FLT1RSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT2BLKE` reader - FLT2BLKE"]
pub type FLT2BLKE_R = crate::BitReader<bool>;
#[doc = "Field `FLT2BLKE` writer - FLT2BLKE"]
pub type FLT2BLKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT2BLKS` reader - FLT2BLKS"]
pub type FLT2BLKS_R = crate::BitReader<bool>;
#[doc = "Field `FLT2BLKS` writer - FLT2BLKS"]
pub type FLT2BLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT2CNT` reader - FLT2CNT"]
pub type FLT2CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT2CNT` writer - FLT2CNT"]
pub type FLT2CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLT2CRES` reader - FLT2CRES"]
pub type FLT2CRES_R = crate::BitReader<bool>;
#[doc = "Field `FLT2CRES` writer - FLT2CRES"]
pub type FLT2CRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT2RSTM` reader - FLT2RSTM"]
pub type FLT2RSTM_R = crate::BitReader<bool>;
#[doc = "Field `FLT2RSTM` writer - FLT2RSTM"]
pub type FLT2RSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT3BLKE` reader - FLT3BLKE"]
pub type FLT3BLKE_R = crate::BitReader<bool>;
#[doc = "Field `FLT3BLKE` writer - FLT3BLKE"]
pub type FLT3BLKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT3BLKS` reader - FLT3BLKS"]
pub type FLT3BLKS_R = crate::BitReader<bool>;
#[doc = "Field `FLT3BLKS` writer - FLT3BLKS"]
pub type FLT3BLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT3CNT` reader - FLT3CNT"]
pub type FLT3CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT3CNT` writer - FLT3CNT"]
pub type FLT3CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLT3CRES` reader - FLT3CRES"]
pub type FLT3CRES_R = crate::BitReader<bool>;
#[doc = "Field `FLT3CRES` writer - FLT3CRES"]
pub type FLT3CRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT3RSTM` reader - FLT3RSTM"]
pub type FLT3RSTM_R = crate::BitReader<bool>;
#[doc = "Field `FLT3RSTM` writer - FLT3RSTM"]
pub type FLT3RSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT4BLKE` reader - FLT4BLKE"]
pub type FLT4BLKE_R = crate::BitReader<bool>;
#[doc = "Field `FLT4BLKE` writer - FLT4BLKE"]
pub type FLT4BLKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT4BLKS` reader - FLT4BLKS"]
pub type FLT4BLKS_R = crate::BitReader<bool>;
#[doc = "Field `FLT4BLKS` writer - FLT4BLKS"]
pub type FLT4BLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT4CNT` reader - FLT4CNT"]
pub type FLT4CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT4CNT` writer - FLT4CNT"]
pub type FLT4CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLT4CRES` reader - FLT4CRES"]
pub type FLT4CRES_R = crate::BitReader<bool>;
#[doc = "Field `FLT4CRES` writer - FLT4CRES"]
pub type FLT4CRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
#[doc = "Field `FLT4RSTM` reader - FLT4RSTM"]
pub type FLT4RSTM_R = crate::BitReader<bool>;
#[doc = "Field `FLT4RSTM` writer - FLT4RSTM"]
pub type FLT4RSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FLT1BLKE"]
    #[inline(always)]
    pub fn flt1blke(&self) -> FLT1BLKE_R {
        FLT1BLKE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT1BLKS"]
    #[inline(always)]
    pub fn flt1blks(&self) -> FLT1BLKS_R {
        FLT1BLKS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - FLT1CNT"]
    #[inline(always)]
    pub fn flt1cnt(&self) -> FLT1CNT_R {
        FLT1CNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - FLT1CRES"]
    #[inline(always)]
    pub fn flt1cres(&self) -> FLT1CRES_R {
        FLT1CRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLT1RSTM"]
    #[inline(always)]
    pub fn flt1rstm(&self) -> FLT1RSTM_R {
        FLT1RSTM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT2BLKE"]
    #[inline(always)]
    pub fn flt2blke(&self) -> FLT2BLKE_R {
        FLT2BLKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT2BLKS"]
    #[inline(always)]
    pub fn flt2blks(&self) -> FLT2BLKS_R {
        FLT2BLKS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - FLT2CNT"]
    #[inline(always)]
    pub fn flt2cnt(&self) -> FLT2CNT_R {
        FLT2CNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - FLT2CRES"]
    #[inline(always)]
    pub fn flt2cres(&self) -> FLT2CRES_R {
        FLT2CRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FLT2RSTM"]
    #[inline(always)]
    pub fn flt2rstm(&self) -> FLT2RSTM_R {
        FLT2RSTM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FLT3BLKE"]
    #[inline(always)]
    pub fn flt3blke(&self) -> FLT3BLKE_R {
        FLT3BLKE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLT3BLKS"]
    #[inline(always)]
    pub fn flt3blks(&self) -> FLT3BLKS_R {
        FLT3BLKS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - FLT3CNT"]
    #[inline(always)]
    pub fn flt3cnt(&self) -> FLT3CNT_R {
        FLT3CNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - FLT3CRES"]
    #[inline(always)]
    pub fn flt3cres(&self) -> FLT3CRES_R {
        FLT3CRES_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - FLT3RSTM"]
    #[inline(always)]
    pub fn flt3rstm(&self) -> FLT3RSTM_R {
        FLT3RSTM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FLT4BLKE"]
    #[inline(always)]
    pub fn flt4blke(&self) -> FLT4BLKE_R {
        FLT4BLKE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FLT4BLKS"]
    #[inline(always)]
    pub fn flt4blks(&self) -> FLT4BLKS_R {
        FLT4BLKS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - FLT4CNT"]
    #[inline(always)]
    pub fn flt4cnt(&self) -> FLT4CNT_R {
        FLT4CNT_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - FLT4CRES"]
    #[inline(always)]
    pub fn flt4cres(&self) -> FLT4CRES_R {
        FLT4CRES_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLT4RSTM"]
    #[inline(always)]
    pub fn flt4rstm(&self) -> FLT4RSTM_R {
        FLT4RSTM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLT1BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt1blke(&mut self) -> FLT1BLKE_W<0> {
        FLT1BLKE_W::new(self)
    }
    #[doc = "Bit 1 - FLT1BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt1blks(&mut self) -> FLT1BLKS_W<1> {
        FLT1BLKS_W::new(self)
    }
    #[doc = "Bits 2:5 - FLT1CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt1cnt(&mut self) -> FLT1CNT_W<2> {
        FLT1CNT_W::new(self)
    }
    #[doc = "Bit 6 - FLT1CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt1cres(&mut self) -> FLT1CRES_W<6> {
        FLT1CRES_W::new(self)
    }
    #[doc = "Bit 7 - FLT1RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt1rstm(&mut self) -> FLT1RSTM_W<7> {
        FLT1RSTM_W::new(self)
    }
    #[doc = "Bit 8 - FLT2BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt2blke(&mut self) -> FLT2BLKE_W<8> {
        FLT2BLKE_W::new(self)
    }
    #[doc = "Bit 9 - FLT2BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt2blks(&mut self) -> FLT2BLKS_W<9> {
        FLT2BLKS_W::new(self)
    }
    #[doc = "Bits 10:13 - FLT2CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt2cnt(&mut self) -> FLT2CNT_W<10> {
        FLT2CNT_W::new(self)
    }
    #[doc = "Bit 14 - FLT2CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt2cres(&mut self) -> FLT2CRES_W<14> {
        FLT2CRES_W::new(self)
    }
    #[doc = "Bit 15 - FLT2RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt2rstm(&mut self) -> FLT2RSTM_W<15> {
        FLT2RSTM_W::new(self)
    }
    #[doc = "Bit 16 - FLT3BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt3blke(&mut self) -> FLT3BLKE_W<16> {
        FLT3BLKE_W::new(self)
    }
    #[doc = "Bit 17 - FLT3BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt3blks(&mut self) -> FLT3BLKS_W<17> {
        FLT3BLKS_W::new(self)
    }
    #[doc = "Bits 18:21 - FLT3CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt3cnt(&mut self) -> FLT3CNT_W<18> {
        FLT3CNT_W::new(self)
    }
    #[doc = "Bit 22 - FLT3CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt3cres(&mut self) -> FLT3CRES_W<22> {
        FLT3CRES_W::new(self)
    }
    #[doc = "Bit 23 - FLT3RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt3rstm(&mut self) -> FLT3RSTM_W<23> {
        FLT3RSTM_W::new(self)
    }
    #[doc = "Bit 24 - FLT4BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt4blke(&mut self) -> FLT4BLKE_W<24> {
        FLT4BLKE_W::new(self)
    }
    #[doc = "Bit 25 - FLT4BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt4blks(&mut self) -> FLT4BLKS_W<25> {
        FLT4BLKS_W::new(self)
    }
    #[doc = "Bits 26:29 - FLT4CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt4cnt(&mut self) -> FLT4CNT_W<26> {
        FLT4CNT_W::new(self)
    }
    #[doc = "Bit 30 - FLT4CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt4cres(&mut self) -> FLT4CRES_W<30> {
        FLT4CRES_W::new(self)
    }
    #[doc = "Bit 31 - FLT4RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt4rstm(&mut self) -> FLT4RSTM_W<31> {
        FLT4RSTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr3](index.html) module"]
pub struct FLTINR3_SPEC;
impl crate::RegisterSpec for FLTINR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr3::R](R) reader structure"]
impl crate::Readable for FLTINR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr3::W](W) writer structure"]
impl crate::Writable for FLTINR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTINR3 to value 0"]
impl crate::Resettable for FLTINR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
