#[doc = "Register `SAI_PDMDLY` reader"]
pub struct R(crate::R<SAI_PDMDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_PDMDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_PDMDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_PDMDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_PDMDLY` writer"]
pub struct W(crate::W<SAI_PDMDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_PDMDLY_SPEC>;
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
impl From<crate::W<SAI_PDMDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_PDMDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYM1L` reader - DLYM1L"]
pub type DLYM1L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYM1L` writer - DLYM1L"]
pub type DLYM1L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_PDMDLY_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLYM1R` reader - DLYM1R"]
pub type DLYM1R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYM1R` writer - DLYM1R"]
pub type DLYM1R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_PDMDLY_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLYM2L` reader - DLYM2L"]
pub type DLYM2L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYM2L` writer - DLYM2L"]
pub type DLYM2L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_PDMDLY_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLYM2R` reader - DLYM2R"]
pub type DLYM2R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYM2R` writer - DLYM2R"]
pub type DLYM2R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_PDMDLY_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLYM3L` reader - DLYM3L"]
pub type DLYM3L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYM3L` writer - DLYM3L"]
pub type DLYM3L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_PDMDLY_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLYM3R` reader - DLYM3R"]
pub type DLYM3R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYM3R` writer - DLYM3R"]
pub type DLYM3R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_PDMDLY_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLYM4L` reader - DLYM4L"]
pub type DLYM4L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYM4L` writer - DLYM4L"]
pub type DLYM4L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_PDMDLY_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLYM4R` reader - DLYM4R"]
pub type DLYM4R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYM4R` writer - DLYM4R"]
pub type DLYM4R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_PDMDLY_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - DLYM1L"]
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYM1L_R {
        DLYM1L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - DLYM1R"]
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYM1R_R {
        DLYM1R_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DLYM2L"]
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYM2L_R {
        DLYM2L_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - DLYM2R"]
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYM2R_R {
        DLYM2R_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - DLYM3L"]
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYM3L_R {
        DLYM3L_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - DLYM3R"]
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYM3R_R {
        DLYM3R_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - DLYM4L"]
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYM4L_R {
        DLYM4L_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - DLYM4R"]
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYM4R_R {
        DLYM4R_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DLYM1L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1l(&mut self) -> DLYM1L_W<0> {
        DLYM1L_W::new(self)
    }
    #[doc = "Bits 4:6 - DLYM1R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1r(&mut self) -> DLYM1R_W<4> {
        DLYM1R_W::new(self)
    }
    #[doc = "Bits 8:10 - DLYM2L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2l(&mut self) -> DLYM2L_W<8> {
        DLYM2L_W::new(self)
    }
    #[doc = "Bits 12:14 - DLYM2R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2r(&mut self) -> DLYM2R_W<12> {
        DLYM2R_W::new(self)
    }
    #[doc = "Bits 16:18 - DLYM3L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3l(&mut self) -> DLYM3L_W<16> {
        DLYM3L_W::new(self)
    }
    #[doc = "Bits 20:22 - DLYM3R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3r(&mut self) -> DLYM3R_W<20> {
        DLYM3R_W::new(self)
    }
    #[doc = "Bits 24:26 - DLYM4L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4l(&mut self) -> DLYM4L_W<24> {
        DLYM4L_W::new(self)
    }
    #[doc = "Bits 28:30 - DLYM4R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4r(&mut self) -> DLYM4R_W<28> {
        DLYM4R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_pdmdly](index.html) module"]
pub struct SAI_PDMDLY_SPEC;
impl crate::RegisterSpec for SAI_PDMDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_pdmdly::R](R) reader structure"]
impl crate::Readable for SAI_PDMDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_pdmdly::W](W) writer structure"]
impl crate::Writable for SAI_PDMDLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_PDMDLY to value 0"]
impl crate::Resettable for SAI_PDMDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
