#[doc = "Register `EMR1` reader"]
pub struct R(crate::R<EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR1` writer"]
pub struct W(crate::W<EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR1_SPEC>;
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
impl From<crate::W<EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM` reader - CPU wakeup with event generation mask"]
pub type EM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EM` writer - CPU wakeup with event generation mask"]
pub type EM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `EM19` reader - EM19"]
pub type EM19_R = crate::BitReader<bool>;
#[doc = "Field `EM19` writer - EM19"]
pub type EM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR1_SPEC, bool, O>;
#[doc = "Field `EM23` reader - EM23"]
pub type EM23_R = crate::BitReader<bool>;
#[doc = "Field `EM23` writer - EM23"]
pub type EM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR1_SPEC, bool, O>;
#[doc = "Field `EM25` reader - EM25"]
pub type EM25_R = crate::BitReader<bool>;
#[doc = "Field `EM25` writer - EM25"]
pub type EM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR1_SPEC, bool, O>;
#[doc = "Field `EM31` reader - EM31"]
pub type EM31_R = crate::BitReader<bool>;
#[doc = "Field `EM31` writer - EM31"]
pub type EM31_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - CPU wakeup with event generation mask"]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 19 - EM19"]
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - EM23"]
    #[inline(always)]
    pub fn em23(&self) -> EM23_R {
        EM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - EM25"]
    #[inline(always)]
    pub fn em25(&self) -> EM25_R {
        EM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - EM31"]
    #[inline(always)]
    pub fn em31(&self) -> EM31_R {
        EM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU wakeup with event generation mask"]
    #[inline(always)]
    #[must_use]
    pub fn em(&mut self) -> EM_W<0> {
        EM_W::new(self)
    }
    #[doc = "Bit 19 - EM19"]
    #[inline(always)]
    #[must_use]
    pub fn em19(&mut self) -> EM19_W<19> {
        EM19_W::new(self)
    }
    #[doc = "Bit 23 - EM23"]
    #[inline(always)]
    #[must_use]
    pub fn em23(&mut self) -> EM23_W<23> {
        EM23_W::new(self)
    }
    #[doc = "Bit 25 - EM25"]
    #[inline(always)]
    #[must_use]
    pub fn em25(&mut self) -> EM25_W<25> {
        EM25_W::new(self)
    }
    #[doc = "Bit 31 - EM31"]
    #[inline(always)]
    #[must_use]
    pub fn em31(&mut self) -> EM31_W<31> {
        EM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr1](index.html) module"]
pub struct EMR1_SPEC;
impl crate::RegisterSpec for EMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr1::R](R) reader structure"]
impl crate::Readable for EMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr1::W](W) writer structure"]
impl crate::Writable for EMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR1 to value 0"]
impl crate::Resettable for EMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
