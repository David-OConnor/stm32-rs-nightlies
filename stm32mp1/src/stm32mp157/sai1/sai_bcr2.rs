#[doc = "Register `SAI_BCR2` reader"]
pub struct R(crate::R<SAI_BCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_BCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_BCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_BCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_BCR2` writer"]
pub struct W(crate::W<SAI_BCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_BCR2_SPEC>;
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
impl From<crate::W<SAI_BCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_BCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTH` reader - FTH"]
pub type FTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTH` writer - FTH"]
pub type FTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_BCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FFLUSH` writer - FFLUSH"]
pub type FFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCR2_SPEC, bool, O>;
#[doc = "Field `TRIS` reader - TRIS"]
pub type TRIS_R = crate::BitReader<bool>;
#[doc = "Field `TRIS` writer - TRIS"]
pub type TRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCR2_SPEC, bool, O>;
#[doc = "Field `MUTE` reader - MUTE"]
pub type MUTE_R = crate::BitReader<bool>;
#[doc = "Field `MUTE` writer - MUTE"]
pub type MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCR2_SPEC, bool, O>;
#[doc = "Field `MUTEVAL` reader - MUTEVAL"]
pub type MUTEVAL_R = crate::BitReader<bool>;
#[doc = "Field `MUTEVAL` writer - MUTEVAL"]
pub type MUTEVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCR2_SPEC, bool, O>;
#[doc = "Field `MUTECNT` reader - MUTECNT"]
pub type MUTECNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUTECNT` writer - MUTECNT"]
pub type MUTECNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_BCR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `CPL` reader - CPL"]
pub type CPL_R = crate::BitReader<bool>;
#[doc = "Field `CPL` writer - CPL"]
pub type CPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCR2_SPEC, bool, O>;
#[doc = "Field `COMP` reader - COMP"]
pub type COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP` writer - COMP"]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_BCR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2 - FTH"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - TRIS"]
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MUTE"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MUTEVAL"]
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - MUTECNT"]
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - CPL"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FTH"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<0> {
        FTH_W::new(self)
    }
    #[doc = "Bit 3 - FFLUSH"]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<3> {
        FFLUSH_W::new(self)
    }
    #[doc = "Bit 4 - TRIS"]
    #[inline(always)]
    #[must_use]
    pub fn tris(&mut self) -> TRIS_W<4> {
        TRIS_W::new(self)
    }
    #[doc = "Bit 5 - MUTE"]
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<5> {
        MUTE_W::new(self)
    }
    #[doc = "Bit 6 - MUTEVAL"]
    #[inline(always)]
    #[must_use]
    pub fn muteval(&mut self) -> MUTEVAL_W<6> {
        MUTEVAL_W::new(self)
    }
    #[doc = "Bits 7:12 - MUTECNT"]
    #[inline(always)]
    #[must_use]
    pub fn mutecnt(&mut self) -> MUTECNT_W<7> {
        MUTECNT_W::new(self)
    }
    #[doc = "Bit 13 - CPL"]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<13> {
        CPL_W::new(self)
    }
    #[doc = "Bits 14:15 - COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<14> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bcr2](index.html) module"]
pub struct SAI_BCR2_SPEC;
impl crate::RegisterSpec for SAI_BCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_bcr2::R](R) reader structure"]
impl crate::Readable for SAI_BCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_bcr2::W](W) writer structure"]
impl crate::Writable for SAI_BCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_BCR2 to value 0"]
impl crate::Resettable for SAI_BCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
