#[doc = "Register `MDMA_C24TCR` reader"]
pub struct R(crate::R<MDMA_C24TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C24TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C24TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C24TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C24TCR` writer"]
pub struct W(crate::W<MDMA_C24TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C24TCR_SPEC>;
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
impl From<crate::W<MDMA_C24TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C24TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINC` reader - SINC"]
pub type SINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINC` writer - SINC"]
pub type SINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DINC` reader - DINC"]
pub type DINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINC` writer - DINC"]
pub type DINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SSIZE` reader - SSIZE"]
pub type SSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSIZE` writer - SSIZE"]
pub type SSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DSIZE` reader - DSIZE"]
pub type DSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSIZE` writer - DSIZE"]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SINCOS` reader - SINCOS"]
pub type SINCOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINCOS` writer - SINCOS"]
pub type SINCOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DINCOS` reader - DINCOS"]
pub type DINCOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINCOS` writer - DINCOS"]
pub type DINCOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SBURST` reader - SBURST"]
pub type SBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBURST` writer - SBURST"]
pub type SBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBURST` reader - DBURST"]
pub type DBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBURST` writer - DBURST"]
pub type DBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TLEN` reader - TLEN"]
pub type TLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLEN` writer - TLEN"]
pub type TLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PKE` reader - PKE"]
pub type PKE_R = crate::BitReader<bool>;
#[doc = "Field `PKE` writer - PKE"]
pub type PKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C24TCR_SPEC, bool, O>;
#[doc = "Field `PAM` reader - PAM"]
pub type PAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAM` writer - PAM"]
pub type PAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRGM` reader - TRGM"]
pub type TRGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGM` writer - TRGM"]
pub type TRGM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C24TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SWRM` reader - SWRM"]
pub type SWRM_R = crate::BitReader<bool>;
#[doc = "Field `SWRM` writer - SWRM"]
pub type SWRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C24TCR_SPEC, bool, O>;
#[doc = "Field `BWM` reader - BWM"]
pub type BWM_R = crate::BitReader<bool>;
#[doc = "Field `BWM` writer - BWM"]
pub type BWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C24TCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    pub fn dincos(&self) -> DINCOS_R {
        DINCOS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    pub fn sburst(&self) -> SBURST_R {
        SBURST_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    pub fn dburst(&self) -> DBURST_R {
        DBURST_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    pub fn trgm(&self) -> TRGM_R {
        TRGM_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    pub fn swrm(&self) -> SWRM_R {
        SWRM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    pub fn bwm(&self) -> BWM_R {
        BWM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SINC_W<0> {
        SINC_W::new(self)
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DINC_W<2> {
        DINC_W::new(self)
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn ssize(&mut self) -> SSIZE_W<4> {
        SSIZE_W::new(self)
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<6> {
        DSIZE_W::new(self)
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    #[must_use]
    pub fn sincos(&mut self) -> SINCOS_W<8> {
        SINCOS_W::new(self)
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    #[must_use]
    pub fn dincos(&mut self) -> DINCOS_W<10> {
        DINCOS_W::new(self)
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    #[must_use]
    pub fn sburst(&mut self) -> SBURST_W<12> {
        SBURST_W::new(self)
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    #[must_use]
    pub fn dburst(&mut self) -> DBURST_W<15> {
        DBURST_W::new(self)
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<18> {
        TLEN_W::new(self)
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    #[must_use]
    pub fn pke(&mut self) -> PKE_W<25> {
        PKE_W::new(self)
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<26> {
        PAM_W::new(self)
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    #[must_use]
    pub fn trgm(&mut self) -> TRGM_W<28> {
        TRGM_W::new(self)
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    #[must_use]
    pub fn swrm(&mut self) -> SWRM_W<30> {
        SWRM_W::new(self)
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    #[must_use]
    pub fn bwm(&mut self) -> BWM_W<31> {
        BWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24tcr](index.html) module"]
pub struct MDMA_C24TCR_SPEC;
impl crate::RegisterSpec for MDMA_C24TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c24tcr::R](R) reader structure"]
impl crate::Readable for MDMA_C24TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c24tcr::W](W) writer structure"]
impl crate::Writable for MDMA_C24TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C24TCR to value 0"]
impl crate::Resettable for MDMA_C24TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
