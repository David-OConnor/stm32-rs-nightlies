#[doc = "Register `SMPR1` reader"]
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR1` writer"]
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
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
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP0` reader - Channel 0 sampling time selection"]
pub type SMP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP0` writer - Channel 0 sampling time selection"]
pub type SMP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP1` reader - Channel 1 sampling time selection"]
pub type SMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP1` writer - Channel 1 sampling time selection"]
pub type SMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP2` reader - Channel 2 sampling time selection"]
pub type SMP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP2` writer - Channel 2 sampling time selection"]
pub type SMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP3` reader - Channel 3 sampling time selection"]
pub type SMP3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP3` writer - Channel 3 sampling time selection"]
pub type SMP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP4` reader - Channel 4 sampling time selection"]
pub type SMP4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP4` writer - Channel 4 sampling time selection"]
pub type SMP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP5` reader - Channel 5 sampling time selection"]
pub type SMP5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP5` writer - Channel 5 sampling time selection"]
pub type SMP5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP6` reader - Channel 6 sampling time selection"]
pub type SMP6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP6` writer - Channel 6 sampling time selection"]
pub type SMP6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP7` reader - Channel 7 sampling time selection"]
pub type SMP7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP7` writer - Channel 7 sampling time selection"]
pub type SMP7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP8` reader - Channel 8 sampling time selection"]
pub type SMP8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP8` writer - Channel 8 sampling time selection"]
pub type SMP8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP9` reader - Channel 9 sampling time selection"]
pub type SMP9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP9` writer - Channel 9 sampling time selection"]
pub type SMP9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp0(&mut self) -> SMP0_W<0> {
        SMP0_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<3> {
        SMP1_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<6> {
        SMP2_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> SMP3_W<9> {
        SMP3_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> SMP4_W<12> {
        SMP4_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> SMP5_W<15> {
        SMP5_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> SMP6_W<18> {
        SMP6_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> SMP7_W<21> {
        SMP7_W::new(self)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> SMP8_W<24> {
        SMP8_W::new(self)
    }
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> SMP9_W<27> {
        SMP9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sampling time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](index.html) module"]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr1::R](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr1::W](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
