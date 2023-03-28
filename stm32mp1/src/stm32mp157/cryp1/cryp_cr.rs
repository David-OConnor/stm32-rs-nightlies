#[doc = "Register `CRYP_CR` reader"]
pub struct R(crate::R<CRYP_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_CR` writer"]
pub struct W(crate::W<CRYP_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_CR_SPEC>;
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
impl From<crate::W<CRYP_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALGODIR` reader - ALGODIR"]
pub type ALGODIR_R = crate::BitReader<bool>;
#[doc = "Field `ALGODIR` writer - ALGODIR"]
pub type ALGODIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_CR_SPEC, bool, O>;
#[doc = "Field `ALGOMODE` reader - ALGOMODE"]
pub type ALGOMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALGOMODE` writer - ALGOMODE"]
pub type ALGOMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYP_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DATATYPE` reader - DATATYPE"]
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATYPE` writer - DATATYPE"]
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYP_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `KEYSIZE` reader - KEYSIZE"]
pub type KEYSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEYSIZE` writer - KEYSIZE"]
pub type KEYSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYP_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FFLUSH` writer - FFLUSH"]
pub type FFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_CR_SPEC, bool, O>;
#[doc = "Field `CRYPEN` reader - CRYPEN"]
pub type CRYPEN_R = crate::BitReader<bool>;
#[doc = "Field `CRYPEN` writer - CRYPEN"]
pub type CRYPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_CR_SPEC, bool, O>;
#[doc = "Field `GCM_CCMPH` reader - GCM_CCMPH"]
pub type GCM_CCMPH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GCM_CCMPH` writer - GCM_CCMPH"]
pub type GCM_CCMPH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYP_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ALGOMODE3` reader - ALGOMODE3"]
pub type ALGOMODE3_R = crate::BitReader<bool>;
#[doc = "Field `ALGOMODE3` writer - ALGOMODE3"]
pub type ALGOMODE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_CR_SPEC, bool, O>;
#[doc = "Field `NPBLB` reader - NPBLB"]
pub type NPBLB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NPBLB` writer - NPBLB"]
pub type NPBLB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYP_CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 2 - ALGODIR"]
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - ALGOMODE"]
    #[inline(always)]
    pub fn algomode(&self) -> ALGOMODE_R {
        ALGOMODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DATATYPE"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - KEYSIZE"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - CRYPEN"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    pub fn gcm_ccmph(&self) -> GCM_CCMPH_R {
        GCM_CCMPH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - ALGOMODE3"]
    #[inline(always)]
    pub fn algomode3(&self) -> ALGOMODE3_R {
        ALGOMODE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - NPBLB"]
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ALGODIR"]
    #[inline(always)]
    #[must_use]
    pub fn algodir(&mut self) -> ALGODIR_W<2> {
        ALGODIR_W::new(self)
    }
    #[doc = "Bits 3:5 - ALGOMODE"]
    #[inline(always)]
    #[must_use]
    pub fn algomode(&mut self) -> ALGOMODE_W<3> {
        ALGOMODE_W::new(self)
    }
    #[doc = "Bits 6:7 - DATATYPE"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<6> {
        DATATYPE_W::new(self)
    }
    #[doc = "Bits 8:9 - KEYSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<8> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bit 14 - FFLUSH"]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<14> {
        FFLUSH_W::new(self)
    }
    #[doc = "Bit 15 - CRYPEN"]
    #[inline(always)]
    #[must_use]
    pub fn crypen(&mut self) -> CRYPEN_W<15> {
        CRYPEN_W::new(self)
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    #[must_use]
    pub fn gcm_ccmph(&mut self) -> GCM_CCMPH_W<16> {
        GCM_CCMPH_W::new(self)
    }
    #[doc = "Bit 19 - ALGOMODE3"]
    #[inline(always)]
    #[must_use]
    pub fn algomode3(&mut self) -> ALGOMODE3_W<19> {
        ALGOMODE3_W::new(self)
    }
    #[doc = "Bits 20:23 - NPBLB"]
    #[inline(always)]
    #[must_use]
    pub fn npblb(&mut self) -> NPBLB_W<20> {
        NPBLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRYP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_cr](index.html) module"]
pub struct CRYP_CR_SPEC;
impl crate::RegisterSpec for CRYP_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_cr::R](R) reader structure"]
impl crate::Readable for CRYP_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_cr::W](W) writer structure"]
impl crate::Writable for CRYP_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYP_CR to value 0"]
impl crate::Resettable for CRYP_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
