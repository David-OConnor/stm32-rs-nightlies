#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP10` reader - Channel 10 sampling time selection"]
pub type SMP10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP10` writer - Channel 10 sampling time selection"]
pub type SMP10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP11` reader - Channel 11 sampling time selection"]
pub type SMP11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP11` writer - Channel 11 sampling time selection"]
pub type SMP11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP12` reader - Channel 12 sampling time selection"]
pub type SMP12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP12` writer - Channel 12 sampling time selection"]
pub type SMP12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP13` reader - Channel 13 sampling time selection"]
pub type SMP13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP13` writer - Channel 13 sampling time selection"]
pub type SMP13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP14` reader - Channel 14 sampling time selection"]
pub type SMP14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP14` writer - Channel 14 sampling time selection"]
pub type SMP14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP15` reader - Channel 15 sampling time selection"]
pub type SMP15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP15` writer - Channel 15 sampling time selection"]
pub type SMP15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP16` reader - Channel 16 sampling time selection"]
pub type SMP16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP16` writer - Channel 16 sampling time selection"]
pub type SMP16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP17` reader - Channel 17 sampling time selection"]
pub type SMP17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP17` writer - Channel 17 sampling time selection"]
pub type SMP17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP18` reader - Channel 18 sampling time selection"]
pub type SMP18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP18` writer - Channel 18 sampling time selection"]
pub type SMP18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<0> {
        SMP10_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 11 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<3> {
        SMP11_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 12 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<6> {
        SMP12_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<9> {
        SMP13_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<12> {
        SMP14_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<15> {
        SMP15_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<18> {
        SMP16_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<21> {
        SMP17_W::new(self)
    }
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<24> {
        SMP18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
