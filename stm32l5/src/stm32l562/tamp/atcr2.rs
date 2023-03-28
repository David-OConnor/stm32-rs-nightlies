#[doc = "Register `ATCR2` reader"]
pub struct R(crate::R<ATCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCR2` writer"]
pub struct W(crate::W<ATCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCR2_SPEC>;
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
impl From<crate::W<ATCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATOSEL1` reader - ATOSEL1"]
pub type ATOSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL1` writer - ATOSEL1"]
pub type ATOSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATOSEL2` reader - ATOSEL2"]
pub type ATOSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL2` writer - ATOSEL2"]
pub type ATOSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATOSEL3` reader - ATOSEL3"]
pub type ATOSEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL3` writer - ATOSEL3"]
pub type ATOSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATOSEL4` reader - ATOSEL4"]
pub type ATOSEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL4` writer - ATOSEL4"]
pub type ATOSEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATOSEL5` reader - ATOSEL5"]
pub type ATOSEL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL5` writer - ATOSEL5"]
pub type ATOSEL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATOSEL6` reader - ATOSEL6"]
pub type ATOSEL6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL6` writer - ATOSEL6"]
pub type ATOSEL6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATOSEL7` reader - ATOSEL7"]
pub type ATOSEL7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL7` writer - ATOSEL7"]
pub type ATOSEL7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATOSEL8` reader - ATOSEL8"]
pub type ATOSEL8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL8` writer - ATOSEL8"]
pub type ATOSEL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 8:10 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - ATOSEL5"]
    #[inline(always)]
    pub fn atosel5(&self) -> ATOSEL5_R {
        ATOSEL5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - ATOSEL6"]
    #[inline(always)]
    pub fn atosel6(&self) -> ATOSEL6_R {
        ATOSEL6_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - ATOSEL7"]
    #[inline(always)]
    pub fn atosel7(&self) -> ATOSEL7_R {
        ATOSEL7_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - ATOSEL8"]
    #[inline(always)]
    pub fn atosel8(&self) -> ATOSEL8_R {
        ATOSEL8_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - ATOSEL1"]
    #[inline(always)]
    #[must_use]
    pub fn atosel1(&mut self) -> ATOSEL1_W<8> {
        ATOSEL1_W::new(self)
    }
    #[doc = "Bits 11:13 - ATOSEL2"]
    #[inline(always)]
    #[must_use]
    pub fn atosel2(&mut self) -> ATOSEL2_W<11> {
        ATOSEL2_W::new(self)
    }
    #[doc = "Bits 14:16 - ATOSEL3"]
    #[inline(always)]
    #[must_use]
    pub fn atosel3(&mut self) -> ATOSEL3_W<14> {
        ATOSEL3_W::new(self)
    }
    #[doc = "Bits 17:19 - ATOSEL4"]
    #[inline(always)]
    #[must_use]
    pub fn atosel4(&mut self) -> ATOSEL4_W<17> {
        ATOSEL4_W::new(self)
    }
    #[doc = "Bits 20:22 - ATOSEL5"]
    #[inline(always)]
    #[must_use]
    pub fn atosel5(&mut self) -> ATOSEL5_W<20> {
        ATOSEL5_W::new(self)
    }
    #[doc = "Bits 23:25 - ATOSEL6"]
    #[inline(always)]
    #[must_use]
    pub fn atosel6(&mut self) -> ATOSEL6_W<23> {
        ATOSEL6_W::new(self)
    }
    #[doc = "Bits 26:28 - ATOSEL7"]
    #[inline(always)]
    #[must_use]
    pub fn atosel7(&mut self) -> ATOSEL7_W<26> {
        ATOSEL7_W::new(self)
    }
    #[doc = "Bits 29:31 - ATOSEL8"]
    #[inline(always)]
    #[must_use]
    pub fn atosel8(&mut self) -> ATOSEL8_W<29> {
        ATOSEL8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP active tamper control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcr2](index.html) module"]
pub struct ATCR2_SPEC;
impl crate::RegisterSpec for ATCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atcr2::R](R) reader structure"]
impl crate::Readable for ATCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcr2::W](W) writer structure"]
impl crate::Writable for ATCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATCR2 to value 0"]
impl crate::Resettable for ATCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
