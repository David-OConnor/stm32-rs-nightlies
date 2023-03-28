#[doc = "Register `CHSELR1` reader"]
pub struct R(crate::R<CHSELR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR1` writer"]
pub struct W(crate::W<CHSELR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR1_SPEC>;
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
impl From<crate::W<CHSELR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ1` reader - SQ1"]
pub type SQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ1` writer - SQ1"]
pub type SQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ2` reader - SQ2"]
pub type SQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ2` writer - SQ2"]
pub type SQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ3` reader - SQ3"]
pub type SQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ3` writer - SQ3"]
pub type SQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ4` reader - SQ4"]
pub type SQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ4` writer - SQ4"]
pub type SQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ5` reader - SQ5"]
pub type SQ5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ5` writer - SQ5"]
pub type SQ5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ6` reader - SQ6"]
pub type SQ6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ6` writer - SQ6"]
pub type SQ6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ7` reader - SQ7"]
pub type SQ7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ7` writer - SQ7"]
pub type SQ7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ8` reader - SQ8"]
pub type SQ8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ8` writer - SQ8"]
pub type SQ8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - SQ1"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SQ2"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SQ3"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SQ4"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SQ5"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SQ6"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SQ7"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SQ8"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SQ1"]
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<0> {
        SQ1_W::new(self)
    }
    #[doc = "Bits 4:7 - SQ2"]
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<4> {
        SQ2_W::new(self)
    }
    #[doc = "Bits 8:11 - SQ3"]
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<8> {
        SQ3_W::new(self)
    }
    #[doc = "Bits 12:15 - SQ4"]
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<12> {
        SQ4_W::new(self)
    }
    #[doc = "Bits 16:19 - SQ5"]
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<16> {
        SQ5_W::new(self)
    }
    #[doc = "Bits 20:23 - SQ6"]
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<20> {
        SQ6_W::new(self)
    }
    #[doc = "Bits 24:27 - SQ7"]
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<24> {
        SQ7_W::new(self)
    }
    #[doc = "Bits 28:31 - SQ8"]
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<28> {
        SQ8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr1](index.html) module"]
pub struct CHSELR1_SPEC;
impl crate::RegisterSpec for CHSELR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr1::R](R) reader structure"]
impl crate::Readable for CHSELR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr1::W](W) writer structure"]
impl crate::Writable for CHSELR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHSELR1 to value 0"]
impl crate::Resettable for CHSELR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
