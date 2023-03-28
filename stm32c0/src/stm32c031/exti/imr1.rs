#[doc = "Register `IMR1` reader"]
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR1` writer"]
pub struct W(crate::W<IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR1_SPEC>;
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
impl From<crate::W<IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM` reader - CPU wakeup with interrupt mask"]
pub type IM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IM` writer - CPU wakeup with interrupt mask"]
pub type IM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `IM19` reader - IM19"]
pub type IM19_R = crate::BitReader<bool>;
#[doc = "Field `IM19` writer - IM19"]
pub type IM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `IM23` reader - IM23"]
pub type IM23_R = crate::BitReader<bool>;
#[doc = "Field `IM23` writer - IM23"]
pub type IM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `IM25` reader - IM25"]
pub type IM25_R = crate::BitReader<bool>;
#[doc = "Field `IM25` writer - IM25"]
pub type IM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `IM31` reader - IM31"]
pub type IM31_R = crate::BitReader<bool>;
#[doc = "Field `IM31` writer - IM31"]
pub type IM31_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - CPU wakeup with interrupt mask"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 19 - IM19"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - IM23"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - IM25"]
    #[inline(always)]
    pub fn im25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - IM31"]
    #[inline(always)]
    pub fn im31(&self) -> IM31_R {
        IM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU wakeup with interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> IM_W<0> {
        IM_W::new(self)
    }
    #[doc = "Bit 19 - IM19"]
    #[inline(always)]
    #[must_use]
    pub fn im19(&mut self) -> IM19_W<19> {
        IM19_W::new(self)
    }
    #[doc = "Bit 23 - IM23"]
    #[inline(always)]
    #[must_use]
    pub fn im23(&mut self) -> IM23_W<23> {
        IM23_W::new(self)
    }
    #[doc = "Bit 25 - IM25"]
    #[inline(always)]
    #[must_use]
    pub fn im25(&mut self) -> IM25_W<25> {
        IM25_W::new(self)
    }
    #[doc = "Bit 31 - IM31"]
    #[inline(always)]
    #[must_use]
    pub fn im31(&mut self) -> IM31_W<31> {
        IM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](index.html) module"]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr1::R](R) reader structure"]
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr1::W](W) writer structure"]
impl crate::Writable for IMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR1 to value 0xfff8_0000"]
impl crate::Resettable for IMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff8_0000;
}
