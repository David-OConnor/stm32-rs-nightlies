#[doc = "Register `ECR` reader"]
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE` reader - Index Enable"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - Index Enable"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `IDIR` reader - Index Direction"]
pub type IDIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDIR` writer - Index Direction"]
pub type IDIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IBLK` reader - Index Blanking"]
pub type IBLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBLK` writer - Index Blanking"]
pub type IBLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FIDX` reader - First Index"]
pub type FIDX_R = crate::BitReader<bool>;
#[doc = "Field `FIDX` writer - First Index"]
pub type FIDX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `IPOS` reader - Index Positioning"]
pub type IPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPOS` writer - Index Positioning"]
pub type IPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PW` reader - Pulse width"]
pub type PW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PW` writer - Pulse width"]
pub type PW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PWPRSC` reader - Pulse Width prescaler"]
pub type PWPRSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWPRSC` writer - Pulse Width prescaler"]
pub type PWPRSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Index Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Index Direction"]
    #[inline(always)]
    pub fn idir(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Index Blanking"]
    #[inline(always)]
    pub fn iblk(&self) -> IBLK_R {
        IBLK_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - First Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Index Positioning"]
    #[inline(always)]
    pub fn ipos(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Pulse width"]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Pulse Width prescaler"]
    #[inline(always)]
    pub fn pwprsc(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Index Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<0> {
        IE_W::new(self)
    }
    #[doc = "Bits 1:2 - Index Direction"]
    #[inline(always)]
    #[must_use]
    pub fn idir(&mut self) -> IDIR_W<1> {
        IDIR_W::new(self)
    }
    #[doc = "Bits 3:4 - Index Blanking"]
    #[inline(always)]
    #[must_use]
    pub fn iblk(&mut self) -> IBLK_W<3> {
        IBLK_W::new(self)
    }
    #[doc = "Bit 5 - First Index"]
    #[inline(always)]
    #[must_use]
    pub fn fidx(&mut self) -> FIDX_W<5> {
        FIDX_W::new(self)
    }
    #[doc = "Bits 6:7 - Index Positioning"]
    #[inline(always)]
    #[must_use]
    pub fn ipos(&mut self) -> IPOS_W<6> {
        IPOS_W::new(self)
    }
    #[doc = "Bits 16:23 - Pulse width"]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<16> {
        PW_W::new(self)
    }
    #[doc = "Bits 24:26 - Pulse Width prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pwprsc(&mut self) -> PWPRSC_W<24> {
        PWPRSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecr::R](R) reader structure"]
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
