#[doc = "Register `CREL` reader"]
pub struct R(crate::R<CREL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CREL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CREL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CREL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CREL` writer"]
pub struct W(crate::W<CREL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CREL_SPEC>;
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
impl From<crate::W<CREL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CREL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAY` reader - Time Stamp Day"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAY` writer - Time Stamp Day"]
pub type DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CREL_SPEC, u8, u8, 8, O>;
#[doc = "Field `MON` reader - Time Stamp Month"]
pub type MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON` writer - Time Stamp Month"]
pub type MON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CREL_SPEC, u8, u8, 8, O>;
#[doc = "Field `YEAR` reader - Time Stamp Year"]
pub type YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` writer - Time Stamp Year"]
pub type YEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CREL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release"]
pub type SUBSTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBSTEP` writer - Sub-step of Core Release"]
pub type SUBSTEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CREL_SPEC, u8, u8, 4, O>;
#[doc = "Field `STEP` reader - Step of Core Release"]
pub type STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEP` writer - Step of Core Release"]
pub type STEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CREL_SPEC, u8, u8, 4, O>;
#[doc = "Field `REL` reader - Core Release"]
pub type REL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REL` writer - Core Release"]
pub type REL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CREL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Month"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Stamp Day"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<0> {
        DAY_W::new(self)
    }
    #[doc = "Bits 8:15 - Time Stamp Month"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<8> {
        MON_W::new(self)
    }
    #[doc = "Bits 16:19 - Time Stamp Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<16> {
        YEAR_W::new(self)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn substep(&mut self) -> SUBSTEP_W<20> {
        SUBSTEP_W::new(self)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<24> {
        STEP_W::new(self)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn rel(&mut self) -> REL_W<28> {
        REL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Calibration Unit Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crel](index.html) module"]
pub struct CREL_SPEC;
impl crate::RegisterSpec for CREL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crel::R](R) reader structure"]
impl crate::Readable for CREL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crel::W](W) writer structure"]
impl crate::Writable for CREL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CREL to value 0"]
impl crate::Resettable for CREL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
